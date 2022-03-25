mod err;

use clap::{Arg, Command, ArgMatches};

use std::net::{SocketAddr, TcpStream, ToSocketAddrs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = Options::load()?;

    println!("to_scan: {:?}", options.to_scan);
    println!("timeout: {:?}", options.timeout);
    println!("  delay: {:?}", options.delay);

    for info in options.to_scan {
        match TcpStream::connect(info.addr) {
            Ok(_) => println!("{} is open", info.display_name),
            Err(_) => println!("{} is closed", info.display_name),
        }
    }

    Ok(())
}

#[derive(Debug)]
struct HostInfo {
    display_name: String,
    addr: SocketAddr,
}

#[derive(Debug)]
struct Options {
    to_scan: Vec<HostInfo>,
    timeout: Option<u32>,
    delay: Option<u32>,
}

impl Options {
    pub const DEFAULT_TIMEOUT: u64 = 10;
    pub const DEFAULT_DELAY: u64 = 30;

    fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let matches =
            Command::new("Connection Test")
                .version(env!("CARGO_PKG_VERSION"))
                // Should probably mention that for now what's scanned is the cartesian product of the hosts and ports
                .arg(
                    Arg::new("hosts")
                        .value_name("HOSTS")
                        .help("Host(s) to connect to. Can be a comma separated list of hosts or a single host.")
                        .takes_value(true)
                        .required(true)
                )
                .arg(
                    Arg::new("ports")
                        .value_name("PORTS")
                        .help("Port(s) to connect to. Can be a comma separated list of ports or a single port.")
                        .takes_value(true)
                        .required(true)
                )
                .arg(
                    Arg::new("timeout")
                        .short('t')
                        .long("timeout")
                        .value_name("TIMEOUT")
                        .help(format!("Timeout in seconds to wait for a response. Default: {}", Self::DEFAULT_TIMEOUT).as_str())
                        .takes_value(true)
                        .required(false)
                )
                .arg(
                    Arg::new("delay")
                        .short('d')
                        .long("delay")
                        .value_name("DELAY")
                        .help(format!("Delay in seconds between each connection attempt. Default: {}", Self::DEFAULT_DELAY).as_str())
                        .takes_value(true)
                        .required(false)
                )
                .get_matches();

        // TODO: Is there some kind of flatten operation that handles this already?
        fn get_int_option(matches: &ArgMatches, name: &str) -> Result<Option<u32>, Box<dyn std::error::Error>> {
            match matches.value_of(name) {
                Some(s) => Ok(Some(s.parse::<u32>()?)),
                None => Ok(None)
            }
        }

        fn get_to_scan(matches: &ArgMatches) -> Result<Vec<HostInfo>, Box<dyn std::error::Error>> {
            fn get_str_option(matches: &ArgMatches, name: &str) -> Result<String, Box<dyn std::error::Error>> {
                match matches.value_of(name) {
                    Some(s) => Ok(s.to_string()),
                    None => Err(err::Error::MissingRequiredArgument(name.into()).into())
                }
            }

            let hosts = get_str_option(matches, "hosts")?;
            let ports = get_str_option(matches, "ports")?;

            let hosts: Vec<String> = hosts.split(',').map(|s| s.to_string()).collect();

            fn try_parse_ports(ports: &str) -> Result<Vec<u16>, Box<dyn std::error::Error>> {
                let ports: Vec<String> = ports.split(',').map(|s| s.to_string()).collect();

                let mut new_ports: Vec<u16> = Vec::new();
                for port in ports {
                    new_ports.push(port.parse::<u16>()?);
                }

                Ok(new_ports)
            }
            let ports: Vec<u16> = try_parse_ports(&ports)?;

            let mut to_scan = Vec::new();
            for host in hosts {
                for port in &ports {
                    let addr = format!("{}:{}", host, port).to_string();
                    to_scan.push(HostInfo {
                        display_name: addr.clone(),
                        // TODO: don't just unwrap this (especially twice!)
                        addr: addr.to_socket_addrs().unwrap().next().unwrap()
                    })
                }
            }

            Ok(to_scan)
        }
        
        Ok(Options {
            to_scan: get_to_scan(&matches)?,
            timeout: get_int_option(&matches, "timeout")?,
            delay: get_int_option(&matches, "delay")?,
        })
    }
}


// TODO: All set with performing a single scan
//       just need to start figuring out some of the finer details around looping and threading/async
/*
    PS C:\git\conntest> cargo run -- localhost 80
        Finished dev [unoptimized + debuginfo] target(s) in 0.02s
        Running `target\debug\conntest.exe localhost 80`        
    to_scan: [HostInfo { display_name: "localhost:80", addr: [::1]:80 }]
    timeout: None       
    delay: None       
    localhost:80 is open
    PS C:\git\conntest> cargo run -- localhost 1174
        Finished dev [unoptimized + debuginfo] target(s) in 0.02s
        Running `target\debug\conntest.exe localhost 1174`      
    to_scan: [HostInfo { display_name: "localhost:1174", addr: [::1]:1174 }]
    timeout: None
    delay: None
    localhost:1174 is closed
    PS C:\git\conntest> cargo run -- localhost,google.com 80,443
        Finished dev [unoptimized + debuginfo] target(s) in 0.02s       
        Running `target\debug\conntest.exe localhost,google.com 80,443`
    to_scan: [HostInfo { display_name: "localhost:80", addr: [::1]:80 }, HostInfo { display_name: "localhost:443", addr: [::1]:443 }, HostInfo { display_name: "google.com:80", addr: 142.250.68.46:80 }, HostInfo { display_name: "google.com:443", addr: 142.250.68.46:443 }]
    timeout: None       
    delay: None       
    localhost:80 is open
    localhost:443 is closed
    google.com:80 is open
    google.com:443 is open
    PS C:\git\conntest>
*/