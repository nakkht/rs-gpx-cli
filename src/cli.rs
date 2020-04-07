use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
pub struct Cli {

	#[structopt(parse(from_os_str))]
	pub file_path: PathBuf,

	pub speed: u32
}