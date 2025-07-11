
use std::ffi::{ CString, CStr };
use std::os::raw::c_int as Int;
use std::os::raw::c_void as Void;
use std::fmt::{ Display };

use crate::{Color, OptionMut, OptionOwned, OptionRef};
use crate::flags::*;


include!("./bindings/imgui_c.rs");
include!("./bindings/vulkan_info.rs");
//include!("./bindings/imgui_cpp.rs");


// interface...
pub unsafe fn show_demo_window<'a>(open: impl OptionMut<'a, bool>) {
  if let Some(open) = open.into_option() {
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
pub unsafe fn is_window_hovered(flags: impl OptionOwned<HoveredFlags>) -> bool {
  _is_window_hovered(flags.into_i32()) != 0
}
pub unsafe fn is_window_focused(flags: impl OptionOwned<FocusedFlags>) -> bool {
  _is_window_focused(flags.into_i32()) != 0
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
  //let (w, h) = get_window_size();
  let (mouse_x, mouse_y) = get_mouse_pos();
  let (mouse_x, mouse_y) = (mouse_x - window_x, mouse_y - window_y);

  //(mouse_x - w / 2.0, -(mouse_y - h / 2.0))
  (mouse_x, mouse_y)
}
pub unsafe fn set_next_window_size(size: [f32; 2], cond: impl OptionOwned<Cond>) {
  _set_next_window_size(
    size[0], 
    size[1], 
    cond.into_option().unwrap_or(Cond::Always) as i32
  );
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
pub unsafe fn set_next_window_pos(pos: [f32; 2], cond: impl OptionOwned<Cond>, pivot: impl OptionOwned<[f32; 2]>) {
  let pivot = pivot.into_option().unwrap_or([0.0, 0.0]);
  _set_next_window_pos(
    pos[0], 
    pos[1], 
    cond.into_option().unwrap_or(Cond::Always) as i32, 
    pivot[0], 
    pivot[1]
  );
}
pub unsafe fn get_window_pos() -> (f32, f32) {
  let mut x = 0.0;
  let mut y = 0.0;
  _get_window_pos(&mut x, &mut y);
  (x, y)
}


pub unsafe fn begin<'a, D: Display>(title: D, open: impl OptionMut<'a, bool>, flags: impl OptionRef<'a, WindowFlags>) -> bool {
  let title = stringify(title);
  let c_title = title.cify();
  let flags = flags.into_i32();
  if let Some(open) = open.into_option() {
    let mut int = if *open { 1 } else { 0 };
    let res = _begin(c_title.as_ptr(), &mut int, flags);
    *open = int != 0;
    res != 0
  } else {
    _begin(c_title.as_ptr(), std::ptr::null_mut(), flags) != 0
  }
}
pub unsafe fn begin_child<'a, D: Display>(id: D, size: impl OptionOwned<[f32; 2]>, child_flags: impl OptionRef<'a, ChildFlags>, window_flags: impl OptionRef<'a, WindowFlags>) -> bool {
  let id = stringify(id);
  let c_id = id.cify();
  let size = size.into_option().unwrap_or([0.0, 0.0]);
  _begin_child(
    c_id.as_ptr(), 
    size[0], 
    size[1], 
    child_flags.into_i32(),
    window_flags.into_i32(),
  ) != 0
}
pub unsafe fn open_popup<'a, D: Display>(id: D, flags: impl OptionRef<'a, PopupFlags>) {
  let id = stringify(id);
  let c_id = id.cify();
  _open_popup(c_id.as_ptr(), flags.into_i32());
}
pub unsafe fn begin_popup<'a, D: Display>(id: D, flags: impl OptionRef<'a, WindowFlags>) -> bool {
  let id = stringify(id);
  let c_id = id.cify();
  _begin_popup(
    c_id.as_ptr(), 
    flags.into_i32(),
  ) != 0
}
pub unsafe fn begin_popup_modal<'a, D: Display>(name: D, open: impl OptionMut<'a, bool>, flags: impl OptionRef<'a, WindowFlags>) -> bool {
  let name = stringify(name);
  let c_name = name.cify();
  let flags = flags.into_i32();
  if let Some(open) = open.into_option() {
    let mut int = if *open { 1 } else { 0 };
    let res = _begin_popup_modal(c_name.as_ptr(), &mut int, flags);
    *open = int != 0;
    res != 0
  } else {
    _begin_popup_modal(c_name.as_ptr(), std::ptr::null_mut(), flags)  != 0
  }
}
pub unsafe fn collapsing_header<D: Display>(label: D) -> bool {
  let label = stringify(label);
  let c_label = label.cify();
  _collapsing_header(c_label.as_ptr()) != 0
}
pub unsafe fn tree_node<D: Display>(label: D) -> bool {
  let label = stringify(label);
  let c_label = label.cify();
  _tree_node(c_label.as_ptr()) != 0
}
// add a GuiBool trait that allows to pass a &mut bool as well as a bool (see imgui.h)
/*pub unsafe fn selectable<D: Display>(label: D, selected: bool, flags: Option<i32>, size: Option<[f32; 2]>) -> bool {
  let label = stringify(label);
  let c_label = label.cify();
  let size = size.unwrap_or([0.0; 2]);
  _selectable(c_label.as_ptr(), selected.cify(), flags.unwrap_or(0), size[0], size[1]) != 0
}*/

