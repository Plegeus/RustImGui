
#![allow(warnings)]

pub mod flags;
//pub mod show;
pub mod color;

pub use color::*;
//pub use show::*;
pub use flags::*;
pub use imgui_derive::Show;

use std::ffi::{ CString, CStr };
use std::os::raw::c_int as Int;
use std::os::raw::c_char as Char;
use std::os::raw::c_void as Void;
use std::fmt::{ Display };

include!("./bindings/imgui_c.rs");

// interface...
pub unsafe fn show_demo_window(open: Option<&mut bool>) {
  if let Some(open) = open {
    let mut int = if *open { 1 } else { 0 };
    _show_demo_window(&mut int);
    *open = int != 0;
  } else {
    _show_demo_window(std::ptr::null_mut());
  }
}
pub unsafe fn show_style_editor() {
  _show_style_editor();
}

pub unsafe fn get_mouse_pos() -> (f32, f32) {
  let mut x = 0.0;
  let mut y = 0.0;
  _get_mouse_pos(&mut x, &mut y);
  (x, y)
}
pub unsafe fn is_window_hovered(flags: Option<i32>) -> bool {
  _is_window_hovered(flags.unwrap_or(0)) != 0
}
pub unsafe fn is_window_focused(flags: Option<i32>) -> bool {
  _is_window_focused(flags.unwrap_or(0)) != 0
}
pub unsafe fn get_cursor_screen_pos() -> (f32, f32) {
  let mut x = 0.0;
  let mut y = 0.0;
  _get_cursor_screen_pos(&mut x, &mut y);
  (x, y)
}
pub unsafe fn set_cursor_screen_pos(x: f32, y: f32) {
  _set_cursor_screen_pos(x, y);
}
pub unsafe fn get_cursor_pos() -> (f32, f32) {
  let mut x = 0.0;
  let mut y = 0.0;
  _get_cursor_pos(&mut x, &mut y);
  (x, y)
}
pub unsafe fn set_cursor_pos(x: f32, y: f32) {
  _set_cursor_pos(x, y);
}

pub unsafe fn get_mouse_pos_in_window() -> (f32, f32) {

  let (window_x, window_y) = get_window_pos();
  let (w, h) = get_window_size();
  let (mouse_x, mouse_y) = get_mouse_pos();
  let (mouse_x, mouse_y) = (mouse_x - window_x, mouse_y - window_y);

  //(mouse_x - w / 2.0, -(mouse_y - h / 2.0))
  (mouse_x, mouse_y)
}

pub unsafe fn set_next_window_size(size: [f32; 2], cond: Option<ImGuiCond>) {
  _set_next_window_size(size[0], size[1], cond.unwrap_or(ImGuiCond::None) as i32)
}
pub unsafe fn get_window_size() -> (f32, f32) {
  let mut w = 0.0;
  let mut h = 0.0;
  _get_window_size(&mut w, &mut h);
  (w, h)
}
pub unsafe fn get_content_region_avail() -> (f32, f32) {
  let mut w = 0.0;
  let mut h = 0.0;
  _get_content_region_avail(&mut w, &mut h);
  (w, h)
}
pub unsafe fn set_next_window_pos(pos: [f32; 2], cond: Option<ImGuiCond>, pivot: Option<[f32; 2]>) {
  _set_next_window_pos(pos[0], pos[1], cond.unwrap_or(ImGuiCond::None) as i32, pivot.unwrap_or([0.0, 0.0])[0], pivot.unwrap_or([0.0, 0.0])[1])
}
pub unsafe fn get_window_pos() -> (f32, f32) {
  let mut x = 0.0;
  let mut y = 0.0;
  _get_window_pos(&mut x, &mut y);
  (x, y)
}

