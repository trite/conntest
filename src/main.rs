mod err;
mod options;

use crossterm::{
    terminal::{enable_raw_mode, disable_raw_mode},
    event::{self, Event as CEvent, KeyCode},
};

use err::Result;
use options::{Options, Defaults, HostInfo};
use tui::{
    backend::CrosstermBackend,
    Terminal,
    layout::{Layout, Direction, Constraint}
};

use std::{
    io,
    net::TcpStream,
    sync::mpsc,
    collections::HashMap,
    thread::{self, sleep},
    time::{Duration, Instant},
};

enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Debug)]
enum Connectivity {
    Success,
    Failure,
}

#[derive(Debug)]
enum ConnectivityUpdate {
    Success(HostInfo),
    Failure(HostInfo),
}

struct State {
    history: HashMap<String, Vec<Connectivity>>,
}

fn main() -> Result<()>{
    enable_raw_mode().expect("can run in raw mode");

    let (cmdTx, cmdRx) = mpsc::channel();
    let (updateTx, updateRx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);

    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    cmdTx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = cmdTx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

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

    // Scanning thread
    thread::spawn(move || {
        // let delay = options.delay.clone();
        // let to_scan = options.to_scan.as_slice().to_vec();
        loop {
            if last_run.elapsed() >= options.delay {
                for info in &options.to_scan {
                    match TcpStream::connect_timeout(&info.addr, options.timeout) {
                        Ok(_) => updateTx.send(ConnectivityUpdate::Success(info.clone())).unwrap(),
                        Err(_) => updateTx.send(ConnectivityUpdate::Failure(info.clone())).unwrap()
                    }
                }
                last_run = Instant::now();
            }
            sleep(options.delay);
        }
    });


    loop {
        let mut state = State {
            history: HashMap::new()
        };

        terminal.draw(|rect| {
            let size = rect.size();
            let mut constraints = vec![];
            for _ in 0..state.history.len() {
                constraints.push(Constraint::Length(2));
            }
            // let constraints: Vec<Constraint> = options.to_scan.clone().iter().map(|h| Constraint::Length(2)).collect::<Vec<Constraint>>().clone();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(constraints)
                .split(size);

            // TODO: Draw out the sparklines for each history
            // (might need to use a running index while inserting into constraint sections)
            println!("{:?}", state.history);
        });

        match cmdRx.recv()? {
            Event::Input(evt) => match evt.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }
                _ => {}
            },
            Event::Tick => { }
        };

        match updateRx.recv()? {
            ConnectivityUpdate::Success(host) => {
                state.history
                    .entry(host.to_string())
                    .or_insert(vec![])
                    .push(Connectivity::Success);
            },
            ConnectivityUpdate::Failure(host) => {
                state.history
                    .entry(host.to_string())
                    .or_insert(vec![])
                    .push(Connectivity::Failure);
            },
        };
    }

    Ok(())
}