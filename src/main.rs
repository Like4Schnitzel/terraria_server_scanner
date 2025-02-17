use std::time::Duration;
use terraria_protocol::net::Terraria;
use std::net::Ipv4Addr;
use ipnetwork::Ipv4Network;
use tokio;

async fn is_server_available(ip: &str, port: u16, timeout: Duration) -> bool {
    let address = format!("{}:{}", ip, port);
    let _ = match Terraria::connect_timeout(&address, timeout) {
        Ok(_) => {
            return true;
        },
        Err(_) => {
            return false;
        },
    };
}

#[tokio::main]
async fn main() {
    let range: Ipv4Network = Ipv4Network::new_checked(
        Ipv4Addr::new(192, 168, 0, 1),
        24,
    ).unwrap();

    for ip in range.iter() {
        println!("{}", ip.to_string().as_str());
        println!("{}", is_server_available(ip.to_string().as_str(), 7777, Duration::from_secs(5)).await);
    }
}
