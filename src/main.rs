mod cli;
mod parser;
use cli::Cli;
use structopt::StructOpt;

const EARTH_RADIUS_IN_METERS: f32 = 6_371_000.0;

fn main() {
	let args = Cli::from_args();
	let time_stamps = parser::process(args.input);
  for (index, element) in time_stamps.iter().enumerate() {
    if index + 1 >= time_stamps.len() { break; }
    let distance = distance_in_meters(element, &time_stamps[index + 1]);
    println!("{}", distance);
  }
}

fn distance_in_meters(point1: &(f32, f32), point2: &(f32, f32)) -> u32 {
  let lat1_rad = point1.0.to_radians();
  let lat2_rad = point2.0.to_radians();

  let lat_diff_rad = (point2.0 - point1.0).to_radians();
  let long_diff_rad = (point2.1 - point1.1).to_radians();

  let a = f32::sin(lat_diff_rad/2.0).powi(2) + f32::cos(lat1_rad) * f32::cos(lat2_rad) * f32::sin(long_diff_rad/2.0).powi(2);
  let distance = f32::atan2(f32::sqrt(a), f32::sqrt(1.0-a)) * 2.0;

  return (distance * EARTH_RADIUS_IN_METERS) as u32;
}