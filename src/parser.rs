use std::path::PathBuf;
use std::fs::*;

pub fn process(input_file: PathBuf) {
	let content = read_to_string(input_file).expect("could not read file");
	println!("{}", content);
}