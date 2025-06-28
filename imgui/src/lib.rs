

pub mod calls;
pub use calls::*;

pub mod flags;
pub use flags::*;

pub mod color;
pub use color::*;


pub trait OptionMut<'a, T> {
  fn into_option(self) -> Option<&'a mut T>;
}
pub trait OptionRef<'a, T> {
  fn into_i32(self) -> i32 where T: GuiFlag;
}
pub trait OptionOwned<T> {
  fn into_option(self) -> Option<T>;
  fn into_i32(self) -> i32 
  where 
    T: GuiFlag + Default,
    Self: Sized,
  {
    self.into_option().unwrap_or(T::default()).as_i32()
  }
}

impl<'a, T: 'static, I> OptionMut<'a, T> for I
where 
  I: Into<Option<&'a mut T>>,
{
  fn into_option(self) -> Option<&'a mut T> {
    self.into()
  }
}

impl<'a, T> OptionRef<'a, T> for &'a Vec<T> {
  fn into_i32(self) -> i32 where T: GuiFlag {
    self.as_i32()
  }
}
impl<'a, T> OptionRef<'a, T> for Option<&'a Vec<T>> {
  fn into_i32(self) -> i32 where T: GuiFlag {
    let Some(vec) = self else { return T::default_i32() };
    <&Vec<T> as OptionRef<'a, T>>::into_i32(vec)
  }
}

impl<T> OptionOwned<T> for T {
  fn into_option(self) -> Option<T> {
    Some(self)
  }
}
impl<T> OptionOwned<T> for Option<T> {
  fn into_option(self) -> Option<T> {
    self
  }
}

impl<T: GuiFlag> GuiFlag for Vec<T> {
  fn as_i32(&self) -> i32 {
    self.iter()
      .fold(0, |l, r| l | r.as_i32())
  }
  fn default_i32() -> i32 {
    T::default_i32()
  }
}


/*
pub trait GuiOption<T> {  
  fn into(self) -> Option<T>;
}
impl<T: 'static> GuiOption<T> for Option<T> {
  fn into(self) -> Option<T> {
    self
  }
}
impl<T: 'static> GuiOption<T> for T {
  fn into(self) -> Option<T> {
    Some(self)
  }
}

// impl Trait<&T> is unstable, hence the use of 
// impl GuiFlags<*mut bool> for example,
// rather than impl GuiFlags<&mut bool>
impl<T> GuiOption<*mut T> for &mut T {
  fn into(self) -> Option<*mut T> {
    Some(self)
  }
}
//impl<T> GuiOption<*mut T> for Option<&mut T> {
//  fn into(self) -> Option<*mut T> {
//    self.map(|t| t as *mut T)
//  }
//}
impl GuiOption<*mut bool> for Option<&mut bool> {
  fn into(self) -> Option<*mut bool> {
    self.map(|b| b as *mut bool)
  }
}
impl<T, const N: usize> GuiOption<*const [T]> for &[T; N] {
  fn into(self) -> Option<*const [T]> {
    Some(self)
  }
}

*/

/*
pub(crate) trait IntoI32 {
  fn into_i32(self) -> i32;
}

impl<F: GuiFlag> IntoI32 for Option<*const [F]> {
  fn into_i32(self) -> i32 {
    let Some(ptr) = self else { return F::default().as_i32() };
    unsafe {
      (&*ptr)
        .into_iter()
        .fold(0, |l, r| l | r.as_i32())
    }
  }
}
*/







