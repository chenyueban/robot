#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[napi]
#[allow(dead_code)]
#[cfg(target_os = "windows")]
pub fn copy() {
  use enigo::*;
  let mut enigo = Enigo::new();
  enigo.key_up(Key::Control);
  enigo.key_up(Key::Alt);
  enigo.key_up(Key::Shift);
  enigo.key_up(Key::Space);
  enigo.key_down(Key::Control);
  enigo.key_click(Key::Layout('c'));
  enigo.key_up(Key::Control);
}
#[napi]
#[allow(dead_code)]
#[cfg(target_os = "macos")]
pub fn copy() {
  use enigo::*;
  let mut enigo = Enigo::new();
  enigo.key_up(Key::Control);
  enigo.key_up(Key::Meta);
  enigo.key_up(Key::Alt);
  enigo.key_up(Key::Shift);
  enigo.key_up(Key::Space);
  enigo.key_up(Key::Tab);
  enigo.key_up(Key::Option);
  enigo.key_down(Key::Meta);
  enigo.key_click(Key::Layout('c'));
  enigo.key_up(Key::Meta);
}
#[napi]
#[allow(dead_code)]
#[cfg(target_os = "linux")]
pub fn copy() {
  use enigo::*;
  let mut enigo = Enigo::new();
  enigo.key_up(Key::Control);
  enigo.key_up(Key::Alt);
  enigo.key_up(Key::Shift);
  enigo.key_up(Key::Space);
  enigo.key_down(Key::Control);
  enigo.key_click(Key::Layout('c'));
  enigo.key_up(Key::Control);
}
