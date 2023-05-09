use crate::BlockStore;
use anyhow::{Ok, Result};
use async_trait::async_trait;
use libipld::{Cid, IpldCodec};
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    io::{Read, self, Write, IoSlice, IoSliceMut},
    net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream},
    path::{Path, PathBuf}, fs::File, thread,
};

/// A disk-based blockstore that you can mutate.
pub struct ServerNetworkBlockStore {}

// -------------------------------------------------------------------------------------------------
// Implementations
// -------------------------------------------------------------------------------------------------

impl ServerNetworkBlockStore {
    pub fn listen(port: u16) -> Result<()> {
        let ip = Ipv4Addr::new(127, 0, 0, 1);
        let socket = SocketAddrV4::new(ip, port);
        let listener = TcpListener::bind(socket).unwrap();

        thread::spawn(move || {
            for stream in listener.incoming() {
                if let Err(_e) = stream {
                    println!("error handling stream");
                }
                else {
                    thread::spawn(move || {
                        // connection succeeded
                        Self::handle_client(stream.unwrap()).unwrap();
                    });
                }
            }
        });

        Ok(())
    }

    pub fn handle_client(mut stream: TcpStream) -> Result<()> {    
        
        loop {
            let mut buf: Vec<u8> = Vec::new();
            let result = stream.read_to_end(&mut buf);
            
            if let Err(e) = result {
                println!("error parsing header: {:?}", e);
                return Err(anyhow::Error::new(e));
            }
            else {
                let len = result.unwrap();
                println!("received {} bytes", len);

                // If the first byte is 0 we are in write mode
                let write_mode =  *buf.get(0).unwrap() == 0;
                let cid_len = (*buf.get(1).unwrap()) as usize;
                let cid = Cid::try_from(&buf[2..2+cid_len])?;
                println!("server sees operation {} on cid {}", write_mode, cid);

                let dir_path = String::from("blockstore_example");
                let file_path = format!("{}/{}", dir_path, cid.to_string());
            
                if write_mode {
                    println!("server is writing!");
                    // The file in question
                    std::fs::create_dir_all(dir_path)?;
                    println!("server created the folder!");
                    let mut file = File::create(file_path)?;
                    println!("server created the file!");
                    let data = &buf[2+cid_len..];
                    println!("server extracted data");
                    file.write_all(data)?;
                    println!("server wrote: {:?}...\n", &buf[0..20]);
                    let mut response_data: Vec<u8> = vec![1];
                    // Write all the data back to the stream from the file
                    stream.write_all(&mut response_data)?;
                }
                else {
                    // The file in question
                    let mut file = File::open(file_path)?;

                    println!("file opened at cid location");

                    let mut data: Vec<u8> = Vec::new();
                    file.read_to_end(&mut data)?;
                    println!("file data read at cid location");
                    // Write all the data back to the stream from the file
                    stream.write_all(&mut data)?;
                    println!("server finished writing back to client");
                }
                
                return Ok(());
            }
        }
    }

}

