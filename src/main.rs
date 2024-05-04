use cardio::CardBuilder;
use clap::Parser;

fn main() {
    let matches = CardBuilder::parse().verify().unwrap();

    println!("{}", matches);
}
