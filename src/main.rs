mod err;
mod options;

use err::Result;
use options::Options;

use std::net::TcpStream;

use std::thread::sleep;

fn main() -> Result<()>{
    let options = Options::load()?;

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

    println!();

    loop {
        for info in &options.to_scan {
            match TcpStream::connect_timeout(&info.addr, options.timeout.unwrap_or(Options::DEFAULT_TIMEOUT)) {
                Ok(_) => println!("{}({}) is open", info.display_name, info.addr),
                Err(_) => println!("{}({}) is closed", info.display_name, info.addr),
            }
        }

        println!();
        sleep(options.delay.unwrap_or(Options::DEFAULT_DELAY));
    }
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