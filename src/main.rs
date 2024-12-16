use advent_of_code::y2024::run_day;
use clap::Parser;

/// Advent of Code command line runner
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    year: u32,

    #[arg(short, long)]
    day: u32,
}

fn main() {
    let args = Args::parse();
    match args.year {
        2024 => run_day(args.day),
        year => eprintln!("Unimplemented year: {}", year),
    }
}
