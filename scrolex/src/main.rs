use clap::Parser;

mod watch {
    use std::process::Command;
    use std::{thread, time};

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
                // Command execution directly outputs the 'Output' object after '.expect()'
                let output = Command::new("sh")
                    .arg("-c")
                    .arg(&self.cmd)
                    .output()
                    .expect("Failed to execute command");

                // Check and print stdout and stderr
                if !output.stdout.is_empty() {
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                }
                if !output.stderr.is_empty() {
                    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                }

                // Sleep for the specified interval
                thread::sleep(time::Duration::from_millis(self.interval as u64));
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