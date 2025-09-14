pub mod client;
pub mod server;
use clap::{Parser, ValueEnum};


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    fluent_type: FluentType,
}
#[derive(Debug, ValueEnum, Clone, Copy)]
enum FluentType {
    Client,
    Server,
}

fn main() {
    let args = Args::parse();
    match args.fluent_type {
        FluentType::Client => client::create(),
        FluentType::Server => server::create(),
    }
}
