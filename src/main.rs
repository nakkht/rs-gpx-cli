mod cli;
use cli::Cli;
use structopt::StructOpt;

fn main() {
	let args = Cli::from_args();
	let result = std::fs::read_to_string(args.input);
	let content = match result {
	    Ok(content) => { content },
	    Err(error) => { panic!("Can't deal with {}, just exit here", error); }
	};
	println!("{}", content);
} 