pub unsafe fn begin<T: Display>(title: T, open: Option<&mut bool>, flags: Option<i32>) -> bool {
  let title = stringify(title);
  let c_title = title.cify();
  if let Some(open) = open {
    let mut int = if *open { 1 } else { 0 };
    let res = _begin(c_title.as_ptr(), &mut int, flags.cify());
    *open = int != 0;
    res != 0
  } else {
    _begin(c_title.as_ptr(), std::ptr::null_mut(), flags.cify()) != 0
  }
}
pub unsafe fn begin_child<T: Display>(id: T, size: Option<[f32; 2]>, child_flags: Option<i32>, window_flags: Option<i32>) -> bool {
  let id = stringify(id);
  let c_id = id.cify();
  let size = size.unwrap_or([0.0, 0.0]);
  _begin_child(c_id.as_ptr(), size[0], size[1], child_flags.unwrap_or(0), window_flags.unwrap_or(0)) == 1
}
pub unsafe fn open_popup<T: Display>(id: T, flags: Option<i32>) {
  let id = stringify(id);
  let c_id = id.cify();
  _open_popup(c_id.as_ptr(), flags.unwrap_or(0));
}
pub unsafe fn begin_popup<T: Display>(id: T, flags: Option<i32>) -> bool {
  let id = stringify(id);
  let c_id = id.cify();
  _begin_popup(c_id.as_ptr(), flags.unwrap_or(0)) != 0
}
pub unsafe fn begin_popup_modal<T: Display>(name: T, open: Option<&mut bool>, flags: Option<i32>) -> bool {
  let name = stringify(name);
  let c_name = name.cify();
    if let Some(open) = open {
    let mut int = if *open { 1 } else { 0 };
    let res = _begin_popup_modal(c_name.as_ptr(), &mut int, flags.unwrap_or(0));
    *open = int != 0;
    res != 0
  } else {
    _begin_popup_modal(c_name.as_ptr(), std::ptr::null_mut(), flags.unwrap_or(0))  != 0
  }
}
pub unsafe fn collapsing_header<T: Display>(label: T) -> bool {
  let label = stringify(label);
  let c_label = label.cify();
  _collapsing_header(c_label.as_ptr()) != 0
}
pub unsafe fn tree_node<T: Display>(label: T) -> bool {
  let label = stringify(label);
  let c_label = label.cify();
  _tree_node(c_label.as_ptr()) != 0
}
pub unsafe fn selectable<T: Display>(label: T, selected: bool, flags: Option<i32>, size: Option<[f32; 2]>) -> bool {
  let label = stringify(label);
  let c_label = label.cify();
  let size = size.unwrap_or([0.0; 2]);
  _selectable(c_label.as_ptr(), selected.cify(), flags.unwrap_or(0), size[0], size[1]) != 0
}

pub unsafe fn begin_tab_bar<T: Display>(label: T, flags: Option<i32>) -> bool {
  let label = stringify(label);
  let c_label = label.cify();
  _begin_tab_bar(c_label.as_ptr(), flags.unwrap_or(0)) != 0
}
pub unsafe fn begin_tab_item<T: Display>(label: T, open: Option<&mut bool>, flags: Option<i32>) -> bool {
  let label = stringify(label);
  let c_label = label.cify();
  if open.is_some() {
    with_bool(
      open,
      |int| _begin_tab_item(c_label.as_ptr(), int, flags.unwrap_or(0))
    )
  } else {
    _begin_tab_item(c_label.as_ptr(), std::ptr::null_mut(), flags.unwrap_or(0)) != 0
  }
}
pub unsafe fn tab_item_button<T: Display>(label: T, flags: Option<i32>) -> bool {
  let label = stringify(label);
  let c_label = label.cify();
  _tab_item_button(c_label.as_ptr(), flags.unwrap_or(0)) != 0
}

pub unsafe fn columns<T: Display>(count: i32, id: Option<T>, border: Option<bool>) {
  _columns(
    count, 
    id
      .map(|id| stringify(id).cify().as_ptr())
      .unwrap_or(std::ptr::null_mut()), 
    border.unwrap_or(true).cify()
  );
}
pub unsafe fn table_setup_column<T: Display>(label: T, flags: Option<i32>, width_or_weight: Option<f32>, user_id: Option<u32>) {
  let label = stringify(label);
  let c_label = label.cify();
  _table_setup_column(
    c_label.as_ptr(),
    flags.cify(),
    width_or_weight.unwrap_or(0.0),
    user_id.unwrap_or(0)
  );
}
pub unsafe fn begin_table<T: Display>(id: T, columns: i32, flags: Option<i32>) -> bool {
  let id = stringify(id);
  let c_id = id.cify();
  _begin_table(
    c_id.as_ptr(), 
    columns,
    flags.cify()
  ) != 0
}
pub unsafe fn table_set_column_index(column: i32) -> bool {
  _table_set_column_index(column) != 0
}
pub unsafe fn table_next_row(flags: Option<i32>, min_row_height: Option<f32>) {
  _table_next_row(flags.unwrap_or(ImGuiTableRowFlags_None), min_row_height.unwrap_or(0.0));
}
pub unsafe fn table_next_column() -> bool {
  _table_next_column() != 0
}


