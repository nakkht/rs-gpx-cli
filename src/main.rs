mod cli;
use cli::Cli;
use structopt::StructOpt;

fn main() {
	let args = Cli::from_args();
	let input_file = args.input.to_str();
	let output_file = args.output.to_str();
	println!("{:?}", args)
} 