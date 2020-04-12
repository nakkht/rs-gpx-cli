mod cli;
mod parser;
use cli::Cli;
use structopt::StructOpt;
use chrono::prelude::*;
use chrono::{Duration, Utc};

const EARTH_RADIUS_IN_METERS: f32 = 6_371_000.0;

fn main() {
	let args = Cli::from_args();
	let coordinates = parser::process(args.input);
  let mut distances = Vec::with_capacity(coordinates.len() - 1);
  for (index, element) in coordinates.iter().enumerate() {
    if index + 1 >= coordinates.len() { break; }
    let distance = distance_in_meters(element, &coordinates[index + 1]);
    distances.push(distance);
  }
  let speed_in_mps = (0.277778 * args.speed).round() as u32;
  let time_stamps = generate_timestamps(distances, speed_in_mps);
  println!("{:?}", time_stamps);
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

fn generate_timestamps(distances: Vec<u32>, speed: u32) -> Vec<DateTime<Utc>> {
  let mut time_stamps = Vec::with_capacity(distances.len() + 1);
  time_stamps.push(Utc::now());
  for (index, distance) in distances.iter().enumerate() {
    let time = distance / speed;
    let new_time = time_stamps[index] + Duration::seconds(time as i64);
    time_stamps.push(new_time);
  }
  return time_stamps
}