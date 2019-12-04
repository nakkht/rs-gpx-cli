mod cli;
use cli::Cli;
use structopt::StructOpt;

fn main() {
	let args = Cli::from_args();
	parse(args.input);
}

fn parse(input_file:std::path::PathBuf) {
	let content = std::fs::read_to_string(input_file).expect("could not read file");
	println!("{}", content);
}