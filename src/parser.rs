use std::path::PathBuf;
use std::fs::{read_to_string, File};
use quick_xml::{Reader, Writer};
use quick_xml::events::{Event, BytesStart, BytesText, BytesEnd, BytesDecl};
use std::str::{from_utf8, FromStr};
use std::io::Cursor;
use std::collections::VecDeque;
use std::io::prelude::*;
use chrono::prelude::*;
use chrono::{Duration, Utc};

type Coordinate = (f32, f32);

const WPT_TAG: &[u8] = "wpt".as_bytes();
const TRK_TAG: &[u8] = "trk".as_bytes();
const LAT_TAG: &[u8] = "lat".as_bytes();
const LON_TAG: &[u8] = "lon".as_bytes();
const EARTH_RADIUS_IN_METERS: f32 = 6_371_000.0;

pub fn process(input_file: &PathBuf, speed: f32) -> VecDeque<DateTime<Utc>>  {
  let content = read_to_string(&input_file).expect("could not read file");
  let mut reader = load(&content);
  let coordinates = get_coordinates(&mut reader);
  let mut distances = Vec::with_capacity(coordinates.len() - 1);
  for (index, element) in coordinates.iter().enumerate() {
    if index + 1 >= coordinates.len() { break; }
    let distance = distance_in_meters(element, &coordinates[index + 1]);
    distances.push(distance);
  }
  let speed_in_mps = (0.277778 * speed).round() as u32;
  return generate_timestamps(distances, speed_in_mps);
}

fn load<'a>(xml: &'a String) -> Reader<&'a [u8]> {
  let mut reader = Reader::from_str(&xml);
  reader.trim_text(true);
  return reader;
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

fn generate_timestamps(distances: Vec<u32>, speed: u32) -> VecDeque<DateTime<Utc>> {
  let mut time_stamps = VecDeque::with_capacity(distances.len() + 1);
  time_stamps.push_back(Utc.ymd(2000, 1, 1).and_hms(0, 0, 0));
  for (index, distance) in distances.iter().enumerate() {
    let time = distance / speed;
    let new_time = time_stamps[index] + Duration::seconds(time as i64);
    time_stamps.push_back(new_time);
  }
  return time_stamps
}

fn get_coordinates<'a>(reader: &mut Reader<&'a [u8]>) -> Vec<Coordinate> {
  let mut buffer = Vec::new();
  let mut coordinates = Vec::new();
  loop {
    match reader.read_event(&mut buffer) {
      Ok(Event::Start(ref element)) => {
        if let Some (lat_long) = extract_coordinates(element) {
          coordinates.push(lat_long);
        }
      },
      Ok(Event::Eof) => break,
      Err(error) => panic!("error at at position {}: {:?}",  reader.buffer_position(), error),
      _ => ()
    }
  }
  buffer.clear();
  return coordinates;
}

fn extract_coordinates(element: &BytesStart) -> Option<Coordinate> {
  match element.name() {
    WPT_TAG | TRK_TAG => {
      let latitude = element.attributes().find(|e| 
        match e {
          Ok(attribute) => attribute.key == LAT_TAG,
          _ => false,
        }).unwrap();
      let longitude = element.attributes().find(|e| 
        match e {
          Ok(attribute) => attribute.key == LON_TAG,
          _ => false,
        }).unwrap();
      let lat_float = f32::from_str(from_utf8(&latitude.unwrap().value.into_owned()).unwrap());
      let long_float = f32::from_str(from_utf8(&longitude.unwrap().value.into_owned()).unwrap());
      return Some((lat_float.unwrap(), long_float.unwrap()))
    },
    _ => None
  }
}

pub fn generate_gpx(time_stamps: &mut VecDeque<DateTime<Utc>>, source: &PathBuf) -> Vec<u8> {
  let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), '\t' as u8, 1);
  let content = read_to_string(&source).expect("could not read file");
  let mut reader = load(&content);
  let mut buffer = Vec::new();
  loop {
    match reader.read_event(&mut buffer) {
      Ok(Event::Decl(element)) => {
        assert!(writer.write_event(Event::Decl(BytesDecl::into_owned(element))).is_ok());
      }
      Ok(Event::Start(element)) => {
        assert!(writer.write_event(Event::Start(BytesStart::borrowed(&element, element.name().len()))).is_ok());
        match element.name() {
          WPT_TAG | TRK_TAG => {
            let value = time_stamps.pop_front().unwrap().to_rfc3339_opts(SecondsFormat::Secs, true);
            let new_tag = format!("<time>{}</time>", value);
            let value = BytesText::from_escaped_str(&new_tag);
            assert!(writer.write_event(Event::Text(value)).is_ok());
          },
          _ => ()
        }
      },
      Ok(Event::End(element)) => {
        assert!(writer.write_event(Event::End(BytesEnd::borrowed(&element))).is_ok());
      },
      Ok(Event::Eof) => break,
      Err(error) => panic!("error at at position {}: {:?}",  reader.buffer_position(), error),
      _ => ()
    }
  }
  buffer.clear();
  return writer.into_inner().into_inner();
}

pub fn write(content: &Vec<u8>, mut destination: PathBuf) {
  destination.push("output");
  destination.set_extension("gpx");
  let mut file = File::create(destination.as_path()).expect("unable to create a file");
  assert!(file.write_all(content).is_ok());
}
