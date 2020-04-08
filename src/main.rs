mod cli;
mod parser;
use cli::Cli;
use structopt::StructOpt;

fn main() {
	let args = Cli::from_args();
	parser::process(args.input)
}