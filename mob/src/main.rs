// TODO: Delete?

use clap::Parser;
use mob::spawn_mob;

mod core;
mod mob;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  /// Name of the person to greet
  #[arg(short, long)]
  address: String,

  /// Number of times to greet
  #[arg(short, long)]
  peers: Vec<String>,
}

fn main() {
  let args = Args::parse();
  spawn_mob(args.address, args.peers);
}
