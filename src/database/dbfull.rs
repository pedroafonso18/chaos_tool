use postgres::{Client, NoTls};
use std::thread;
use std::time::Duration;

pub fn dbfull(db_url: &str, users: u32, seconds: u32, remove_safety: bool) {
    let safe_users = 40;
    if users > safe_users && !remove_safety {
        println!("Safety is ON: Refusing to allocate more than {} users. Use --remove-safety to override.", safe_users);
        return;
    }
    let mut client_vec: Vec<Client> = Vec::new();
    for user in 0..users {
        match Client::connect(db_url, NoTls) {
            Ok(client) => {
                client_vec.push(client);
                println!("Opened connection {}", user + 1);
            }
            Err(e) => {
                eprintln!("Failed to open connection {}: {}", user + 1, e);
            }
        }
    }
    println!("Holding {} open connections for {} seconds...", client_vec.len(), seconds);
    thread::sleep(Duration::from_secs(seconds as u64));
}