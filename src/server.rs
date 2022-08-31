    pub struct Server {
        port: String,
    }
    impl Server {
        pub fn run(port:String) -> Self {
            println!("cargo run");
            return Server {
                port
            };
        }

        pub fn start(self){
            println!("Server started on port {}", self.port);
        }
    }