pub unsafe fn text<T: Display>(text: T) {
  let c_text = stringify(text);
  let c_text = c_text.cify();
  _text(c_text.as_ptr());
}
pub unsafe fn text_wrapped<T: Display>(text: T) {
  let c_text = stringify(text);
  let c_text = c_text.cify();
  _text_wrapped(c_text.as_ptr());
}
pub unsafe fn input_text<T: Display>(label: T, string: &mut String, flags: Option<i32>, callback: Option<fn()>, user_data: Option<*const ()>) -> bool {
  
  assert!(callback.is_none());
  assert!(user_data.is_none());

  // allocated a zero terminated buffer...
  const CAPACITY: usize = 256;
  let mut buf: [u8; CAPACITY] = [65; CAPACITY]; // all 'a'...
  string.push('\0');
  let bytes = string.as_bytes();
  for i in 0..bytes.len() {
    buf[i] = bytes[i];
  }

  let label = stringify(label);
  let c_label = label.cify();

  let res = _input_text(c_label.as_ptr(), buf.as_mut_ptr().cast(), CAPACITY as i32, flags.cify()) != 0;
  
  let cstr = CStr::from_bytes_until_nul(&buf).unwrap();
  *string = String::from(cstr.to_str().unwrap());

  res
}
pub unsafe fn input_text_multi_line<T: Display>(label: T, string: &mut String, size: Option<[f32; 2]>, flags: Option<i32>, callback: Option<fn()>, user_data: Option<*const ()>) -> bool {
  
  assert!(callback.is_none());
  assert!(user_data.is_none());

  // allocated a zero terminated buffer...
  const CAPACITY: usize = 1024;
  let mut buf: [u8; CAPACITY] = [65; CAPACITY]; // all 'a'...
  string.push('\0');
  let bytes = string.as_bytes();
  for i in 0..bytes.len() {
    buf[i] = bytes[i];
  }

  let label = stringify(label);
  let c_label = label.cify();
  let size = size.unwrap_or([0.0; 2]);

  let res = _input_text_multi_line(c_label.as_ptr(), buf.as_mut_ptr().cast(), CAPACITY as i32, size[0], size[1], flags.cify(), std::ptr::null_mut(), std::ptr::null_mut()) != 0;
  
  let cstr = CStr::from_bytes_until_nul(&buf).unwrap();
  *string = String::from(cstr.to_str().unwrap());

  res
}
pub unsafe fn text_colored<T, C>(color: C, text: T) 
where
  T: Display, 
  C: Into<Color>,
{
  let c_text = stringify(text);
  let c_text = c_text.cify();
  let color = color.into();
  _text_colored(color[0], color[1], color[2], color[3], c_text.as_ptr());
}

pub unsafe fn image(texture_id: *mut Void, img_size: &mut [f32; 2], uv0: Option<&mut [f32; 2]>, uv1: Option<&mut [f32; 2]>, tint_col: Option<&mut [f32; 4]>, border_col: Option<&mut [f32; 4]>) {
  _image(
    texture_id, 
    img_size.as_mut_ptr(),
    uv0.unwrap_or(&mut [0.0, 0.0]).as_mut_ptr(),
    uv1.unwrap_or(&mut [1.0, 1.0]).as_mut_ptr(),
    tint_col.unwrap_or(&mut [1.0, 1.0, 1.0, 1.0]).as_mut_ptr(),
    border_col.unwrap_or(&mut [0.0, 0.0, 0.0, 0.0]).as_mut_ptr(),
  );
}

pub unsafe fn begin_main_menu_bar() -> bool {
  _begin_main_menu_bar() == 1
}
pub unsafe fn begin_menu_bar() -> bool {
  _begin_menu_bar() != 0
}

pub unsafe fn begin_menu<T: Display>(label: T, enabled: Option<bool>) -> bool {
  let label = stringify(label);
  let c_label = label.cify();
  let enabled = if enabled.unwrap_or(true) { 1 } else { 0 };
  _begin_menu(c_label.as_ptr(), enabled) == 1
}
pub unsafe fn menu_item<T: Display>(label: T, shortcut: Option<&str>, selected: Option<&mut bool>, enabled: Option<bool>) -> bool {
  let label = stringify(label);
  let c_label = label.cify();
  let shortcut = if let Some(shortcut) = shortcut {
    let c_shortcut = shortcut.cify();
    c_shortcut.as_ptr()
  } else {
    std::ptr::null_mut()
  };
  let enabled = if let Some(enabled) = enabled {
    if enabled { 1 } else { 0 }
  } else {
    1
  };
  if let Some(selected) = selected {
    let mut int = if *selected { 1 } else { 0 };
    let res = _menu_item(c_label.as_ptr(), shortcut, &mut int, enabled);
    *selected = int != 0;
    res != 0
  } else {
    _menu_item(c_label.as_ptr(), shortcut, &mut 0, enabled) != 0
  }
}

