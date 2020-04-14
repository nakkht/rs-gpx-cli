use std::path::PathBuf;
use std::fs::{read_to_string, File};
use quick_xml::{Reader, Writer};
use quick_xml::events::{Event, BytesStart, BytesText};
use std::str::{from_utf8, FromStr};
use std::io::Cursor;
use std::collections::VecDeque;
use std::io::prelude::*;
use chrono::Utc;
use chrono::prelude::*;

type Coordinate = (f32, f32);

const WPT_TAG: &[u8] = "wpt".as_bytes();
const TRK_TAG: &[u8] = "trk".as_bytes();
const LAT_TAG: &[u8] = "lat".as_bytes();
const LON_TAG: &[u8] = "lon".as_bytes();

pub fn process(input_file: &PathBuf) -> Vec<(f32, f32)> {
  let content = read_to_string(&input_file).expect("could not read file");
  let mut reader = load(&content);
  return get_coordinates(&mut reader);
}

fn load<'a>(xml: &'a String) -> Reader<&'a [u8]> {
  let mut reader = Reader::from_str(&xml);
  reader.trim_text(true);
  return reader;
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

pub fn write(time_stamps: &mut VecDeque<DateTime<Utc>>, source: &PathBuf, destination: Option<PathBuf>) {
  let mut writer = Writer::new(Cursor::new(Vec::new()));
  let content = read_to_string(&source).expect("could not read file");
  let mut reader = load(&content);
  let mut buffer = Vec::new();
  loop {
    match reader.read_event(&mut buffer) {
      Ok(Event::Start(ref element)) => {
        match element.name() {
          WPT_TAG | TRK_TAG => {
            let value =  time_stamps.pop_front().unwrap();
            let new_tag = format!("<time>{}</time>", value);
            let value = BytesText::from_plain_str(&new_tag);
            assert!(writer.write_event(Event::Text(value)).is_ok());
          },
          _ => ()
        }
      },
      Ok(Event::Eof) => break,
      Err(error) => panic!("error at at position {}: {:?}",  reader.buffer_position(), error),
      _ => ()
    }
  }
  buffer.clear();
  let mut default_path = source.parent().unwrap().to_path_buf();
  default_path.push("output");
  default_path.set_extension("gpx");
  let destination = destination.unwrap_or(default_path);
  println!("{:?}", destination);
  let mut file = File::create(destination.as_path()).expect("unable to create a file");
  assert!(file.write_all(&writer.into_inner().into_inner()).is_ok());
}
