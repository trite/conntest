mod err;
mod options;

use err::Result;
use options::Options;

use std::net::{TcpStream};

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

    for info in options.to_scan {
        match TcpStream::connect_timeout(&info.addr, options.timeout.unwrap_or(Options::DEFAULT_TIMEOUT)) {
            Ok(_) => println!("{}({}) is open", info.display_name, info.addr),
            Err(_) => println!("{}({}) is closed", info.display_name, info.addr),
        }
    }

    for fail in options.cannot_scan {
        println!("{} cannot be scanned", fail);
    }

    Ok(())
}

// TODO: Looping, eventually
// Can now see scans that couldn't be performed:
/*
    PS C:\git\conntest> cargo run -- localhost,google.com,blorg.badtld 80,443
    Compiling conntest v0.1.0 (C:\git\conntest)
        Finished dev [unoptimized + debuginfo] target(s) in 0.64s
        Running `target\debug\conntest.exe localhost,google.com,blorg.badtld 80,443`
    to_scan: [HostInfo { display_name: HostName("localhost"), addr: [::1]:80 }, HostInfo { display_name: HostName("localhost"), addr: [::1]:443 }, HostInfo { display_name: HostName("google.com"), addr: 142.250.217.142:80 }, HostInfo { display_name: HostName("google.com"), addr: 142.250.217.142:443 }]
    timeout: None
    delay: None
    localhost([::1]:80) is open
    localhost([::1]:443) is closed
    google.com(142.250.217.142:80) is open
    google.com(142.250.217.142:443) is open
    blorg.badtld:80 cannot be scanned
    blorg.badtld:443 cannot be scanned
    PS C:\git\conntest>
*/