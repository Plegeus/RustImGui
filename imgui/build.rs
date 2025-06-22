
extern crate bindgen;
use std::path::PathBuf;


fn main() {

  println!("cargo:rustc-link-search=/Users/timo/Documents/coding/black-death-studios/the-cosmos-game-engine/deps/imgui_bindings/ImGui_bindings/target");
  println!("cargo:rustc-link-search=/Users/timo/Documents/coding/black-death-studios/the-cosmos-game-engine/deps/imgui_bindings/ImGui_docking/target");
  println!("cargo:rustc-link-search=/Users/timo/Documents/coding/black-death-studios/the-cosmos-game-engine/deps/imgui_bindings/ImGui_docking_mtl/DerivedData/ImGui_docking_mtl/Build/Products/Debug");
  println!("cargo:rustc-link-search=/Users/timo/Documents/coding/glfw-3.3.9.bin.MACOS/lib-arm64");
  
  println!("cargo:rustc-link-lib=ImGui_bindings");
  println!("cargo:rustc-link-lib=ImGui_docking");
  println!("cargo:rustc-link-lib=ImGui_docking_mtl");
  println!("cargo:rustc-link-lib=glfw3");

  // system...
  println!("cargo:rustc-link-lib=c++");   
  println!("cargo:rustc-link-lib=framework=Foundation"); 
  println!("cargo:rustc-link-lib=framework=Metal"); 
  println!("cargo:rustc-link-lib=framework=QuartzCore"); 
  println!("cargo:rustc-link-lib=framework=Cocoa"); 
  println!("cargo:rustc-link-lib=framework=IOKit"); 
  
  bind(
    "/Users/timo/Documents/coding/black-death-studios/the-cosmos-game-engine/deps/imgui_bindings/ImGui_bindings/include/imgui_bindings.h",
    "./src",
    "bindings.rs"
  );

}


fn bind(header: &str, target: &str, binding: &str) {

  println!("cargo:rerun-if-changed={}", header);

  let out_path = PathBuf::from(target);

  // https://rust-lang.github.io/rust-bindgen/customizing-generated-bindings.html
  let bindings = bindgen::Builder::default()
    .header(header)
    .default_visibility(bindgen::FieldVisibilityKind::PublicCrate) // https://docs.rs/bindgen/latest/bindgen/struct.Builder.html#method.default_visibility
    .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    .generate()
    .expect(format!("Unable to generate bindings for {}", header).as_str());
  bindings
    .write_to_file(out_path.join(binding))
    .expect(format!("Couldn't write bindings to {}!", binding).as_str());

}



