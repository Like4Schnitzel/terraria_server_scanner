use std::{future::Future, sync::{Arc, Mutex}, time::Duration};
use terraria_protocol::net::Terraria;
use std::net::Ipv4Addr;
use ipnetwork::Ipv4Network;
use tokio::spawn;

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
    const MAX_TASKS: u8 = 10;
    
    let range: Ipv4Network = Ipv4Network::new_checked(
        Ipv4Addr::new(192, 168, 0, 1),
        24,
    ).unwrap();

    let active_tasks = Arc::new(Mutex::new(0));
    for ip in range.iter() {
        while *active_tasks.lock().unwrap() >= MAX_TASKS as usize {
            // wait for a task to finish
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        *active_tasks.lock().unwrap() += 1;
        let active_tasks_clone = Arc::clone(&active_tasks);
        let _ = spawn(async move {
            let available = is_server_available(
                ip.to_string().as_str(),
                7777,
                Duration::from_secs(5)
            ).await;

            let mut output = ip.to_string();
            if available {
                output += " is reachable.";
            } else {
                output += " is not reachable.";
            }
            println!("{}", output);

            let mut active_tasks = active_tasks_clone.lock().unwrap();
            *active_tasks -= 1;
        });
    }
    while *active_tasks.lock().unwrap() > 0 {
        // wait for a task to finish
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}
