use std::time::Duration;
use terraria_protocol::net::Terraria;
use iprange::IpRange;

fn is_server_available(ip: &str, port: u16, timeout: Duration) -> bool {
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

fn main() {
    println!("{}", is_server_available("62.46.80.167", 7777, Duration::from_secs(5)));
}
