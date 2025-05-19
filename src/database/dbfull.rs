use postgres::{Client, NoTls};
use std::thread;
use std::time::Duration;

pub fn dbfull(db_url: &str, users: u32, seconds: u32, remove_safety: bool) -> Result<(), String> {
    let safe_users = 40;
    if users > safe_users && !remove_safety {
        return Err(format!(
            "Safety is ON: Refusing to allocate more than {} users. Use --remove-safety to override.",
            safe_users
        ));
    }
    let mut client_vec: Vec<Client> = Vec::new();
    let mut failed = 0;
    for user in 0..users {
        match Client::connect(db_url, NoTls) {
            Ok(client) => {
                client_vec.push(client);
                println!("Opened connection {}", user + 1);
            }
            Err(e) => {
                eprintln!("Failed to open connection {}: {}", user + 1, e);
                failed += 1;
            }
        }
    }
    if client_vec.is_empty() {
        return Err("Failed to open any database connections.".to_string());
    }
    println!("Holding {} open connections for {} seconds...", client_vec.len(), seconds);
    thread::sleep(Duration::from_secs(seconds as u64));
    if failed > 0 {
        println!("{} connections failed to open.", failed);
    }
    Ok(())
}