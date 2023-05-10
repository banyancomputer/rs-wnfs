use crate::{BlockStore, BlockStoreError};
use async_trait::async_trait;
use form_data::FormData;
use hyper::{body::HttpBody as _, Client, Uri, client::conn::SendRequest};
use libipld::{Cid, IpldCodec};
use std::{borrow::{Cow, Borrow}, net::Ipv4Addr, cell::{RefCell, Ref}};
use tokio::{
    io::{stdout, AsyncWriteExt as _},
    net::TcpStream,
};

use hyper::{
    client::conn,
    http::{Request, StatusCode, uri},
    Body,
};

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// A disk-based blockstore that you can mutate.

pub struct ClientNetworkBlockStore {
    pub addr: String,
    pub request_sender: RefCell<SendRequest<Body>>
}

// -------------------------------------------------------------------------------------------------
// Implementations
// -------------------------------------------------------------------------------------------------

impl ClientNetworkBlockStore {
    // Initializes the NetworkBlockStore in client mode
    pub async fn new(ip: Ipv4Addr, port: u16) -> Self {
        let addr = format!("{}:{}", ip.to_string(), port);
        println!("address being used is {}", addr);

        let target_stream = TcpStream::connect(&addr).await.unwrap();
        let (request_sender, connection) = conn::handshake(target_stream).await.unwrap();

         // spawn a task to poll the connection and drive the HTTP state
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Error in connection: {}", e);
            }
        });

        // Create/return the new instance of self
        Self { 
            addr,
            request_sender: RefCell::new(request_sender)
         }
    }

    pub async fn test(&self) -> Result<()> {
        // Still inside `async fn main`...
        let client = Client::new();
        println!("c: Client created");
        let uri = self.addr.parse()?;
        println!("c: uri parsed");
        // Await the response...
        let mut resp = client.get(uri).await?;
        println!("Response: {}", resp.status());
        // And now...
        while let Some(chunk) = resp.body_mut().data().await {
            println!("chunk: {:?}", &chunk?);
        }

        Ok(())
    }

    async fn send_request(&self, request: Request<Body>) -> Result<Vec<u8>> {
        println!("c: request built. sending...");
        let sender = self.request_sender.borrow_mut();

        let response = self.request_sender.borrow_mut().send_request(request).await?;
        println!("c: response received. interpreting...");
        // Grab the content from the body
        let response_content = response.into_body().data().await.unwrap()?.to_vec();
        println!("c: response interped: {:?}", response_content);
        let response_plain = std::str::from_utf8(&response_content).ok();
        println!("c: response strung: {:?}", response_plain);
        Ok(response_content)
    }
}

#[async_trait(?Send)]
impl BlockStore for ClientNetworkBlockStore {
    /// Stores an array of bytes in the block store.
    async fn put_block(&self, bytes: Vec<u8>, codec: IpldCodec) -> anyhow::Result<Cid> {
        println!("client calling put_block");
        // Try to build the CID from the bytes and codec
        let cid = self.create_cid(&bytes, codec)?;
        // Construct the appropriate URI for a block request
        let uri: Uri = format!("http://{}/api/v0/block/put/{}", self.addr, cid.to_string()).parse()?;
        println!("c: the uri being requested is {}", uri.to_string());

        // curl -X POST -F file=@myfile "http://127.0.0.1:5001/api/v0/block/put?cid-codec=raw&mhtype=sha2-256&mhlen=-1&pin=false&allow-big-block=false&format=<value>"
        let request = Request::builder()
            // We need to manually add the host header because SendRequest does not
            .header("Host", &self.addr)
            .uri(uri)
            .method("POST")
            
            .body(Body::from("data"))?;

        let x = FormData::new(&bytes, "data");
        let body = Body::default();

        let response = self.send_request(request).await.unwrap();

        Ok(cid)
    }

    /// Retrieves an array of bytes from the block store with given CID.
    async fn get_block(&self, cid: &Cid) -> anyhow::Result<Cow<Vec<u8>>> {
        // The authority of our URL will be the hostname of the httpbin remote
        println!("client calling get_block");
        // Construct the appropriate URI for a block request
        let uri: Uri = format!("{}/api/v0/block/get/", self.addr).parse()?;
        println!("c: the uri being requested is {}", uri.to_string());

        // curl -X POST "http://127.0.0.1:5001/api/v0/block/get?arg=<cid>"
        let request = Request::builder()
            // We need to manually add the host header because SendRequest does not
            .header("Host", "example.com")
            .header("arg", cid.to_string())
            .method("POST")
            .body(Body::from(""))?;

        let response = self.send_request(request).await.unwrap();

        // Return Ok status with the bytes
        return Ok(Cow::Owned(response));
    }
}
