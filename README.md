# RustImGui
Rust bindings and low level wrappers around the Dear ImGui ui rendering framework.

## Structure
### C++ From Rust
Because Dear ImGui is written in C++ and Rust does not directly support C++ calls, the functions of Dear ImGui are first wrapped in C calls.
Using bindgen, said C calls are then possible from Rust, though wrapped in a *rust friendly* functions.

Since Rust does not support variadic nor overloaded functions, the wrapper functions take Option<T> where optional arguments are found in the C++ code.

 - imgui: Rust crate
 - imgui_derive: derive macros for imgui
 - ImGui_docking: the Dear ImGui source code (wrapped in C compatible calls)
 - ImGui_docking_mtl: Apple specific code for ImGui_docking
 - ImGui_bindings: Rust compatible C calls

## Contributions
Feel free to contribute.