pub unsafe fn begin_tab_bar<'a, D: Display>(label: D, flags: impl OptionRef<'a, TabBarFlags>) -> bool {
  let label = stringify(label);
  let c_label = label.cify();
  _begin_tab_bar(c_label.as_ptr(), flags.into_i32()) != 0
}
pub unsafe fn begin_tab_item<'a, D: Display>(label: D, open: impl OptionMut<'a, bool>, flags: impl OptionRef<'a, TabItemFlags>) -> bool {
  let label = stringify(label);
  let c_label = label.cify();
  let flags = flags.into_i32();
  if let Some(open) = open.into_option() {
    let mut int = if *open { 1 } else { 0 };
    let res = _begin_tab_item(c_label.as_ptr(), &mut int, flags) != 0;
    *open = int != 0;
    res
  } else {
    _begin_tab_item(c_label.as_ptr(), std::ptr::null_mut(), flags) != 0
  }
}
pub unsafe fn tab_item_button<'a, D: Display>(label: D, flags: impl OptionRef<'a, TabItemFlags>) -> bool {
  let label = stringify(label);
  let c_label = label.cify();
  _tab_item_button(c_label.as_ptr(), flags.into_i32()) != 0
}

pub unsafe fn columns<'a, D: Display>(count: i32, id: impl OptionOwned<D>, border: impl OptionOwned<bool>) {
  _columns(
    count, 
    id
      .into_option()
      .map(|id| stringify(id).cify().as_ptr())
      .unwrap_or(std::ptr::null_mut()), 
    if border.into_option().unwrap_or(true) { 1 } else { 0 }
  );
}
pub unsafe fn table_setup_column<'a, D: Display>(label: D, flags: impl OptionRef<'a, TableColumnFlags>, width_or_weight: impl OptionOwned<f32>, user_id: impl OptionOwned<u32>) {
  let label = stringify(label);
  let c_label = label.cify();
  _table_setup_column(
    c_label.as_ptr(),
    flags.into_i32(),
    width_or_weight.into_option().unwrap_or(0.0),
    user_id.into_option().unwrap_or(0)
  );
}
pub unsafe fn begin_table<'a, D: Display>(id: D, columns: i32, flags: impl OptionRef<'a, TableFlags>) -> bool {
  let id = stringify(id);
  let c_id = id.cify();
  _begin_table(
    c_id.as_ptr(), 
    columns,
    flags.into_i32(),
  ) != 0
}
pub unsafe fn table_set_column_index(column: i32) -> bool {
  _table_set_column_index(column) != 0
}
pub unsafe fn table_next_row<'a>(flags: impl OptionRef<'a, TableRowFlags>, min_row_height: impl OptionOwned<f32>) {
  _table_next_row(
    flags.into_i32(), 
    min_row_height.into_option().unwrap_or(0.0)
  );
}
pub unsafe fn table_next_column() -> bool {
  _table_next_column() != 0
}


