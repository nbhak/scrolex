use clap::Parser;

mod terminal {
    use std::process::Command;
    use std::io::{self, Write};

    pub fn execute(cmd: &str) {
        let output: std::process::Output = Command::new("sh")
                    .arg("-c")
                    .arg(cmd)
                    .output()
                    .expect("Failed to execute command");

        // Check and print stdout and stderr
        if !output.stdout.is_empty() {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
        if !output.stderr.is_empty() {
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        }
    }

    pub fn clear() {
        print!("\x1B[2J\x1B[1;1H");
        io::stdout().flush().expect("Failed to flush stdout");
    }
}

mod watch {
    use std::{thread, time};
    use crate::terminal;

    pub struct Watch {
        pub cmd: String,
        pub interval: u32,
    }

    impl Watch {
        pub fn new(cmd: String, interval: u32) -> Watch{
            Watch {cmd, interval}
        }
        pub fn run(&self) {
            loop {
                terminal::execute(&self.cmd);
                thread::sleep(time::Duration::from_millis(self.interval as u64));
                terminal::clear();
            }
        }
    }
}

#[derive(Parser, Debug)]
#[command(version = "1.0", about = "watch with scrolling", long_about = None)]
struct Args {
    /// Interval in milliseconds to refresh
    #[arg(short = 'n', long = "interval", default_value_t = 1000)]
    interval: u32,

    /// Command to execute repeatedly
    command: String,
}

fn main() {
    let args = Args::parse();
    let watcher = watch::Watch::new(args.command, args.interval);
    println!("Hello, world!");
    watcher.run();
}