pub unsafe fn separator_text<T: Display>(text: T) {
  let text = stringify(text);
  let c_text = text.cify();
  _separator_text(c_text.as_ptr());
}
pub unsafe fn same_line(offset: Option<f32>, spacing: Option<f32>) {
  _same_line(offset.unwrap_or(0.0), spacing.unwrap_or(-1.0));
}

pub unsafe fn button<T: Display>(label: T, size: Option<[f32; 2]>) -> bool {
  let label = stringify(label);
  let c_label = label.cify();
  let size = size.unwrap_or([0.0; 2]);
  _button(c_label.as_ptr(), size[0], size[1]) == 1
}

enum RadioButton<'a> {
  Bool(bool),
  Int(&'a mut i32, i32)
}
pub unsafe fn radio_button<T, U>(label: T, radio_button: U) -> bool
where 
  T: Display, 
  U: for<'a> Into<RadioButton<'a>>,
{
  let label = stringify(label);
  let c_label = label.cify();
  match radio_button.into() {
    RadioButton::Bool(boo) => {
      let mut i = boo.cify();
      _radio_button(c_label.as_ptr(), &mut i, -1) != 0
    },
    RadioButton::Int(value, v_button) => {
      _radio_button(c_label.as_ptr(), value, v_button) != 0
    },
  }
}
pub unsafe fn small_button<T: Display>(label: T) -> bool {
  let label = stringify(label);
  let c_label = label.cify();
  _small_button(c_label.as_ptr()) != 0
}

impl<'a> Into<RadioButton<'a>> for bool {
  fn into(self) -> RadioButton<'a> {
    RadioButton::Bool(self)
  }
}
impl Into<RadioButton<'_>> for (&'static mut i32, i32)
{
  fn into(self) -> RadioButton<'static> {
    RadioButton::Int(self.0, self.1)
  }
}


pub unsafe fn list_box<T, I>(label: T, current: &mut i32, items: &Vec<I>) -> bool 
where
  T: Display, 
  I: Display
{
  let label = stringify(label);
  let c_label = label.cify();
  let items: Vec<_> = items
    .iter()
    .map(|item| stringify(item))
    .collect();
  let c_items = items.iter().map(|item| item.cify()).collect::<Vec<_>>();
  let c_item_ptrs = c_items.iter().map(|item| item.as_ptr()).collect::<Vec<_>>();
  _list_box(c_label.as_ptr(), current, c_item_ptrs.as_ptr(), items.len() as i32) != 0
}

pub unsafe fn combo<T, I>(label: T, current: &mut usize, items: &Vec<I>, max_height: Option<i32>) -> bool 
where
  T: Display, 
  I: Display
{
  let label = stringify(label);
  let c_label = label.cify();
  let items: Vec<_> = items
    .iter()
    .map(|item| stringify(item))
    .collect();
  let c_items = items.iter().map(|item| item.cify()).collect::<Vec<_>>();
  let c_item_ptrs = c_items.iter().map(|item| item.as_ptr()).collect::<Vec<_>>();
  let mut current2 = *current as i32;
  let res = _combo(
    c_label.as_ptr(), 
    &mut current2, 
    c_item_ptrs.as_ptr(), 
    items.len() as i32, 
    max_height.unwrap_or(-1)
  ) != 0;
  *current = current2 as usize;
  res
}

pub unsafe fn push_style_var(idx: ImGuiStyleVar, x: f32, y: Option<f32>) {
  _push_style_var(idx as i32, x, y.unwrap_or(0.0));
}
pub unsafe fn pop_style_var(count: Option<i32>) {
  _pop_style_var(count.unwrap_or(0));
}
pub unsafe fn get_style_var(idx: ImGuiStyleVar, x: &mut f32, y: Option<&mut f32>) {
  let mut tmp = 0.0;
  _get_style_var(idx as i32, x, &mut tmp);
  if let Some(p_y) = y {
    *p_y = tmp;
  }
}

pub unsafe fn push_style_color<C: Into<Color>>(idx: ImGuiCol, color: C) {
  let mut color = color.into();
  _push_style_color(idx as i32, color.as_mut_ptr());
}

pub unsafe fn is_item_clicked(mouse_button: Option<i32>) -> bool {
  _is_item_clicked(mouse_button.unwrap_or(0)) != 0
}
pub unsafe fn is_item_hovered(flags: Option<i32>) -> bool {
  _is_item_hovered(flags.unwrap_or(0)) != 0
}


