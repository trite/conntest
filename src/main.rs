mod err;

use clap::{Arg, Command, ArgMatches};


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let options = Options::load()?;
    println!("to_scan: {:?}", options.to_scan);
    println!("timeout: {:?}", options.timeout);
    println!("delay: {:?}", options.delay);

    Ok(())
}

#[derive(Debug)]
struct HostInfo {
    _host: String,
    _port: u16,
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
                    to_scan.push(HostInfo { _host: host.clone(), _port: *port });
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


// TODO: Next up is wiring up the actual calls, but for now args are being parsed nicely:
/*
    PS C:\git\conntest> cargo run -- -h "asdf" -p "80"
    Compiling conntest v0.1.0 (C:\git\conntest)
        Finished dev [unoptimized + debuginfo] target(s) in 0.62s                                                                                                                                           
        Running `target\debug\conntest.exe -h asdf -p 80`
    to_scan: [HostInfo { _host: "asdf", _port: 80 }]
    timeout: None
    delay: None
    PS C:\git\conntest> cargo run -- -h "asdf,asdf2" -p "80,3389"
        Finished dev [unoptimized + debuginfo] target(s) in 0.02s
        Running `target\debug\conntest.exe -h asdf,asdf2 -p 80,3389`
    to_scan: [HostInfo { _host: "asdf", _port: 80 }, HostInfo { _host: "asdf", _port: 3389 }, HostInfo { _host: "asdf2", _port: 80 }, HostInfo { _host: "asdf2", _port: 3389 }]
    timeout: None
    delay: None
    PS C:\git\conntest>
*/

// Help looks good so far too:
/*
    PS C:\git\conntest> cargo run -- --help
    Compiling conntest v0.1.0 (C:\git\conntest)
        Finished dev [unoptimized + debuginfo] target(s) in 0.59s
        Running `target\debug\conntest.exe --help`
    Connection Test 0.1.0

    USAGE:
        conntest.exe [OPTIONS] <HOSTS> <PORTS>

    ARGS:
        <HOSTS>    Host(s) to connect to. Can be a comma separated list of hosts or a single host.
        <PORTS>    Port(s) to connect to. Can be a comma separated list of ports or a single port.

    OPTIONS:
        -d, --delay <DELAY>        Delay in seconds between each connection attempt. Default: 30
        -h, --help                 Print help information
        -t, --timeout <TIMEOUT>    Timeout in seconds to wait for a response. Default: 10
        -V, --version              Print version information
    PS C:\git\conntest>
*/