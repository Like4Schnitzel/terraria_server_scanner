use std::io::Read;
use std::net::TcpStream;
use terraria_protocol::net::{self, Terraria};

fn main() {    
    let terraria = match Terraria::connect("62.46.80.167:7777") {
        Ok(_) => {println!("Fortnite");},
        Err(_) => {println!("Brawl Stars")},
    };
}
