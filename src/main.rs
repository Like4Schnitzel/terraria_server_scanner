use std::{sync::{Arc, Mutex}, time::Duration};
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

async fn scan_range(range: Ipv4Network, max_tasks: u8, print: bool) -> Vec<Ipv4Addr> {
    let result = Arc::new(Mutex::new(Vec::new()));

    let active_tasks = Arc::new(Mutex::new(0));
    for ip in range.iter() {
        while *active_tasks.lock().unwrap() >= max_tasks as usize {
            // wait for a task to finish
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        *active_tasks.lock().unwrap() += 1;
        let active_tasks_clone = Arc::clone(&active_tasks);
        let result_clone = Arc::clone(&result);
        let _ = spawn(async move {
            let available = is_server_available(
                ip.to_string().as_str(),
                7777,
                Duration::from_secs(5)
            ).await;

            if available {
                result_clone.lock().unwrap().push(ip);
            }

            if print {
                let mut output = ip.to_string();
                if available {
                    output += " is reachable.";
                } else {
                    output += " is not reachable.";
                }
                println!("{}", output);
            }

            *active_tasks_clone.lock().unwrap() -= 1;
        });
    }

    while *active_tasks.lock().unwrap() > 0 {
        // wait for a task to finish
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    return result.lock().unwrap().clone();
}

#[tokio::main]
async fn main() {
    const MAX_TASKS: u8 = 10;

    let range: Ipv4Network = Ipv4Network::new_checked(
        Ipv4Addr::new(149, 255, 151, 17),
        24,
    ).unwrap();

    let found_servers = scan_range(range, MAX_TASKS, false).await;

    println!("Found servers: {:?}", found_servers);
}
