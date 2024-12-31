use gdbstub::conn::{Connection, ConnectionExt};
use std::{
    io::Read, net::{Shutdown, TcpListener, TcpStream}, str
};

pub struct TcpConnection {
    stream: TcpStream,
}

impl TcpConnection {
    pub fn new_localhost(port: u16) -> Result<TcpConnection, &'static str> {
        let listener = TcpListener::bind(("127.0.0.1", port)).unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            stream.set_read_timeout(Some(std::time::Duration::from_millis(1)))
                .expect("set_read_timeout call failed");
            // stream.set_nonblocking(true).expect("set_nonblocking call failed");
            return Ok(TcpConnection { stream });
        };
        
        Err("could not accept socket connection")
    }
}

impl Drop for TcpConnection {
    fn drop(&mut self) {
        self.stream.shutdown(Shutdown::Both).expect("shutdown failed");
    }
}

impl Connection for TcpConnection {
    type Error = &'static str;

    fn write(&mut self, b: u8) -> Result<(), &'static str> {
        match self.stream.write(b) {
            Ok(_) => Ok(()),
            Err(_) => Err("socket write failed")
        }
    }

    fn flush(&mut self) -> Result<(), &'static str> {
        match self.stream.flush() {
            Ok(_) => Ok(()),
            Err(_) => Err("socket flush failed")
        }
    }
}

impl ConnectionExt for TcpConnection {

    fn read(&mut self) -> std::result::Result<u8, Self::Error> {
        let mut buf: [u8; 1] = [0];
        loop {
            match self.stream.read_exact(&mut buf)
            {
                Ok(_) => break,
                Err(e) => match e.kind() {
                    #[cfg(windows)]
                    std::io::ErrorKind::TimedOut => continue,
                    #[cfg(unix)]
                    std::io::ErrorKind::WouldBlock => continue,
                    _ => return Err("socket read failed")
                }
            }
        }
        Ok(buf[0])
    }

    fn peek(&mut self) -> std::result::Result<Option<u8>, Self::Error> {
        let mut buf: [u8; 1] = [0];
        loop {
            match self.stream.peek(&mut buf)
            {
                Ok(_) => break,
                Err(e) => match e.kind() {
                    #[cfg(windows)]
                    std::io::ErrorKind::TimedOut => return Ok(None),
                    #[cfg(unix)]
                    std::io::ErrorKind::WouldBlock => return Ok(None),
                    _ => {
                        println!("peek error: {:?}", e);
                        return Err("socket peek failed")
                    }
                }
            }
        }
        Ok(Some(buf[0]))
    }
}