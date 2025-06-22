# RustImGui
Rust bindings and low level wrappers around the Dear ImGui ui rendering framework.

## Structure
Because Dear ImGui is written in C++ and Rust does not directly support C++ calls, the functions of Dear ImGui are first wrapped in C calls.
Using bindgen, said C calls are then possible from Rust, though wrapped in a *rust friendly* functions.

Since Rust does not support variadic nor overloaded functions, the wrapper functions take Option<T> where optional arguments are found in the C++ code.

## Contributions
Feel free to contribute.