pub unsafe fn text<D: Display>(text: D) {
  let c_text = stringify(text);
  let c_text = c_text.cify();
  _text(c_text.as_ptr());
}
pub unsafe fn text_wrapped<D: Display>(text: D) {
  let c_text = stringify(text);
  let c_text = c_text.cify();
  _text_wrapped(c_text.as_ptr());
}
pub unsafe fn input_text<'a, D: Display>(label: D, string: &mut String, flags: impl OptionRef<'a, InputTextFlags>, callback: impl OptionOwned<fn()>, user_data: impl OptionOwned<*const ()>) -> bool {
  
  assert!(callback.into_option().is_none());
  assert!(user_data.into_option().is_none());

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

  let res = _input_text(
    c_label.as_ptr(), 
    buf.as_mut_ptr().cast(), 
    CAPACITY as i32, 
    flags.into_i32(),
  ) != 0;
  
  let cstr = CStr::from_bytes_until_nul(&buf).unwrap();
  *string = String::from(cstr.to_str().unwrap());

  res
}
pub unsafe fn input_text_multiline<'a, D: Display>(label: D, string: &mut String, size: impl OptionOwned<[f32; 2]>, flags: impl OptionRef<'a, InputTextFlags>, callback: impl OptionOwned<fn()>, user_data: impl OptionOwned<*const ()>) -> bool {
  
  assert!(callback.into_option().is_none());
  assert!(user_data.into_option().is_none());

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
  let size = size.into_option().unwrap_or([0.0; 2]);

  let res = _input_text_multi_line(
    c_label.as_ptr(), 
    buf.as_mut_ptr().cast(), 
    CAPACITY as i32, 
    size[0], 
    size[1], 
    flags.into_i32(),
    std::ptr::null_mut(), 
    std::ptr::null_mut()
  ) != 0;
  
  let cstr = CStr::from_bytes_until_nul(&buf).unwrap();
  *string = String::from(cstr.to_str().unwrap());

  res
}
pub unsafe fn text_colored<D, C>(color: C, text: D) 
where
  D: Display, 
  C: Into<Color>,
{
  let c_text = stringify(text);
  let c_text = c_text.cify();
  let color = color.into();
  _text_colored(color[0], color[1], color[2], color[3], c_text.as_ptr());
}

pub unsafe fn image(texture_id: *const Void, img_size: [f32; 2], uv0: impl OptionOwned<[f32; 2]>, uv1: impl OptionOwned<[f32; 2]>, tint_col: impl OptionOwned<[f32; 4]>, border_col: impl OptionOwned<[f32; 4]>) {
  _image(
    texture_id, 
    img_size.as_ptr(),
    uv0.into_option().unwrap_or([0.0, 0.0]).as_ptr(),
    uv1.into_option().unwrap_or([1.0, 1.0]).as_ptr(),
    tint_col.into_option().unwrap_or([1.0, 1.0, 1.0, 1.0]).as_ptr(),
    border_col.into_option().unwrap_or([0.0, 0.0, 0.0, 0.0]).as_ptr(),
  );
}

pub unsafe fn begin_main_menu_bar() -> bool {
  _begin_main_menu_bar() == 1
}
pub unsafe fn begin_menu_bar() -> bool {
  _begin_menu_bar() != 0
}

pub unsafe fn begin_menu<D: Display>(label: D, enabled: impl OptionOwned<bool>) -> bool {
  let label = stringify(label);
  let c_label = label.cify();
  let enabled = if enabled.into_option().unwrap_or(true) { 1 } else { 0 };
  _begin_menu(c_label.as_ptr(), enabled) == 1
}
pub unsafe fn menu_item<'a, D: Display>(label: D, shortcut: impl OptionOwned<&'static str>, selected: impl OptionMut<'a, bool>, enabled: impl OptionOwned<bool>) -> bool {
  let label = stringify(label);
  let c_label = label.cify();
  let shortcut = shortcut.into_option().map_or(std::ptr::null(), |sh| sh.cify().as_ptr());
  let enabled = if enabled.into_option().unwrap_or(true) { 1 } else { 0 };
  if let Some(selected) = selected.into_option() {
    let mut int = if *selected { 1 } else { 0 };
    let res = _menu_item(c_label.as_ptr(), shortcut, &mut int, enabled);
    *selected = int != 0;
    res != 0
  } else {
    _menu_item(c_label.as_ptr(), shortcut, std::ptr::null_mut(), enabled) != 0
  }
}

pub unsafe fn separator_text<D: Display>(text: D) {
  let text = stringify(text);
  let c_text = text.cify();
  _separator_text(c_text.as_ptr());
}
pub unsafe fn same_line(offset: impl OptionOwned<f32>, spacing: impl OptionOwned<f32>) {
  _same_line(
    offset.into_option().unwrap_or(0.0), 
    spacing.into_option().unwrap_or(-1.0)
  );
}

