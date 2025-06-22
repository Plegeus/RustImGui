
use std::fmt::Debug;
use std::time::*;

pub trait Show {
  
  unsafe fn show(&self, id: &mut i32) {  }
  unsafe fn show_mut(&mut self, id: &mut i32) { 
    self.show(id);
  }

  //unsafe fn show_labeled<T: Debug>(&self, label: T) { panic!(); }
  //unsafe fn show_labeled_mut<T: Debug>(&self, label: T) { panic!(); }

}

pub unsafe fn push_unique_id() {
  // maybe just use a static int...?
  let now = SystemTime::now();
  let dur = now.duration_since(UNIX_EPOCH).unwrap();
  crate::push_id(dur.as_micros() as i32);
}

macro_rules! _impl_show_int {
  ($T:ty) => {
    impl Show for $T {
      unsafe fn show(&self, _: &mut i32) {
        crate::text(self);
      }
      unsafe fn show_mut(&mut self, id: &mut i32) {
        assert!(*self as i32 <= i32::MAX);
        let mut int: i32 = (*self).into();
        crate::push_id(*id);
        *id += 1;
        crate::input_i32("", &mut int, None, None, None);
        crate::pop_id();
        int = std::cmp::min(int, <$T>::MAX as i32);
        int = std::cmp::max(<$T>::MIN as i32, int);
        let int = int as $T;
        *self = int;
      }
    }
  }
}
macro_rules! _impl_show_any {
  ($T:ty) => {
    impl Show for $T {
      unsafe fn show(&self, _: &mut i32) {
        crate::text(self);
      }
      unsafe fn show_mut(&mut self, id: &mut i32) {
        self.show(id);
      }
    }
  }
}

_impl_show_int!(i8);
_impl_show_int!(i16);
_impl_show_int!(i32);
_impl_show_any!(i64);
_impl_show_any!(i128);
_impl_show_any!(isize);
_impl_show_int!(u8);
_impl_show_int!(u16);
_impl_show_any!(u32);
_impl_show_any!(u64);
_impl_show_any!(u128);
_impl_show_any!(usize);
impl Show for f32 {
  unsafe fn show(&self, _: &mut i32) {
    crate::text(self);
  }
  unsafe fn show_mut(&mut self, id: &mut i32) {
    crate::push_id(*id);
    *id += 1;
    crate::input_f32("", self, None, None, None, None);
    crate::pop_id();
  }
}
impl Show for f64 {
  unsafe fn show(&self, _: &mut i32) {
    crate::text(self);
  }
  unsafe fn show_mut(&mut self, id: &mut i32) {
    let mut flt: f32 = self.min(f32::MAX as f64) as f32;
    flt = flt.max(f32::MIN);
    flt.show_mut(id);
    *self = flt as f64;
  }
}
_impl_show_any!(char);
impl Show for bool {
  unsafe fn show(&self, _: &mut i32) {
    crate::text_colored(self, self);
  }
  unsafe fn show_mut(&mut self, id: &mut i32) {
    crate::push_id(*id);
    *id += 1;
    crate::checkbox("", self);
    crate::pop_id();
  }
}
_impl_show_any!(());
_impl_show_any!(std::time::SystemTime);
_impl_show_any!(std::time::Duration);
impl Show for [f32; 2] {
  unsafe fn show(&self, _: &mut i32) {
    crate::text(self);
  }
  unsafe fn show_mut(&mut self, id: &mut i32) {
    crate::push_id(*id);
    *id += 1;
    crate::push_item_width(-1.0);
    crate::input_float_2("##", self, None, None);
    crate::pop_item_width();
    crate::pop_id();
  }
}
impl Show for [f32; 3] {
  unsafe fn show(&self, _: &mut i32) {
    crate::text(self);
  }
  unsafe fn show_mut(&mut self, id: &mut i32) {
    crate::push_id(*id);
    *id += 1;
    crate::push_item_width(-1.0);
    crate::input_float_3("##", self, None, None);
    crate::pop_item_width();
    crate::pop_id();
  }
}
impl Show for [f32; 4] {
  unsafe fn show(&self, _: &mut i32) {
    crate::text(self);
  }
  unsafe fn show_mut(&mut self, id: &mut i32) {
    crate::push_id(*id);
    *id += 1;
    crate::push_item_width(-1.0);
    crate::input_float_4("##", self, None, None);
    crate::pop_item_width();
    crate::pop_id();
  }
}


impl<T, U> Show for (T, U) 
where
  T: Show,
  U: Show
{
  unsafe fn show(&self, id: &mut i32) {
    let (t, u) = self else { panic!() };
    t.show(id);
    u.show(id);
  }
  unsafe fn show_mut(&mut self, id: &mut i32) {
    let (t, u) = self else { panic!() };
    t.show_mut(id);
    u.show_mut(id);
  }
}


