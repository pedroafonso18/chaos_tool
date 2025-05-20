use std::process::Command;

use rand::seq::IndexedRandom;


pub fn dockerkill(container_name: Option<String>, is_random: bool, prune: bool) -> Result<(), String> {
    if !is_random && !prune{
        if let Some(name) = container_name {
            println!("Stopping container {}", name);
            let status = Command::new("docker")
                .arg("stop")
                .arg(&name)
                .status()
                .map_err(|e| format!("Failed to stop container: {}", e))?;
            if !status.success() {
                return Err("Error stopping container".to_string());
            }

            println!("Now removing the container {}...", name);
            let removal = Command::new("docker")
                .arg("rm")
                .arg(&name)
                .status()
                .map_err(|e| format!("Failed to remove container: {}", e))?;
            if !removal.success() {
                return Err("Error removing container".to_string());
            }
        } else {
            return Err("Random flag not passed, but no container name passed as well.".to_string());
        }
    } else if prune{
        println!("Pruning all containers...");
        let output = Command::new("docker")
            .args(&["ps", "-aq"])
            .output()
            .map_err(|e| format!("Failed to list all containers: {}", e))?;
        let ids = String::from_utf8_lossy(&output.stdout);
        if !ids.trim().is_empty() {
            let status = Command::new("docker")
                .arg("rm")
                .arg("-f")
                .args(ids.split_whitespace())
                .status()
                .map_err(|e| format!("Failed to prune docker containers: {}", e))?;
            if !status.success() {
                return Err("Error pruning all containers".to_string());
            }
        } else {
            println!("No containers to prune.");
        }
    } else if is_random{
        println!("Removing random docker container...");
        let output = Command::new("docker")
        .args(&["ps", "--format", "{{.Names}}"])
        .output()
        .map_err(|e| format!("Failed to list containers: {}", e))?;

    if !output.status.success() {
        return Err("Failed to list running containers".to_string());
    }
    let containers = String::from_utf8_lossy(&output.stdout)
    .lines()
    .map(|s| s.to_string())
    .collect::<Vec<_>>();

    if containers.is_empty() {
        return Err("No running containers found".to_string());
    }

    let mut rng = rand::rng();
    let random_container = containers.choose(&mut rng).unwrap();

    println!("Deleting random_container: {}", random_container);
    let status = Command::new("docker")
        .arg("stop")
        .arg(&random_container)
        .status()
        .map_err(|e| format!("Failed to stop container: {}", e))?;
    if !status.success() {
        return Err("Error stopping container".to_string());
    }

    println!("Now removing the container {}...", random_container);
    let removal = Command::new("docker")
        .arg("rm")
        .arg(&random_container)
        .status()
        .map_err(|e| format!("Failed to remove container: {}", e))?;
    if !removal.success() {
        return Err("Error removing container".to_string());
    }


    }
    Ok(())
}