pub unsafe fn button<D: Display>(label: D, size: impl OptionOwned<[f32; 2]>) -> bool {
  let label = stringify(label);
  let c_label = label.cify();
  let size = size.into_option().unwrap_or([0.0; 2]);
  _button(c_label.as_ptr(), size[0], size[1]) == 1
}

enum RadioButton<'a> {
  Bool(bool),
  Int(&'a mut i32, i32)
}
pub unsafe fn radio_button<D, U>(label: D, radio_button: U) -> bool
where 
  D: Display, 
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
pub unsafe fn small_button<D: Display>(label: D) -> bool {
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


pub unsafe fn list_box<D, I>(label: D, current: &mut usize, items: &Vec<I>) -> bool 
where
  D: Display, 
  I: Display
{
  let mut current_u = *current as i32;
  let label = stringify(label);
  let c_label = label.cify();
  let items: Vec<_> = items
    .iter()
    .map(|item| stringify(item))
    .collect();
  let c_items = items.iter().map(|item| item.cify()).collect::<Vec<_>>();
  let c_item_ptrs = c_items.iter().map(|item| item.as_ptr()).collect::<Vec<_>>();
  let res = _list_box(c_label.as_ptr(), &mut current_u, c_item_ptrs.as_ptr(), items.len() as i32) != 0;
  *current = current_u as usize;
  res
}

// !!! WARNING !!!
// overloaded methods exist in imgui.h, should their be equivalents in rust?
// !!! WARNING !!!
pub unsafe fn combo<D, I>(label: D, current: &mut usize, items: &Vec<I>, max_height: impl OptionOwned<i32>) -> bool 
where
  D: Display, 
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
    max_height.into_option().unwrap_or(-1),
  ) != 0;
  *current = current2 as usize;
  res
}

pub unsafe fn push_style_var(idx: StyleVar, x: f32, y: impl OptionOwned<f32>) {
  _push_style_var(idx as i32, x, y.into_option().unwrap_or(0.0));
}
pub unsafe fn pop_style_var(count: usize) {
  _pop_style_var(count as i32);
}
pub unsafe fn get_style_var<'a>(idx: StyleVar, x: &mut f32, y: impl OptionMut<'a, f32>) {
  let mut tmp = 0.0;
  _get_style_var(idx as i32, x, &mut tmp);
  if let Some(p_y) = y.into_option() {
    *p_y = tmp;
  }
}

pub unsafe fn push_style_color<C: Into<Color>>(idx: ColorVar, color: C) {
  let mut color = color.into();
  _push_style_color(idx as i32, color.as_mut_ptr());
}

pub unsafe fn is_item_clicked(mouse_button: impl OptionOwned<MouseButton>) -> bool {
  _is_item_clicked(mouse_button.into_option().unwrap_or(MouseButton::Left) as i32) != 0
}
pub unsafe fn is_item_hovered<'a>(flags: impl OptionRef<'a, WindowFlags>) -> bool {
  _is_item_hovered(flags.into_i32()) != 0
}


