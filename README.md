Rewrite of a decade+ old C# tool I wrote and have been using to monitor ports on servers. Already looking like an improvement in a few ways, especially memory usage:

![taskman](taskman.png)

# Uses
Watching port(s) on a server for availablility. Uses simple polling logic to check availability of everything in its list, then wait `delay` seconds and repeat.

## Example
Running `conntest.exe localhost,google.com,baddomain.badtld 80,443`
```
baddomain.badtld:80 cannot be scanned
baddomain.badtld:443 cannot be scanned

CTRL-C to break

localhost([::1]:80) is open
localhost([::1]:443) is closed
google.com(142.250.188.238:80) is open
google.com(142.250.188.238:443) is open

localhost([::1]:80) is open
localhost([::1]:443) is closed
google.com(142.250.188.238:80) is open
google.com(142.250.188.238:443) is open
^C
```

# CLI usage (-h to view):
Running `conntest.exe -h`:
```
Connection Test 0.1.0

USAGE:
    conntest.exe [OPTIONS] <HOSTS> <PORTS>

ARGS:
    <HOSTS>    Host(s) to connect to. Can be a comma separated list of hosts or a single host.
    <PORTS>    Port(s) to connect to. Can be a comma separated list of ports or a single port.

OPTIONS:
    -d, --delay <DELAY>        Delay in seconds between each connection attempt. Default: 5   
    -h, --help                 Print help information
    -t, --timeout <TIMEOUT>    Timeout in seconds to wait for a response. Default: 10
    -v, --verbose              Verbose output
    -V, --version              Print version information
```