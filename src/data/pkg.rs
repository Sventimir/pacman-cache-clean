use std::io;
use std::fs;
use std::path::Path;
use std::time;


pub const CACHE_DIR : &str = "/var/cache/pacman/pkg";

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Pkg {
  pub pkg_name: String,
  pub file_name: String,
  ctime: time::SystemTime
}

pub enum ReadError {
  ParseError(String),
  MetaReadError(String, io::Error)
}

fn split_on_colon(fname: &str) -> Option<String> {
  let mut split = fname.split(':');
  let suf = split.next_back();
  let name = split.next_back();
  let nothing = split.next_back();
  match (name, suf, nothing) {
    (Some(n), Some(_), None) =>
      Some(n.to_string()),
    _ =>
      None
  }
}

fn split_on_dash(fname: &str) -> Option<String> {
  let mut split = fname.split('-');
  let suf_opt = split.next_back();
  let subver_opt = split.next_back();
  let (_suffix, _subver, _ver) = match (suf_opt, subver_opt) {
    (Some(suf), Some(sv_str)) =>
      match sv_str.parse::<u16>() {
        Ok(subver) => {
          let v_opt = split.next_back();
          match v_opt {
            None =>
              return None,
            Some(v) =>
              (suf, Some(subver), v)
          }
        },
        Err(_) =>
          (suf, None, sv_str)
      },
    _ =>
      return None
  };
  let name = split.collect::<Vec<&str>>().join("-");
  if name.len() > 0 {
    Some(name)  
  } else {
    None
  }
}

impl Pkg {

  // This is specifically exposed for testing purposes.
  pub fn parse_pkg_name(fname: &str) -> Option<String> {
    match split_on_colon(fname) {
      None => split_on_dash(fname),
      name => name
    }
  }

  pub fn new(file_name: String) -> Result<Pkg, ReadError> {
    let cache_dir = Path::new(CACHE_DIR);
    let pkg_name = match Pkg::parse_pkg_name(&file_name) {
      None => return Err(ReadError::ParseError(file_name)),
      Some(name) => name
    };
    let meta = match fs::metadata(cache_dir.join(&file_name)) {
      Ok(meta) => meta,
      Err(e) => return Err(ReadError::MetaReadError(file_name, e))
    };
    let created = match meta.created() {
      Ok(ctime) => ctime,
      Err(e) => return Err(ReadError::MetaReadError(file_name, e))
    };
    Ok(Pkg { pkg_name, file_name, ctime: created })
  }

}

impl ReadError {
  pub fn show(&self) -> String {
    match self {
      ReadError::MetaReadError(fname, err) =>
        format!("Could not read metadata for file '{}' ({})!", fname, err),
      ReadError::ParseError(fname) =>
        format!("Could not parse filename: '{}'!", fname)
    }
  }
}

pub struct PkgWithVersions {
  pub current: Pkg,
  pub versions: Vec<Pkg>
}

impl PkgWithVersions {
  pub fn new(pkg: Pkg) -> PkgWithVersions {
    PkgWithVersions { current: pkg, versions: Vec::new() }
  }

  pub fn add_version(&mut self, ver: Pkg) {
    assert!(self.current.pkg_name == ver.pkg_name);
    if self.current.ctime > ver.ctime {
      self.versions.push(self.current.clone())
    } else {
      self.versions.push(self.current.clone());
      self.current = ver;
    }
  }
}
