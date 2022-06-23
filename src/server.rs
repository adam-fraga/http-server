use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server { addr }
    }
    pub fn run(self: Self) {
        println!("Listening on {}", &self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            listener.accept();
        }
    }
}
