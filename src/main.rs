use clap::Parser;
use regex::Regex;
use std::fs::{read_dir, remove_file, rename, File};
use std::io::{Read, Write};
use std::path::Path;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
  #[clap(value_parser)]
  input: String,
}

const BUFFER_SIZE: usize = 1024;
fn process_file(p: &Path) -> Result<(), anyhow::Error> {
  let mut original_file = File::open(p)?;
  let filename = p
    .file_name()
    .expect("Failed to get filename.")
    .to_str()
    .expect("Failed to unwrap path str.");

  let mut buffer = [0u8; BUFFER_SIZE];
  let file_regex = Regex::new(r"\.mp4$").expect("Failed to new file regex.");
  if file_regex.is_match(filename) {
    let mut count = original_file.read(&mut buffer)?;
    if buffer[0] == 0xFF && buffer[1] == 0xFF && buffer[2] == 0xFF {
      let new_filename = String::from("new_") + filename;
      let new_filename = p.with_file_name(new_filename);
      let new_file_path = Path::new(&new_filename);
      let mut new_file = File::create(new_file_path)?;
      let mut is_first = true;
      while count != 0 {
        if is_first {
          new_file.write_all(&buffer[3..count])?;
          is_first = false;
        } else {
          new_file.write_all(&buffer[..count])?;
        }
        count = original_file.read(&mut buffer)?;
      }

      remove_file(&p)?;
      rename(new_file_path, p)?;
    }
  }

  Ok(())
}

fn read_file(dir: &Path) {
  for sub_path in read_dir(dir).expect("Failed to read file path.").flatten() {
    let p = sub_path.path();
    if p.is_file() {
      process_file(&p).unwrap();
    } else {
      read_file(&p);
    }
  }
}

fn main() {
  let args: Args = Args::parse();
  let p = Path::new(&args.input);
  if !p.is_dir() {
    panic!("The input is not directory!");
  }
  read_file(p);
}
