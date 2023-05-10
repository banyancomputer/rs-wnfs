use crate::BlockStore;
use async_trait::async_trait;
use libipld::{Cid, IpldCodec};
use reqwest::{
    multipart::{Form, Part},
    Client, Response,
};
use std::{borrow::Cow, collections::HashMap, net::Ipv4Addr};

/// A disk-based blockstore that you can mutate.

pub struct ClientNetworkBlockStore {
    pub addr: String,
}

// -------------------------------------------------------------------------------------------------
// Implementations
// -------------------------------------------------------------------------------------------------

impl ClientNetworkBlockStore {
    // Initializes the NetworkBlockStore in client mode
    pub async fn new(ip: Ipv4Addr, port: u16) -> Self {
        let addr = format!("{}:{}", ip, port);
        println!("address being used is {}", addr);

        // Create/return the new instance of self
        Self { addr }
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
        let url: String = format!("http://{}/api/v0/block/put/{}", self.addr, cid);
        println!("c: the uri being requested is {}", url);

        let mut form_data = HashMap::new();
        form_data.insert("data", "rust");

        // Construct the Form data that will be sending content bytes over the network
        let form = Form::new().part("data", Part::bytes(bytes));

        // curl -X POST -F file=@myfile "http://127.0.0.1:5001/api/v0/block/put?cid-codec=raw&mhtype=sha2-256&mhlen=-1&pin=false&allow-big-block=false&format=<value>"
        let response: Response = Client::new()
            .post(url)
            .multipart(form)
            .send()
            .await
            .expect("Failed to send");

        // Grab the Bytes response
        let bytes: Vec<u8> = response.bytes().await?.to_vec();
        let plain = std::str::from_utf8(&bytes).ok();
        println!("c: response received: {:?}", plain);

        Ok(cid)
    }

    /// Retrieves an array of bytes from the block store with given CID.
    async fn get_block(&self, cid: &Cid) -> anyhow::Result<Cow<Vec<u8>>> {
        // The authority of our URL will be the hostname of the httpbin remote
        println!("client calling get_block");
        // Construct the appropriate URI for a block request
        let url: String = format!("http://{}/api/v0/block/get?arg={}", self.addr, cid);
        println!("c: the uri being requested is {}", url);
        println!("c: the cid being requested is {}", cid);

        // curl -X POST "http://127.0.0.1:5001/api/v0/block/get?arg=<cid>"
        let response: Response = Client::new()
            .post(url)
            // We need to manually add the host header because SendRequest does not
            // .header("Host", "example.com")
            .send()
            .await
            .expect("Failed to send get_block");

        // Grab the Bytes response
        let bytes: Vec<u8> = response.bytes().await?.to_vec();
        let plain = std::str::from_utf8(&bytes).ok();
        println!("c: response received: {:?}", plain);

        // Return Ok status with the bytes
        return Ok(Cow::Owned(bytes));
    }
}
