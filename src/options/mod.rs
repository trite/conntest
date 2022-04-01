use clap::{Arg, Command, ArgMatches};
use std::{
    net::{
        SocketAddr,
        ToSocketAddrs,
    },
    time::Duration,
    fmt::{
        Debug,
        Display,
        Formatter,
        Result as FmtResult,
    },
};

use crate::err::{Error, Result};


#[derive(Debug, Clone)]
pub struct HostName(String);
impl Display for HostName {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
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

#[derive(Clone)]
pub struct HostInfo {
    pub display_name: HostName,
    pub addr: SocketAddr,
}
impl Debug for HostInfo {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        write!(fmt, "{}:{}", self.display_name, self.addr)
    }
}
impl Display for HostInfo {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        write!(fmt, "{}:{}", self.display_name, self.addr)
    }
}

#[derive(Debug, Clone)]
pub struct Options {
    pub to_scan: Vec<HostInfo>,
    pub scan_count: usize,
    pub cannot_scan: Vec<HostName>,
    pub timeout: Duration,
    pub delay: Duration,
    pub verbose: bool,
}

pub struct Defaults {
    pub timeout: Duration,
    pub delay: Duration,
}

impl Options {
    pub fn load(defaults: Defaults) -> Result<Self> {
        let matches =
            Command::new("Connection Test")
                .version(env!("CARGO_PKG_VERSION"))
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
                        .help(
                            format!(
                                "Timeout in seconds to wait for a response. Default: {}",
                                defaults.timeout.as_secs()
                            ).as_str()
                        )
                        .takes_value(true)
                        .required(false)
                )
                .arg(
                    Arg::new("delay")
                        .short('d')
                        .long("delay")
                        .value_name("DELAY")
                        .help(
                            format!(
                                "Delay in seconds between each connection attempt. Default: {}",
                                defaults.delay.as_secs()
                            ).as_str()
                        )
                        .takes_value(true)
                        .required(false)
                )
                .arg(
                    Arg::new("verbose")
                        .short('v')
                        .long("verbose")
                        .help("Verbose output")
                        .takes_value(false)
                        .required(false)
                )
                .get_matches();

        fn get_to_scan(matches: &ArgMatches) -> Result<(Vec<HostInfo>, Vec<HostName>, usize)> {
            fn get_str_option(matches: &ArgMatches, name: &str) -> Result<String> {
                match matches.value_of(name) {
                    Some(s) => Ok(s.to_string()),
                    None => Err(Error::MissingRequiredArgument(name.into()).into())
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
            let mut scan_count = 0;
            for host in hosts {
                for port in &ports {
                    let addr = format!("{}:{}", host, port).to_string();

                    if let Ok(socket_addr) = addr.to_socket_addrs() {
                        if let Some(ip) = socket_addr.clone().next() {
                            to_scan.push(HostInfo {
                                display_name: HostName(host.clone()),
                                addr: ip
                            });
                            scan_count += 1;
                        }
                    } else {
                        bad_hosts.push(HostName(addr));
                    }
                }
            }

            match to_scan.len() {
                0 => Err(Error::MissingRequiredArgument("hosts".into()).into()),
                _ => Ok((to_scan, bad_hosts, scan_count))
            }
        }
        let (to_scan, cannot_scan, scan_count) = get_to_scan(&matches)?;

        fn get_duration(matches: &ArgMatches, name: &str, fallback: Duration) -> Result<Duration> {
            match matches.value_of(name) {
                Some(s) => Ok(Duration::from_secs(s.parse::<u32>()?.into())),
                None => Ok(fallback)
            }
        }
        
        Ok(Options {
            to_scan,
            cannot_scan,
            scan_count,
            timeout: get_duration(&matches, "timeout", defaults.timeout)?,
            delay: get_duration(&matches, "delay", defaults.delay)?,
            verbose: matches.is_present("verbose"),
        })
    }
}