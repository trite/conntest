mod err;
mod options;

use err::Result;
use options::{Options, Defaults};

use std::net::TcpStream;
use std::thread::sleep;
use std::time::{Duration, Instant};

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

const POLL_INTERVAL: Duration = Duration::from_millis(100);

fn main() -> Result<()>{
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    let options = Options::load(Defaults {
        timeout: Duration::from_secs(10),
        delay: Duration::from_secs(5),
    })?;

    if options.verbose {
        println!("====== Details ======");
        println!("  to_scan:");
        for host in &options.to_scan {
            println!("    {}", host);
        }
        println!("\n  cannot_scan:");
        for host in &options.cannot_scan {
            println!("    {}", host);
        }
        println!("\n  timeout: {:?}", options.timeout);
        println!("  delay: {:?}", options.delay);
        println!("====== Output ======");
    }

    for fail in options.cannot_scan {
        println!("{} cannot be scanned", fail);
    }

    println!("\nCTRL-C to break\n");

    let mut last_run = Instant::now().checked_sub(options.delay).unwrap();

    while running.load(Ordering::SeqCst) {
        if last_run.elapsed() >= options.delay {
            for info in &options.to_scan {
                match TcpStream::connect_timeout(&info.addr, options.timeout) {
                    Ok(_) => println!("{}({}) is open", info.display_name, info.addr),
                    Err(_) => println!("{}({}) is closed", info.display_name, info.addr),
                }
            }
            println!();
            last_run = Instant::now();
        }
        sleep(POLL_INTERVAL);
    }

    Ok(())
}

// TODO: UI work and improvements?
// Current state:
/*
    PS C:\git\conntest> cargo run -- localhost,google.com,blorg.badtld 80,443 -v
    Compiling conntest v0.1.0 (C:\git\conntest)
        Finished dev [unoptimized + debuginfo] target(s) in 1.13s
        Running `target\debug\conntest.exe localhost,google.com,blorg.badtld 80,443 -v`
    ====== Details ======
    to_scan:
        localhost([::1]:80)
        localhost([::1]:443)
        google.com(172.217.14.110:80) 
        google.com(172.217.14.110:443)

    cannot_scan:
        blorg.badtld:80
        blorg.badtld:443

    timeout: None
    delay: None
    ====== Output ======
    blorg.badtld:80 cannot be scanned
    blorg.badtld:443 cannot be scanned
    localhost([::1]:80) is open
    localhost([::1]:443) is closed
    google.com(172.217.14.110:80) is open
    google.com(172.217.14.110:443) is open

    localhost([::1]:80) is open
    localhost([::1]:443) is closed
    google.com(172.217.14.110:80) is open
    google.com(172.217.14.110:443) is open

    error: process didn't exit successfully: `target\debug\conntest.exe localhost,google.com,blorg.badtld 80,443 -v` (exit code: 0xc000013a, STATUS_CONTROL_C_EXIT)
    PS C:\git\conntest> 
*/