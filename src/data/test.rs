use super::pkg::Pkg;

#[test]
fn test_alsa_plugins() {
  let act = Pkg::parse_pkg_name("alsa-plugins-1:1.2.7.1-2-x86_64.pkg.tar.zst");
  let exp = "alsa-plugins-1".to_string();
  assert_eq!(act, Some(exp))
}

#[test]
fn test_audacity() {
  let act = Pkg::parse_pkg_name("audacity-1:3.4.2-2-x86_64.pkg.tar.zst");
  let exp = "audacity-1".to_string();
  assert_eq!(act, Some(exp))
}

#[test]
fn test_aribb() {
  let act = Pkg::parse_pkg_name("aribb24-1.0.3-3-x86_64.pkg.tar.zst");
  let exp = "aribb24".to_string();
  assert_eq!(act, Some(exp))
}

#[test]
fn test_acl() {
  let act = Pkg::parse_pkg_name("acl-2.3.2-1-x86_64.pkg.tar.zst");
  let exp = "acl".to_string();
  assert_eq!(act, Some(exp))
}

#[test]
fn test_adobe_source_code_pro_fonts() {
  let act = Pkg::parse_pkg_name("adobe-source-code-pro-fonts-2.042u+1.062i+1.026vf-1-any.pkg.tar.zst");
  let exp = "adobe-source-code-pro-fonts".to_string();
  assert_eq!(act, Some(exp))
} 

#[test]
fn test_archlinux_keyring() {
  let act = Pkg::parse_pkg_name("archlinux-keyring-20240208-1-any.pkg.tar.zst");
  let exp = "archlinux-keyring".to_string();
  assert_eq!(act, Some(exp))
}

#[test]
fn test_awahi() {
  let act = Pkg::parse_pkg_name("avahi-0.8+22+gfd482a7-4-x86_64.pkg.tar.zst");
  let exp = "avahi".to_string();
  assert_eq!(act, Some(exp))
}

#[test]
fn test_emacs() {
  let act = Pkg::parse_pkg_name("emacs-29.2-1-x86_64.pkg.tar.zst");
  let exp = "emacs".to_string();
  assert_eq!(act, Some(exp))
}
