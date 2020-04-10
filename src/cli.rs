use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
pub struct Cli {

	#[structopt(parse(from_os_str), short)]
	pub input: PathBuf,
	
	#[structopt(parse(from_os_str), short)]
	pub output: Option<PathBuf>,

	#[structopt(long)]
	pub speed: Option<u32>
}