
#[derive(PartialEq, Eq, Debug)]
pub enum Pkg {
  ColonSepPkg(String, String),
  DashSepPkg{ name: String, ver: String, subver:Option<u16>, suffix: String }
}

fn split_on_colon(fname: &str) -> Option<Pkg> {
  let mut split = fname.split(':');
  let suf = split.next_back();
  let name = split.next_back();
  let nothing = split.next_back();
  match (name, suf, nothing) {
    (Some(n), Some(suf), None) =>
      Some(Pkg::ColonSepPkg(n.to_string(), suf.to_string())),
    _ =>
      None
  }
}

fn split_on_dash(fname: &str) -> Option<Pkg> {
  let mut split = fname.split('-');
  let suf_opt = split.next_back();
  let subver_opt = split.next_back();
  let (suffix, subver, ver) = match (suf_opt, subver_opt) {
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
    Some(Pkg::DashSepPkg { name, ver: ver.to_string(), subver, suffix: suffix.to_string() })  
  } else {
    None
  }
}

impl Pkg {

  pub fn new(fname: &str) -> Option<Pkg> {
    match split_on_colon(fname) {
      None => split_on_dash(fname),
      pkg => pkg
    }
  }

  pub fn get_name(&self) -> &str {
    match self {
      Pkg::ColonSepPkg(name, _) => name,
      Pkg::DashSepPkg { name, ver, subver, suffix } => name
    }
  }

  pub fn filename(&self) -> String {
    let mut ret = self.get_name().to_string();
    match self {
      Pkg::ColonSepPkg(name, suffix) => {
        ret.push('-');
        ret.push_str(suffix)
      }
      Pkg::DashSepPkg { name, ver, subver, suffix } => {
        ret.push('-');
        ret.push_str(ver);
        match subver {
          None => (),
          Some(v) => {
            ret.push_str(&v.to_string());
          }
        }
        ret.push('-');
        ret.push_str(suffix);
      }
    }
    ret
  }
}
