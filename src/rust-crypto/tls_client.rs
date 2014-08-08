extern crate time

use time;
use std::io::{IoResult, Steam, TcpStream};

struct TlsProtocolVersion {
    major: u8,
    minor: u8
}


struct TlsClientHello {
    version: TlsProtocolVersion,
    random: TlsRandom,
    session: Vec<u8>
}

impl TlsClientHello {
    pub fn new() -> TlsClientHello {
        TlsClientHello {
            
        }
    }
}

struct TlsRandom {
    time: u32,
    bytes: [u8, ..28]
}

impl TlsRandom {
    fn new() -> TlsRandom {
        let mut random = TlsRandom {
            time: time::now_utc().to_timespec().sec as u32,
            bytes: [0, ..28]
        };
        task_rng().fill_bytes(random.bytes);
        random
    }
}

struct TlsClient {
    stream: TcpStream
}

impl TlsClient {

    /*!
     * Wraps the TCP stream using TLS
     */
    pub fn wrap(stream: TcpStream) -> IoResult<> {
        let client = TlsClient {
            steam: stream
        }

        client.handshake(None);
        
        client
    }

    /*!
     * The TLS handshake
     *
     * Starts with a ClientHello
     * 
     */
    pub fn handshake(&mut self, session: Option<&[u8]>) {
        let hello = ClientHello {
            
        };
        
        self.write_client_hello(&hello);
    }

    pub fn write_client_hello(&mut self) {
    
    }
}

impl Stream for TlsClient {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<uint> {
        self.read(buf)   
    }   
    fn write(&mut self, buf: &mut [u8]) -> IoResult<uint> {
        self.write(buf)   
    }   
}
