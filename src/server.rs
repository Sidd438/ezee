use std::net::TcpListener;
use std::io::Read;
use crate::http::Request;
use std::convert::TryFrom;

pub struct Server {
    addr: String,
}
impl Server {
    pub fn run(port:String) -> Self {
        println!("cargo run");
        return Server {
            addr:port
        };
    }
    pub fn start(self){
        println!("Server started on address {}", self.addr);

        let listner = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listner.accept(){
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Recieved a request: {}", String::from_utf8_lossy(&buffer));
                            Request::try_from(&buffer[..]);
                        }
                        Err(e) => println!("Failed to Establish connection: {}", e) 
                    }
                }
                Err(e) => println!("Failed to Establish connection: {}", e)
            }
        }
    }
}
