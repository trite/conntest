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

// TODO: Async and UI