pub unsafe fn input_float_2<'a, D: Display>(label: D, value: &mut [f32; 2], format: impl OptionOwned<&'static str>, flags: impl OptionRef<'a, InputTextFlags>) -> bool {
  let label = stringify(label);
  let c_label = label.cify();
  let c_format = format.into_option().unwrap_or("%.3f").cify();
  _input_float_2(
    c_label.as_ptr(),
    value as *mut f32,
    c_format.as_ptr(),
    flags.into_i32(),
  ) != 0
}
pub unsafe fn input_float_3<'a, D: Display>(label: D, value: &mut [f32; 3], format: impl OptionOwned<&'static str>, flags: impl OptionRef<'a, InputTextFlags>) -> bool {
  let label = stringify(label);
  let c_label = label.cify();
  let c_format = format.into_option().unwrap_or("%.3f").cify();
  _input_float_3(
    c_label.as_ptr(),
    value as *mut f32,
    c_format.as_ptr(),
    flags.into_i32(),
  ) != 0
}
pub unsafe fn input_float_4<'a, D: Display>(label: D, value: &mut [f32; 4], format: impl OptionOwned<&'static str>, flags: impl OptionRef<'a, InputTextFlags>) -> bool {
  let label = stringify(label);
  let c_label = label.cify();
  let c_format = format.into_option().unwrap_or("%.3f").cify();
  _input_float_4(
    c_label.as_ptr(),
    value as *mut f32,
    c_format.as_ptr(),
    flags.into_i32(),
  ) != 0
}
pub unsafe fn slider_float_2<'a, D: Display>(label: D, value: &mut [f32; 2], min: f32, max: f32, format: impl OptionOwned<&'static str>, flags: impl OptionRef<'a, SliderFlags>) -> bool {
  let label = stringify(label);
  let c_label = label.cify();
  let c_format = format.into_option().unwrap_or("%.3f").cify();
  _slider_float_2(
    c_label.as_ptr(), 
    value as *mut f32, 
    min, 
    max, 
    c_format.as_ptr(), 
    flags.into_i32()
  ) != 0
}
pub unsafe fn slider_float_3<'a, D: Display>(label: D, value: &mut [f32; 3], min: f32, max: f32, format: impl OptionOwned<&'static str>, flags: impl OptionRef<'a, SliderFlags>) -> bool {
    let label = stringify(label);
  let c_label = label.cify();
  let c_format = format.into_option().unwrap_or("%.3f").cify();
  _slider_float_3(
    c_label.as_ptr(), 
    value as *mut f32, 
    min, 
    max, 
    c_format.as_ptr(), 
    flags.into_i32()
  ) != 0
}
pub unsafe fn slider_float_4<'a, D: Display>(label: D, value: &mut [f32; 4], min: f32, max: f32, format: impl OptionOwned<&'static str>, flags: impl OptionRef<'a, SliderFlags>) -> bool {
    let label = stringify(label);
  let c_label = label.cify();
  let c_format = format.into_option().unwrap_or("%.3f").cify();
  _slider_float_4(
    c_label.as_ptr(), 
    value as *mut f32, 
    min, 
    max, 
    c_format.as_ptr(), 
    flags.into_i32()
  ) != 0
}
pub unsafe fn drag_float_2<'a, D: Display>(label: D, value: &mut [f32; 2], speed: impl OptionOwned<f32>, min: impl OptionOwned<f32>, max: impl OptionOwned<f32>, format: impl OptionOwned<&'static str>, flags: impl OptionRef<'a, SliderFlags>) -> bool {
  let label = stringify(label);
  let c_label = label.cify();
  let c_format = format.into_option().unwrap_or("%.3f").cify();
  _drag_float_2(
    c_label.as_ptr(), 
    value as *mut f32, 
    speed.into_option().unwrap_or(1.0),
    min.into_option().unwrap_or(0.0),
    max.into_option().unwrap_or(0.0),
    c_format.as_ptr(), 
    flags.into_i32()
  ) != 0
}
pub unsafe fn drag_float_3<'a, D: Display>(label: D, value: &mut [f32; 3], speed: impl OptionOwned<f32>, min: impl OptionOwned<f32>, max: impl OptionOwned<f32>, format: impl OptionOwned<&'static str>, flags: impl OptionRef<'a, SliderFlags>) -> bool {
    let label = stringify(label);
  let c_label = label.cify();
  let c_format = format.into_option().unwrap_or("%.3f").cify();
  _drag_float_3(
    c_label.as_ptr(), 
    value as *mut f32, 
    speed.into_option().unwrap_or(1.0),
    min.into_option().unwrap_or(0.0),
    max.into_option().unwrap_or(0.0),
    c_format.as_ptr(), 
    flags.into_i32()
  ) != 0
}
pub unsafe fn drag_float_4<'a, D: Display>(label: D, value: &mut [f32; 4], speed: impl OptionOwned<f32>, min: impl OptionOwned<f32>, max: impl OptionOwned<f32>, format: impl OptionOwned<&'static str>, flags: impl OptionRef<'a, SliderFlags>) -> bool {
    let label = stringify(label);
  let c_label = label.cify();
  let c_format = format.into_option().unwrap_or("%.3f").cify();
  _drag_float_4(
    c_label.as_ptr(), 
    value as *mut f32, 
    speed.into_option().unwrap_or(1.0),
    min.into_option().unwrap_or(0.0),
    max.into_option().unwrap_or(0.0),
    c_format.as_ptr(), 
    flags.into_i32()
  ) != 0
}
pub unsafe fn drag_int_2<'a, D: Display>(label: D, value: &mut [i32; 2], speed: impl OptionOwned<f32>, min: impl OptionOwned<i32>, max: impl OptionOwned<i32>, format: impl OptionOwned<&'static str>, flags: impl OptionRef<'a, SliderFlags>) -> bool {
  let label = stringify(label);
  let c_label = label.cify();
  let c_format = format.into_option().unwrap_or("%d").cify();
  _drag_int_2(
    c_label.as_ptr(), 
    value as *mut i32, 
    speed.into_option().unwrap_or(1.0),
    min.into_option().unwrap_or(0),
    max.into_option().unwrap_or(0),
    c_format.as_ptr(), 
    flags.into_i32()
  ) != 0
}
pub unsafe fn drag_int_3<'a, D: Display>(label: D, value: &mut [i32; 3], speed: impl OptionOwned<f32>, min: impl OptionOwned<i32>, max: impl OptionOwned<i32>, format: impl OptionOwned<&'static str>, flags: impl OptionRef<'a, SliderFlags>) -> bool {
    let label = stringify(label);
  let c_label = label.cify();
  let c_format = format.into_option().unwrap_or("%d").cify();
  _drag_int_3(
    c_label.as_ptr(), 
    value as *mut i32, 
    speed.into_option().unwrap_or(1.0),
    min.into_option().unwrap_or(0),
    max.into_option().unwrap_or(0),
    c_format.as_ptr(), 
    flags.into_i32()
  ) != 0
}
pub unsafe fn drag_int_4<'a, D: Display>(label: D, value: &mut [i32; 4], speed: impl OptionOwned<f32>, min: impl OptionOwned<i32>, max: impl OptionOwned<i32>, format: impl OptionOwned<&'static str>, flags: impl OptionRef<'a, SliderFlags>) -> bool {
    let label = stringify(label);
  let c_label = label.cify();
  let c_format = format.into_option().unwrap_or("%d").cify();
  _drag_int_4(
    c_label.as_ptr(), 
    value as *mut i32, 
    speed.into_option().unwrap_or(1.0),
    min.into_option().unwrap_or(0),
    max.into_option().unwrap_or(0),
    c_format.as_ptr(), 
    flags.into_i32()
  ) != 0
}


