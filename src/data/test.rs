use super::pkg::Pkg;

#[test]
fn test_alsa_plugins() {
  let act = Pkg::new("alsa-plugins-1:1.2.7.1-2-x86_64.pkg.tar.zst");
  let exp = Pkg::ColonSepPkg(
    "alsa-plugins-1".to_string(),
    "1.2.7.1-2-x86_64.pkg.tar.zst".to_string()
  );
  assert_eq!(act, Some(exp))
}

#[test]
fn test_audacity() {
  let act = Pkg::new("audacity-1:3.4.2-2-x86_64.pkg.tar.zst");
  let exp = Pkg::ColonSepPkg(
    "audacity-1".to_string(),
    "3.4.2-2-x86_64.pkg.tar.zst".to_string()
  );
  assert_eq!(act, Some(exp))
}

#[test]
fn test_aribb() {
  let act = Pkg::new("aribb24-1.0.3-3-x86_64.pkg.tar.zst");
  let exp = Pkg::DashSepPkg{
    name: "aribb24".to_string(),
    ver: "1.0.3".to_string(),
    subver: Some(3),
    suffix: "x86_64.pkg.tar.zst".to_string()
  };
  assert_eq!(act, Some(exp))
}

#[test]
fn test_acl() {
  let act = Pkg::new("acl-2.3.2-1-x86_64.pkg.tar.zst");
  let exp = Pkg::DashSepPkg{
    name: "acl".to_string(),
    ver: "2.3.2".to_string(),
    subver: Some(1),
    suffix: "x86_64.pkg.tar.zst".to_string()
  };
  assert_eq!(act, Some(exp))
}

#[test]
fn test_adobe_source_code_pro_fonts() {
  let act = Pkg::new("adobe-source-code-pro-fonts-2.042u+1.062i+1.026vf-1-any.pkg.tar.zst");
  let exp = Pkg::DashSepPkg{
    name: "adobe-source-code-pro-fonts".to_string(),
    ver: "2.042u+1.062i+1.026vf".to_string(),
    subver: Some(1),
    suffix: "any.pkg.tar.zst".to_string()
  };
  assert_eq!(act, Some(exp))
} 

#[test]
fn test_archlinux_keyring() {
  let act = Pkg::new("archlinux-keyring-20240208-1-any.pkg.tar.zst");
  let exp = Pkg::DashSepPkg{
    name: "archlinux-keyring".to_string(),
    ver: "20240208".to_string(),
    subver: Some(1),
    suffix: "any.pkg.tar.zst".to_string()
  };
  assert_eq!(act, Some(exp))
}

#[test]
fn test_awahi() {
  let act = Pkg::new("avahi-0.8+22+gfd482a7-4-x86_64.pkg.tar.zst");
  let exp = Pkg::DashSepPkg{
    name: "avahi".to_string(),
    ver: "0.8+22+gfd482a7".to_string(),
    subver: Some(4),
    suffix: "x86_64.pkg.tar.zst".to_string()
  };
  assert_eq!(act, Some(exp))
}

#[test]
fn test_emacs() {
  let act = Pkg::new("emacs-29.2-1-x86_64.pkg.tar.zst");
  let exp = Pkg::DashSepPkg{
    name: "emacs".to_string(),
    ver: "29.2".to_string(),
    subver: Some(1),
    suffix: "x86_64.pkg.tar.zst".to_string()
  };
  assert_eq!(act, Some(exp))
}
