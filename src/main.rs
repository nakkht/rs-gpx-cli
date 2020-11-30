mod cli;
mod parser;
use cli::Cli;
use structopt::StructOpt;

fn main() {
  let args = Cli::from_args();
  let mut time_stamps = parser::process(&args.input, args.speed);
  let destination = args.input.parent().unwrap().to_path_buf();
  let output = parser::generate_gpx(&mut time_stamps, &args.input);
  parser::write(&output, destination);
}
