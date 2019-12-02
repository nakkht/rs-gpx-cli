use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Cli {

	#[structopt(parse(from_os_str))]
	pub input: std::path::PathBuf,

	#[structopt(parse(from_os_str))]
	pub output: std::path::PathBuf,

	pub speed: u32
}