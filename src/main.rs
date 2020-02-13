mod cli;
use cli::Cli;
use std::path::PathBuf;
use structopt::StructOpt;
use std::fs::*;

fn main() {
	let args = Cli::from_args();
	parse(args.input);
}

fn parse(input_file: PathBuf) {
	let content = read_to_string(input_file).expect("could not read file");
	println!("{}", content);
}