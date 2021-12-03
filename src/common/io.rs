use std::{
  fs::{self, File},
  io::{prelude::*, BufReader},
  path::Path,
};

fn get_input(day: u8) -> String {
  let path = format!("inputs/d{:0>2}.txt", day);
  if Path::new(&path).exists() {
    return path;
  }

  panic!("input file does not exist");
}

pub fn read_lined_file(day: u8) -> Vec<String> {
  let file = File::open(get_input(day)).expect("no such file");
  let buf = BufReader::new(file);

  buf.lines().map(|line| line.unwrap()).filter(|line| !line.is_empty()).collect()
}

pub fn read_split_file(day: u8, sep: &'static str) -> Vec<String> {
  fs::read_to_string(get_input(day)).expect("no such file").trim().split(sep).map(|x| x.to_string()).collect()
}

pub fn read_chared_file(day: u8) -> Vec<String> {
  fs::read_to_string(get_input(day)).expect("no such file").trim().chars().map(|x| x.to_string()).collect()
}
