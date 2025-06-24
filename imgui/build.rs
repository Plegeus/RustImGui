
extern crate bindgen;


fn main() {
  link();
  bind();    
  //compile();
}


fn link() {

  #[cfg(target_os = "macos")]
  println!("cargo:rustc-link-search=/usr/local/lib/cms");

  println!("cargo:rustc-link-lib=ImGui_c");
  println!("cargo:rustc-link-lib=ImGui_cpp");
  #[cfg(target_os = "macos")] {
    println!("cargo:rustc-link-lib=ImGui_cpp_mtl");
    println!("cargo:rustc-link-lib=c++");   
    println!("cargo:rustc-link-lib=framework=Foundation"); 
    println!("cargo:rustc-link-lib=framework=Metal"); 
    println!("cargo:rustc-link-lib=framework=QuartzCore"); 
    println!("cargo:rustc-link-lib=framework=Cocoa"); 
    println!("cargo:rustc-link-lib=framework=IOKit"); 
  }

}
fn bind() {
    
  // https://rust-lang.github.io/rust-bindgen/customizing-generated-bindings.html
  bindgen::Builder::default()
    .header("../ImGui_cpp/include/imgui_cpp.h")
    .default_visibility(bindgen::FieldVisibilityKind::PublicCrate) // https://docs.rs/bindgen/latest/bindgen/struct.Builder.html#method.default_visibility
    .generate()
    .unwrap()
    .write_to_file("./src/bindings/imgui_cpp.rs")
    .unwrap();
  bindgen::Builder::default()
    .header("../ImGui_c/include/imgui_c.h")
    .default_visibility(bindgen::FieldVisibilityKind::PublicCrate) // https://docs.rs/bindgen/latest/bindgen/struct.Builder.html#method.default_visibility
    .generate()
    .unwrap()
    .write_to_file("./src/bindings/imgui_c.rs")
    .unwrap();

  // !!! WARNING !!!
  // at the time of writing, bindgen does not cover all of C++
  // !!! WARNING !!!
  bindgen::Builder::default()
    .clang_args(["-x", "c++"]) // explicitely tell clang to parse C++, not C.
    .header("../deps/imgui-docking/imgui.h")
    //.allowlist_item(r#"enum [a-zA-Z]* { .* }"#)
    .generate()
    .unwrap()
    .write_to_file("./src/bindings/imgui_h.rs")
    .unwrap();

}
fn compile() {

  use std::process::Command;

  fn _compile(dir: &str) {
    Command::new("cmake")
      .args(["-B", "./build"])
      .current_dir(dir)
      .status()
      .unwrap();
    Command::new("cmake")
      .args(["--build", "./build"])
      .current_dir(dir)
      .status()
      .unwrap();
  }

  #[cfg(target_os = "windows")]
  _compile("./../ImGui_cpp_vk");
  #[cfg(target_os = "macos")] ();
  // xcode...
  _compile("./../ImGui_cpp");
  _compile("./../ImGui_c");


}

