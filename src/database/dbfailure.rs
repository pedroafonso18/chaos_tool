use std::process::Command;
use std::time::Duration;
use std::thread;
use regex::Regex;

pub fn dbfailure(remote_host: &str, port: i32, seconds: i32) -> Result<(), String> {
    if port < 1 || port > 65535 {
        return Err("Port must be between 1 and 65535".to_string());
    }

    let host_re = Regex::new(r"^([a-zA-Z0-9\.\-]+|\d{1,3}(\.\d{1,3}){3})$").unwrap();
    if !host_re.is_match(remote_host) {
        return Err("Invalid remote host format".to_string());
    }

    let block_args = ["-A", "INPUT", "-p", "tcp", "--dport", &port.to_string(), "-j", "DROP"];
    let unblock_args = ["-D", "INPUT", "-p", "tcp", "--dport", &port.to_string(), "-j", "DROP"];

    println!("Blocking remote DB...");
    let status = Command::new("ssh")
        .arg(remote_host)
        .arg("sudo")
        .arg("iptables")
        .args(&block_args)
        .status()
        .map_err(|e| format!("Failed to block remote database: {}", e))?;
    if !status.success() {
        return Err("Block command failed".to_string());
    }

    thread::sleep(Duration::from_secs(seconds as u64));

    println!("Unblocking remote DB...");
    let status = Command::new("ssh")
        .arg(remote_host)
        .arg("sudo")
        .arg("iptables")
        .args(&unblock_args)
        .status()
        .map_err(|e| format!("Failed to unblock remote DB: {}", e))?;
    if !status.success() {
        return Err("Unblock command failed".to_string());
    }

    Ok(())
}