pub unsafe fn input_f32<'a, D: Display>(label: D, value: &mut f32, step: impl OptionOwned<f32>, step_fast: impl OptionOwned<f32>, format: impl OptionOwned<&'static str>, flags: impl OptionRef<'a, InputTextFlags>) -> bool {
  let label = stringify(label);
  let c_label = label.cify();
  let c_format = format.into_option().unwrap_or("%.3f").cify();
  _input_float(
    c_label.as_ptr(),
    value, 
    step.into_option().unwrap_or(0.0),
    step_fast.into_option().unwrap_or(0.0),
    c_format.as_ptr(),
    flags.into_i32(),
  ) != 0
}
pub unsafe fn slider_float<'a, D: Display>(label: D, value: &mut f32, min: f32, max: f32, format: impl OptionOwned<&'static str>, flags: impl OptionRef<'a, SliderFlags>) -> bool {
  let label = stringify(label);
  let c_label = label.cify();
  let c_format = format.into_option().unwrap_or("%.3f").cify();
  _slider_float(
    c_label.as_ptr(), 
    value, 
    min, 
    max, 
    c_format.as_ptr(), 
    flags.into_i32()
  ) != 0
}
pub unsafe fn drag_float<'a, D: Display>(label: D, value: &mut f32, speed: impl OptionOwned<f32>, min: impl OptionOwned<f32>, max: impl OptionOwned<f32>, format: impl OptionOwned<&'static str>, flags: impl OptionRef<'a, SliderFlags>) -> bool {
  let label = stringify(label);
  let c_label = label.cify();
  let c_format = format.into_option().unwrap_or("%.3f").cify();
  _drag_float(
    c_label.as_ptr(), 
    value, 
    speed.into_option().unwrap_or(1.0),
    min.into_option().unwrap_or(0.0),
    max.into_option().unwrap_or(0.0),
    c_format.as_ptr(), 
    flags.into_i32()
  ) != 0
}
pub unsafe fn drag_int<'a, D: Display>(label: D, value: &mut i32, speed: impl OptionOwned<f32>, min: impl OptionOwned<i32>, max: impl OptionOwned<i32>, format: impl OptionOwned<&'static str>, flags: impl OptionRef<'a, SliderFlags>) -> bool {
  let label = stringify(label);
  let c_label = label.cify();
  let c_format = format.into_option().unwrap_or("%d").cify();
  _drag_int(
    c_label.as_ptr(), 
    value, 
    speed.into_option().unwrap_or(1.0),
    min.into_option().unwrap_or(0),
    max.into_option().unwrap_or(0),
    c_format.as_ptr(), 
    flags.into_i32()
  ) != 0
}
pub unsafe fn input_f64<'a, D: Display>(label: D, value: &mut f64, step: impl OptionOwned<f64>, step_fast: impl OptionOwned<f64>, format: impl OptionOwned<&'static str>, flags: impl OptionRef<'a, InputTextFlags>) -> bool {
  let label = stringify(label);
  let c_label = label.cify();
  let c_format = format.into_option().unwrap_or("%.3f").cify();
  _input_double(
    c_label.as_ptr(),
    value, 
    step.into_option().unwrap_or(0.0),
    step_fast.into_option().unwrap_or(0.0),
    c_format.as_ptr(),
    flags.into_i32(),
  ) != 0
}
pub unsafe fn input_i32<'a, D: Display>(label: D, value: &mut i32, step: impl OptionOwned<i32>, step_fast: impl OptionOwned<i32>, flags: impl OptionRef<'a, InputTextFlags>) -> bool {
  let label = stringify(label);
  let c_label = label.cify();
  _input_int(
    c_label.as_ptr(),
    value, 
    step.into_option().unwrap_or(1),
    step_fast.into_option().unwrap_or(100),
    flags.into_i32(),
  ) != 0
}
pub unsafe fn input_usize<'a, D: Display>(label: D, value: &mut usize, step: impl OptionOwned<i32>, step_fast: impl OptionOwned<i32>, flags: impl OptionRef<'a, InputTextFlags>) -> bool {
  let mut i32 = *value as i32;
  let res = input_i32(label, &mut i32, step, step_fast, flags);
  *value = i32 as usize;
  res
}
pub unsafe fn checkbox<D: Display>(label: D, value: &mut bool) -> bool {
  let label = stringify(label);
  let c_label = label.cify();
  let mut i = if *value { 1 } else { 0 };
  let res = _checkbox(c_label.as_ptr(), &mut i) != 0;
  *value = i != 0;
  res
}