pub unsafe fn input_float_2<T: Display>(label: T, value: &mut [f32; 2], format: Option<&str>, flags: Option<i32>) -> bool {
  let label = stringify(label);
  let c_label = label.cify();
  let c_format = format.unwrap_or("%.3f").cify();
  _input_float_2(
    c_label.as_ptr(),
    value as *mut f32,
    c_format.as_ptr(),
    flags.unwrap_or(0)
  ) != 0
}
pub unsafe fn input_float_3<T: Display>(label: T, value: &mut [f32; 3], format: Option<&str>, flags: Option<i32>) -> bool {
  let label = stringify(label);
  let c_label = label.cify();
  let c_format = format.unwrap_or("%.3f").cify();
  _input_float_3(
    c_label.as_ptr(),
    value as *mut f32,
    c_format.as_ptr(),
    flags.unwrap_or(0)
  ) != 0
}
pub unsafe fn input_float_4<T: Display>(label: T, value: &mut [f32; 4], format: Option<&str>, flags: Option<i32>) -> bool {
  let label = stringify(label);
  let c_label = label.cify();
  let c_format = format.unwrap_or("%.3f").cify();
  _input_float_4(
    c_label.as_ptr(),
    value as *mut f32,
    c_format.as_ptr(),
    flags.unwrap_or(0)
  ) != 0
}

pub unsafe fn input_f32<T: Display>(label: T, value: &mut f32, step: Option<f32>, step_fast: Option<f32>, format: Option<&str>, flags: Option<i32>) -> bool {
  let label = stringify(label);
  let c_label = label.cify();
  let c_format = format.unwrap_or("%.3f").cify();
  _input_float(
    c_label.as_ptr(),
    value, 
    step.unwrap_or(0.0),
    step_fast.unwrap_or(0.0),
    c_format.as_ptr(),
    flags.unwrap_or(0)
  ) != 0
}
pub unsafe fn input_f64<T: Display>(label: T, value: &mut f64, step: Option<f64>, step_fast: Option<f64>, format: Option<&str>, flags: Option<i32>) -> bool {
  let label = stringify(label);
  let c_label = label.cify();
  let c_format = format.unwrap_or("%.3f").cify();
  _input_double(
    c_label.as_ptr(),
    value, 
    step.unwrap_or(0.0),
    step_fast.unwrap_or(0.0),
    c_format.as_ptr(),
    flags.unwrap_or(0)
  ) != 0
}
pub unsafe fn input_i32<T: Display>(label: T, value: &mut i32, step: Option<i32>, step_fast: Option<i32>, flags: Option<i32>) -> bool {
  let label = stringify(label);
  let c_label = label.cify();
  _input_int(
    c_label.as_ptr(),
    value, 
    step.unwrap_or(1),
    step_fast.unwrap_or(100),
    flags.unwrap_or(0),
  ) != 0
}
pub unsafe fn checkbox<T: Display>(label: T, value: &mut bool) -> bool {
  let label = stringify(label);
  let c_label = label.cify();
  let mut i = if *value { 1 } else { 0 };
  let res = _checkbox(c_label.as_ptr(), &mut i) != 0;
  *value = i != 0;
  res
}


// helpers...
trait Cify<T> {
  fn cify(&self) -> T;
}

impl Cify<Int> for bool {
  fn cify(&self) -> Int {
    if *self { 1 } else { 0 }
  }
}
impl Cify<CString> for str {
  fn cify(&self) -> CString {
    CString::new(self).unwrap()
  }
}
impl Cify<i32> for Option<i32> {
  fn cify(&self) -> i32 {
    if let Some(i) = *self { i } else { 0 }
  }
}


unsafe fn with_bool(b: Option<&mut bool>, f: impl Fn(&mut Int) -> Int) -> bool {
  if let Some(b) = b {
    let mut int = if *b { 1 } else { 0 };
    let res = f(&mut int);
    *b = int == 1;
    res != 0
  } else {
    let mut int = i32::MAX;
    f(&mut int) != 0
  } 
}
unsafe fn with_string(s: Option<&str>, f: impl Fn(*const Char)) {
  if let Some(s) = s {
    let c_s = s.cify();
    f(c_s.as_ptr());
  } else {
    f(std::ptr::null_mut());
  }
}


fn stringify<T: Display>(d: T) -> String {
  format!("{d}").replace("\"", "").replace("\\", "")
}



