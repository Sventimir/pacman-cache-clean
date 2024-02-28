mod data;

use std::fs;
use data::pkg;

const CACHE_DIR : &str = "/var/cache/pacman/pkg";

fn main() {
  for entry in fs::read_dir(CACHE_DIR).unwrap() {
    let fname = entry.unwrap().file_name().into_string().unwrap();
    if fname.ends_with(".pkg.tar.zst") {
      let p = pkg::Pkg::new(fname).unwrap();
      println!("{}: {}", p.file_name, p.pkg_name);
    }
  }
}