pub unsafe fn color_edit_3<'a, D: Display>(label: D, color: &mut [f32; 3], flags: impl OptionRef<'a, ColorEditFlags>) -> bool {
  let label = stringify(label);
  let c_label = label.cify();
  _color_edit_3(
    c_label.as_ptr(), 
    color as *mut f32, 
    flags.into_i32(),
  ) != 0
}
pub unsafe fn color_edit_4<'a, D: Display>(label: D, color: &mut [f32; 4], flags: impl OptionRef<'a, ColorEditFlags>) -> bool {
  let label = stringify(label);
  let c_label = label.cify();
  _color_edit_4(
    c_label.as_ptr(), 
    color as *mut f32, 
    flags.into_i32(),
  ) != 0
}
pub unsafe fn color_picker_3<'a, D: Display>(label: D, color: &mut [f32; 3], flags: impl OptionRef<'a, ColorEditFlags>) -> bool {
  let label = stringify(label);
  let c_label = label.cify();
  _color_picker_3(
    c_label.as_ptr(), 
    color as *mut f32, 
    flags.into_i32(),
  ) != 0
}
pub unsafe fn color_picker_4<'a, D: Display>(label: D, color: &mut [f32; 4], flags: impl OptionRef<'a, ColorEditFlags>, ref_col: impl OptionOwned<[f32; 4]>) -> bool {
  let label = stringify(label);
  let c_label = label.cify();
  let mut p_ref_col = std::ptr::null();
  if let Some(ref_col) = ref_col.into_option() {
    p_ref_col = ref_col.as_ptr();
  }
  _color_picker_4(
    c_label.as_ptr(), 
    color as *mut f32, 
    flags.into_i32(),
    p_ref_col,
  ) != 0
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

/*
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
*/

fn stringify<D: Display>(d: D) -> String {
  format!("{d}").replace("\"", "").replace("\\", "")
}



