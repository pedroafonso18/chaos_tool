use std::process::Command;
use std::time::Duration;
use std::thread;

pub fn dbfailure(remote_host: &str, port: i32, seconds: i32) {
    let block_cmd = format!("sudo iptables -A INPUT -p tcp --dport {} -j DROP", port);
    let unblock_cmd = format!("sudo iptables -D INPUT -p tcp --dport {} -j DROP", port);
    println!("Blocking remote DB...");
    Command::new("ssh")
        .arg(remote_host)
        .arg(&block_cmd)
        .status()
        .expect("Failed to block remote database");

    thread::sleep(Duration::from_secs(seconds as u64));

    println!("Unblocking remote DB...");
    Command::new("ssh")
        .arg(remote_host)
        .arg(&unblock_cmd)
        .status()
        .expect("Failed to unblock remote DB");
}