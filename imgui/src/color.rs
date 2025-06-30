

#![allow(dead_code)]

use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Copy)]
pub struct Color([f32; 4]);
impl Color {

  pub const White: Color = Color([1.0; 4]);
  pub const Gray: Color = Color([0.66, 0.66, 0.66, 1.0]);
  pub const DarkGray: Color = Color([0.5, 0.5, 0.5, 1.0]);
  pub const LightGray: Color = Color([0.83, 0.83, 0.83, 1.0]);
  pub const Black: Color = Color([0.0, 0.0, 0.0, 1.0]);
  pub const Red: Color = Color([245.0 / 255.0, 39.0 / 255.0, 39.0 / 255.0, 1.0]);
  pub const DarkRed: Color = Color([145.0 / 255.0, 20.0 / 255.0, 20.0 / 255.0, 1.0]);
  pub const LightRed: Color = Color([230.0 / 255.0, 108.0 / 255.0, 108.0 / 255.0, 1.0]);
  pub const Green: Color = Color([0.0, 1.0, 0.0, 1.0]);
  pub const Blue: Color = Color([0.0, 0.0, 1.0, 1.0]);
  pub const LightBlue: Color = Color([128.0 / 255.0, 210.0 / 255.0, 1.0, 1.0]);

  fn set_r(&mut self, r: f32) {
    self[0] = r;
  }
  fn sub_r(&self, r: f32) -> Self {
    let mut c = *self;
    c.set_r(r);
    c
  }
  fn set_g(&mut self, g: f32) {
    self[1] = g;
  }
  fn sub_g(&self, g: f32) -> Self {
    let mut c = *self;
    c.set_g(g);
    c
  }
  fn set_b(&mut self, b: f32) {
    self[2] = b;
  }
  fn sub_b(&self, b: f32) -> Self {
    let mut c = *self;
    c.set_b(b);
    c
  }
  fn set_a(&mut self, a: f32) {
    self[3] = a;
  }
  fn sub_a(&self, a: f32) -> Self {
    let mut c = *self;
    c.set_a(a);
    c
  }


}

impl Deref for Color {
  type Target = [f32; 4];
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl DerefMut for Color {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
impl From<[f32; 4]> for Color {
  fn from(value: [f32; 4]) -> Self {
    Color(value)
  }
}
impl From<[f32; 3]> for Color {
  fn from(value: [f32; 3]) -> Self {
    Color([value[0], value[1], value[2], 1.0])
  }
}
impl From<bool> for Color {
  fn from(value: bool) -> Self {
    if value { 
      Color::Green 
    } else {
      Color::Red
    }
  }
}





