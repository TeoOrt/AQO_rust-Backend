//creating a server class or struct
use crate::http::request;
use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::Read;
use std::net::TcpListener;
pub struct Server {
    addr: String,
}

impl Server {
    //constructor
    pub fn new(addr: String) -> Self {
        Self {
            // Self = Server
            addr, // same as addr: addr
                  //this will contain the ip  address
        }
    }
    pub fn run(self) {
        // self is this in c++
        //self will  take ownership of struct
        //and once ownership is  lost the funciton memory will  be dealloc
        println!("Listening on   {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();
        //while true
        loop {
            // the _ means that you can add as many of arguments for the match
            //the match is sort of a switch statement
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                }
                                Err(e) => println!("Failed to parse a request: {}", e),
                            }
                        }
                        Err(e) => println!("failed to read connection: {}", e),
                    }
                }
                Err(err) => println!("Failed to stablish connection: \n {}", err),
            }

            //listens for new connections
            //block calling thread until new connection
            /*
            let res = listener.accept();
            if res.is_err() {
                continue;
            }
            let (stream, addr) = res.unwrap(); */
        }
    }
}
