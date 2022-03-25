mod err;

use clap::{Arg, Command, ArgMatches};

use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::fmt::{Display, Formatter};
use std::fmt;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()>{
    let options = Options::load()?;

    println!("to_scan: {:?}", options.to_scan);
    println!("timeout: {:?}", options.timeout);
    println!("  delay: {:?}", options.delay);

    for info in options.to_scan {
        match TcpStream::connect(info.addr) {
            Ok(_) => println!("{}({}) is open", info.display_name, info.addr),
            Err(_) => println!("{}({}) is closed", info.display_name, info.addr),
        }
    }

    for fail in options.cannot_scan {
        println!("{} cannot be scanned", fail);
    }

    Ok(())
}

#[derive(Debug)]
struct HostName(String);
impl Display for HostName {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{}", self.0)
    }
}
impl From<String> for HostName {
    fn from(s: String) -> Self {
        HostName(s)
    }
}
impl From<SocketAddr> for HostName {
    fn from(s: SocketAddr) -> Self {
        HostName(s.to_string())
    }
}

#[derive(Debug)]
struct HostInfo {
    display_name: HostName,
    addr: SocketAddr,
}

#[derive(Debug)]
struct Options {
    to_scan: Vec<HostInfo>,
    cannot_scan: Vec<HostName>,
    timeout: Option<u32>,
    delay: Option<u32>,
}

impl Options {
    pub const DEFAULT_TIMEOUT: u64 = 10;
    pub const DEFAULT_DELAY: u64 = 30;

    fn load() -> Result<Self> {
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
        fn get_int_option(matches: &ArgMatches, name: &str) -> Result<Option<u32>> {
            match matches.value_of(name) {
                Some(s) => Ok(Some(s.parse::<u32>()?)),
                None => Ok(None)
            }
        }

        fn get_to_scan(matches: &ArgMatches) -> Result<(Vec<HostInfo>, Vec<HostName>)> {
            fn get_str_option(matches: &ArgMatches, name: &str) -> Result<String> {
                match matches.value_of(name) {
                    Some(s) => Ok(s.to_string()),
                    None => Err(err::Error::MissingRequiredArgument(name.into()).into())
                }
            }

            let hosts = get_str_option(matches, "hosts")?;
            let ports = get_str_option(matches, "ports")?;

            let hosts: Vec<String> = hosts.split(',').map(|s| s.to_string()).collect();

            fn try_parse_ports(ports: &str) -> Result<Vec<u16>> {
                let ports: Vec<String> = ports.split(',').map(|s| s.to_string()).collect();

                let mut new_ports: Vec<u16> = Vec::new();
                for port in ports {
                    new_ports.push(port.parse::<u16>()?);
                }

                Ok(new_ports)
            }
            let ports: Vec<u16> = try_parse_ports(&ports)?;

            let mut to_scan = Vec::new();
            let mut bad_hosts = Vec::new();
            for host in hosts {
                for port in &ports {
                    let addr = format!("{}:{}", host, port).to_string();

                    if let Ok(socket_addr) = addr.to_socket_addrs() {
                        if let Some(ip) = socket_addr.clone().next() {
                            to_scan.push(HostInfo {
                                display_name: HostName(host.clone()),
                                addr: ip
                            });
                        }
                    } else {
                        bad_hosts.push(HostName(addr));
                    }
                }
            }

            match to_scan.len() {
                0 => Err(err::Error::MissingRequiredArgument("hosts".into()).into()),
                _ => Ok((to_scan, bad_hosts))
            }
        }

        let (to_scan, cannot_scan) = get_to_scan(&matches)?;
        
        Ok(Options {
            to_scan,
            cannot_scan,
            timeout: get_int_option(&matches, "timeout")?,
            delay: get_int_option(&matches, "delay")?,
        })
    }
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