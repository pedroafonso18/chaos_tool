use std::process::Command;
use std::time::Duration;
use std::thread;
use regex::Regex;

pub fn vmfailure(remote_host: &str, seconds: i32) -> Result<(), String> {
    let host_re = Regex::new(r"^([a-zA-Z0-9\.\-]+|\d{1,3}(\.\d{1,3}){3})$").unwrap();
    if !host_re.is_match(remote_host) {
        return Err("Invalid remote host format".to_string());
    }

    let block_args = ["-I", "INPUT", "-j", "DROP"];
    let block_args_2 = ["-I", "OUTPUT", "-j", "DROP"];

    let unblock_args = ["-D", "INPUT", "-j", "DROP"];
    let unblock_args_2 = ["-D", "OUTPUT", "-j", "DROP"];

    println!("Blocking remote virtual machine...");
    let status_in = Command::new("ssh")
        .arg(remote_host)
        .arg("sudo")
        .arg("iptables")
        .args(&block_args)
        .status()
        .map_err(|e| format!("Failed to block INPUT: {}", e))?;
    if !status_in.success() {
        return Err("Block INPUT command failed".to_string());
    }

    let status_out = Command::new("ssh")
        .arg(remote_host)
        .arg("sudo")
        .arg("iptables")
        .args(&block_args_2)
        .status()
        .map_err(|e| format!("Failed to block OUTPUT: {}", e))?;
    if !status_out.success() {
        return Err("Block OUTPUT command failed".to_string());
    }

    thread::sleep(Duration::from_secs(seconds as u64));

    println!("Unblocking remote virtual machine...");
    let status_in = Command::new("ssh")
        .arg(remote_host)
        .arg("sudo")
        .arg("iptables")
        .args(&unblock_args)
        .status()
        .map_err(|e| format!("Failed to unblock INPUT: {}", e))?;
    if !status_in.success() {
        return Err("Unblock INPUT command failed".to_string());
    }

    let status_out = Command::new("ssh")
        .arg(remote_host)
        .arg("sudo")
        .arg("iptables")
        .args(&unblock_args_2)
        .status()
        .map_err(|e| format!("Failed to unblock OUTPUT: {}", e))?;
    if !status_out.success() {
        return Err("Unblock OUTPUT command failed".to_string());
    }

    Ok(())
}