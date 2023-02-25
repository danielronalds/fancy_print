use clap::Parser;
use std::time;

use fancy_hello_world::FancyPrinter;

#[derive(Parser)]
#[clap(author, version, about)]
/// That fancy instagram animation for hello world in rust
struct ProgramOptions {
    #[arg(long, short)]
    /// The string to print, the default being "Hello, world!"
    string: Option<String>,

    #[arg(long, short)]
    /// The milliseconds taken inbetween char iterations, the default being 2ms
    time: Option<u64>,

    #[arg(long, short)]
    /// Whether new iterations should be printed on the same line
    one_line: bool,
}

const DEFAULT_STR: &str = "Hello, world!";

const DEFAULT_TIME: u64 = 2;

fn main() {
    let args = ProgramOptions::parse();

    let string = args.string.unwrap_or(DEFAULT_STR.to_string());

    let time_to_sleep = time::Duration::from_millis(args.time.unwrap_or(DEFAULT_TIME));

    let printer = FancyPrinter::builder()
        .time_delay(time_to_sleep)
        .one_line(args.one_line)
        .build();

    printer.print(string);
}
