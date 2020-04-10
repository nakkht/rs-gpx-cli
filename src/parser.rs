use std::path::PathBuf;
use std::fs::*;
use quick_xml::Reader;
use quick_xml::events::*;

const WPT_TAG: &[u8] = "wpt".as_bytes();
const TRK_TAG: &[u8] = "trk".as_bytes();
const LAT_TAG: &[u8] = "lat".as_bytes();
const LON_TAG: &[u8] = "lon".as_bytes();

pub fn process(input_file: PathBuf) {
	let content = read_to_string(input_file).expect("could not read file");
	let mut reader = load(&content);
  replace(&mut reader);
}

fn load<'a>(xml: &'a String) -> Reader<&'a [u8]> {
	let mut reader = Reader::from_str(&xml);
	reader.trim_text(true);
  return reader;
}

fn replace<'a>(reader: &mut Reader<&'a [u8]>) {
  let mut buffer = Vec::new();
  loop {
    match reader.read_event(&mut buffer) {
      Ok(Event::Start(ref element)) => match_tags(element),
      Ok(Event::Eof) => break,
      Err(error) => panic!("error at at position {}: {:?}",  reader.buffer_position(), error),
      _ => ()
    }
  }
}

fn match_tags(element: &BytesStart) {
  match element.name() {
    WPT_TAG | TRK_TAG => {
      let values = element.attributes().find(|e| 
        match e {
          Ok(attribute) => attribute.key == LAT_TAG || attribute.key == LON_TAG,
          Err(_) => false,
        }).unwrap();
      println!("{:?}", values);
    },
    _ => ()
  }
}