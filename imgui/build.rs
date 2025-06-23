
extern crate bindgen;


fn main() {

  //println!("cargo:rustc-link-search=/Users/timo/Documents/coding/black-death-studios/the-cosmos-game-engine/deps/imgui_bindings/ImGui_bindings/target");
  //println!("cargo:rustc-link-search=/Users/timo/Documents/coding/black-death-studios/the-cosmos-game-engine/deps/imgui_bindings/ImGui_docking/target");`
  println!("cargo:rustc-link-search=E:/Coding/BDS/RustImGui/ImGui_c/target/Debug");
  println!("cargo:rustc-link-search=E:/Coding/BDS/RustImGui/ImGui_cpp/target/Debug");
  println!("cargo:rustc-link-search=E:/Coding/BDS/RustImGui/ImGui_cpp_vk/target/Debug");
  #[cfg(target_os = "macos")] {
    println!("cargo:rustc-link-search=/Users/timo/Documents/coding/black-death-studios/the-cosmos-game-engine/deps/imgui_bindings/ImGui_docking_mtl/DerivedData/ImGui_docking_mtl/Build/Products/Debug");
    println!("cargo:rustc-link-search=/Users/timo/Documents/coding/glfw-3.3.9.bin.MACOS/lib-arm64");
  }
  
  println!("cargo:rustc-link-lib=ImGui_c");
  println!("cargo:rustc-link-lib=ImGui_cpp");
  #[cfg(target_os = "macos")]
  println!("cargo:rustc-link-lib=ImGui_cpp_mtl");
  #[cfg(target_os = "windows")]
  println!("cargo:rustc-link-lib=ImGui_cpp_vk");
  //println!("cargo:rustc-link-lib=glfw3");
  
  #[cfg(target_os = "macos")] {
    println!("cargo:rustc-link-lib=c++");   
    println!("cargo:rustc-link-lib=framework=Foundation"); 
    println!("cargo:rustc-link-lib=framework=Metal"); 
    println!("cargo:rustc-link-lib=framework=QuartzCore"); 
    println!("cargo:rustc-link-lib=framework=Cocoa"); 
    println!("cargo:rustc-link-lib=framework=IOKit"); 
  }

  // https://rust-lang.github.io/rust-bindgen/customizing-generated-bindings.html
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






