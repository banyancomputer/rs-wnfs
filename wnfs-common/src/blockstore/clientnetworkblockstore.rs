use crate::{BlockStore, BlockStoreError};
use anyhow::Result;
use async_trait::async_trait;
use libipld::{Cid, IpldCodec};
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    io::{Write, IoSlice, Read},
    net::{Ipv4Addr, TcpStream},
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

/// A disk-based blockstore that you can mutate.

pub struct ClientNetworkBlockStore {
    pub stream: Arc<RwLock<TcpStream>>,
}

// -------------------------------------------------------------------------------------------------
// Implementations
// -------------------------------------------------------------------------------------------------

impl ClientNetworkBlockStore {
    // Initializes the NetworkBlockStore in client mode
    pub fn new(ip: Ipv4Addr, port: u16) -> Self {
        let addr = format!("{}:{}", ip.to_string(), port);
        println!("the address to connect to is {}", addr);
        let stream = TcpStream::connect(addr).expect("Couldn't connect to the server");
        let stream = Arc::new(RwLock::new(stream));

        Self { stream }
    }

    pub fn flush(&self) -> Result<()> {
        self.stream
            .write()
            .map_err(|_| BlockStoreError::LockPoisoned)?
            .flush()
            .map_err(|e| anyhow::Error::new(e))
    }
}

#[async_trait(?Send)]
impl BlockStore for ClientNetworkBlockStore {
    /// Stores an array of bytes in the block store.
    async fn put_block(&self, bytes: Vec<u8>, codec: IpldCodec) -> Result<Cid> {
        println!("client calling putblock");
        // Try to build the CID from the bytes and codec
        let cid = self.create_cid(&bytes, codec)?;

        println!("the cid being sent is {}", cid.to_string());

        let mut stream = self
            .stream
            .write()
            .map_err(|_| BlockStoreError::LockPoisoned)?;

        let cid_bytes = cid.to_bytes();
        let cid_len = vec![cid_bytes.len() as u8];

        // Conjoin all the data together
        let all_data = [&vec![0], &cid_len[..], &cid_bytes[..], &bytes[..]].concat();

        // Write all this data in sequence
        stream.write_all(&all_data)?;
        println!("bytes were sent to the network");
        let mut result_buf: [u8; 1] = [0];
        stream.read_exact(&mut result_buf)?;
        println!("{:?} received back from network write", result_buf);
        Ok(cid)
    }

    /// Retrieves an array of bytes from the block store with given CID.
    async fn get_block(&self, cid: &Cid) -> Result<Cow<Vec<u8>>> {
        println!("client calling getblock");

        let mut stream = self.stream.write().map_err(|_| BlockStoreError::LockPoisoned)?;

        let cid_bytes = cid.to_bytes();
        let cid_len = vec![cid_bytes.len() as u8];

        // Send a read request
        let all_data = [&vec![1], &cid_len[..], &cid_bytes[..]].concat();
        stream.write_all(&all_data)?;
        
        let mut data_bytes: Vec<u8> = Vec::new();
        // Get the response
        stream.read(&mut data_bytes)?;

        // Return Ok status with the bytes
        return Ok(Cow::Owned(data_bytes));
    }
}
