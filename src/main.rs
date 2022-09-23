#![feature(path_file_prefix)]

mod macos;
use self::macos::{home_dir, home_dir_string};
use clap::Parser;
use regex::Regex;
use std::collections::HashMap;
use std::fs::{self, remove_dir, rename};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
  #[clap(value_parser)]
  input: String,
}

struct Context {
  pub files: HashMap<PathBuf, Vec<PathBuf>>,
}

impl Context {
  pub fn new() -> Self {
    Self {
      files: HashMap::new(),
    }
  }

  pub fn collect_files(&mut self, dir: &Path) -> &mut Self {
    let expect = "Failed to read file path.";
    for sub_path in fs::read_dir(dir).expect(expect).flatten() {
      let p = sub_path.path();
      if p.is_file() {
        if let Some(paths) = self.files.get_mut(dir) {
          paths.push(p);
        } else {
          self.files.insert(dir.to_path_buf(), vec![p]);
        }
      } else {
        self.collect_files(&p);
      }
    }
    &mut *self
  }

  pub fn move_files(&mut self, root: &Path) -> &mut Self {
    self.files.iter().for_each(|(parent, files)| {
      let prefix = parent.to_str().expect("Failed to get dir name.");
      let home = home_dir_string().unwrap();
      let prefix_regex0 = Regex::new(home.as_str()).expect("Failed to new prefix_regex0.");
      let prefix_regex1 = Regex::new(r"/").expect("Failed to new prefix_regex1.");
      let prefix = prefix_regex0.replace_all(prefix, "").to_string();
      let prefix = prefix_regex1.replace_all(&prefix, "_").into_owned();
      for file in files {
        let file_absolute_path = file.to_str().expect("Failed to get file path.");
        let filename = file
          // stable rust should use file_stem
          .file_prefix()
          .and_then(|f| f.to_str())
          .expect("Failed to get file prefix name.");

        let file_regex = Regex::new(r"^\.").expect("Failed to new file regex.");
        if file_regex.is_match(filename) {
          continue;
        }

        let file_ext = file
          .extension()
          .and_then(|f| f.to_str())
          .expect("Failed to get file ext.");

        let to =
          root.to_str().unwrap().to_string() + "/" + filename + prefix.as_str() + "." + file_ext;
        rename(file_absolute_path, to).unwrap();
      }
    });
    self.files = HashMap::new();
    &mut *self
  }

  pub fn remove_empty_dirs(&mut self, root: &Path) {
    for dir_entry in root.read_dir().expect("Failed to get sub paths").flatten() {
      let p = dir_entry.path();
      if p.is_dir() {
        self.remove_empty_dirs(&p);
      }
    }
    if root
      .read_dir()
      .map(|mut i| i.next().is_none())
      .unwrap_or(false)
    {
      remove_dir(root).unwrap();
    }
  }
}

fn main() {
  let args: Args = Args::parse();
  let p = Path::new(&args.input);
  if !p.is_dir() {
    panic!("The input is not directory!");
  }

  if home_dir().is_none() {
    return;
  }

  let mut ctx = Context::new();
  ctx.collect_files(p).move_files(p).remove_empty_dirs(p);
}
