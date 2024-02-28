mod data;

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use data::pkg;

fn main() {
  let mut pkgs: HashMap<String, pkg::PkgWithVersions> = HashMap::new();
  let cache_dir = Path::new(pkg::CACHE_DIR);
  for entry in fs::read_dir(cache_dir).unwrap() {
    let fname = entry.unwrap().file_name().into_string().unwrap();
    if fname.ends_with(".pkg.tar.zst") {
      let p = pkg::Pkg::new(fname).unwrap_or_else(|e| panic!("{}", e.show()));
      if pkgs.contains_key(&p.pkg_name) {
        pkgs.get_mut(&p.pkg_name).unwrap().add_version(p);
      } else {
        pkgs.insert(p.pkg_name.clone(), pkg::PkgWithVersions::new(p));
      }
    }
  }
  for (_, pkg) in pkgs.iter() {
    for ver in pkg.versions.iter() {
      println!("Removing old package version: {}.", ver.file_name);
      fs::remove_file(cache_dir.join(&ver.file_name)).unwrap_or_else(|e| panic!("{}", e))
    }
  }
}
