use clap::{arg, Arg, Command};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    Ok(())
}

struct Options {
    to_scan: Vec<HostInfo>,
    timeout: Option<u32>,
    delay: Option<u32>,
}

impl Options {
    fn load() -> Self {
        let matches =
            Command::new("Connection Test")
                .version(env!("CARGO_PKG_VERSION"))
                // Should probably mention that for now what's scanned is the cartesian product of the hosts and ports
                .arg(
                    Arg::new("hosts")
                        .short('h')
                        .long("hosts")
                        .value_name("HOSTS")
                        .help("Host(s) to connect to. Can be a comma separated list of hosts or a single host.")
                        .takes_value(true)
                        .required(true)
                )
                .arg(
                    Arg::new("ports")
                        .short('p')
                        .long("ports")
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
                        .help("Timeout in seconds to wait for a response.")
                        .takes_value(true)
                        .required(false)
                )
                .arg(
                    Arg::new("delay")
                        .short('d')
                        .long("delay")
                        .value_name("DELAY")
                        .help("Delay in seconds between each connection attempt.")
                        .takes_value(true)
                        .required(false)
                )
                .get_matches();

        let parse = |s: &str| s.parse::<u32>().unwrap();
        
        Options {
            // TODO: Resume here, need to parse hosts/ports into a Vec<HostInfo>
            to_scan: (),
            timeout: matches
                .value_of("timeout")
                .map(parse),
            delay: matches
                .value_of("delay")
                .map(parse)
        }
        
    }
}

struct HostInfo {
    host: String,
    port: u16,
}