# chaostool

This is a chaos tool created by and for Pedro Afonso, tailored for my specific use cases.

## Current Features
- **CPU Hogger:** Simulate high CPU usage by allocating multiple cores for a specified duration.
- **RAM Hogger:** Simulate high memory usage by allocating a specified amount of RAM for a specified duration.
- **Database Connection Hogger:** Open and hold a large number of database connections for a specified duration to simulate max user or resource exhaustion scenarios.
- **Database Failure Simulation:** Temporarily block access to a remote database by manipulating firewall rules over SSH.

## Planned Features
- **VM Shutdown Simulation:** Simulate virtual machine shutdown events for resilience testing.

---

## Installation

1. **Clone the repository:**
   ```sh
   git clone <your-repo-url>
   cd chaostool
   ```
2. **Build the project (requires Rust and Cargo):**
   ```sh
   cargo build --release
   ```
3. **The binary will be available at:**
   ```sh
   target/release/chaostool
   ```

---

## Usage

### CPU Hogger
Simulate high CPU usage:
```sh
chaostool cpuhog --cores 2 --seconds 10 [--remove-safety]
```
- `--cores`: Number of CPU cores to hog
- `--seconds`: Duration to hog the CPU
- `--remove-safety`: (Optional) Remove safety checks (use with caution)

### RAM Hogger
Simulate high RAM usage:
```sh
chaostool memhog --megabytes 1024 --seconds 10 [--remove-safety]
```
- `--megabytes`: Amount of RAM to allocate (in MB)
- `--seconds`: Duration to hold the memory
- `--remove-safety`: (Optional) Remove safety checks (use with caution)

### Database Connection Hogger
Open and hold many database connections:
```sh
chaostool dbfull --users 50 --seconds 10 --dburl <DATABASE_URL> [--remove-safety]
```
- `--users`: Number of connections to open
- `--seconds`: Duration to hold the connections
- `--dburl`: The database connection string (e.g., postgres://user:pass@host/db)
- `--remove-safety`: (Optional) Remove safety checks (use with caution)

### Database Failure Simulation
Temporarily block access to a remote database by manipulating firewall rules over SSH:
```sh
chaostool dbfailure <REMOTE_HOST> --remote_port 5432 --seconds 10
```
- `<REMOTE_HOST>`: The SSH address of the remote host (e.g., root@your-db-server)
- `--remote_port`: The port of the database service (e.g., 5432 for PostgreSQL)
- `--seconds`: Duration to block the database (in seconds)

**Warning:** This command is extremely unsafe by nature. Use at your own risk and only on test environments!

---

This tool is under active development and will continue to evolve with more chaos engineering features tailored to my needs.
