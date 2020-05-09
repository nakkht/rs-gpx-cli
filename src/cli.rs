use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
pub struct Cli {

  /// Absolute path to the input file
	#[structopt(parse(from_os_str), short = "i")]
	pub input: PathBuf,
	
  /// Absolute path to the output file. Input file folder with name output.gpx otherwise
	#[structopt(parse(from_os_str), short = "o")]
	pub output: Option<PathBuf>,

  /// Constant spped of waypoints in km/h
	#[structopt(short = "s")]
	pub speed: f32
}