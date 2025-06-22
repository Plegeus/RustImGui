cmake_minimum_required(VERSION 3.23)
project(ImGui_bindings)

set(CMAKE_CXX_STANDARD 23)
set(LIBRARY_OUTPUT_PATH  ${PROJECT_SOURCE_DIR}/target/)

add_library(ImGui_bindings STATIC src/imgui_bindings.c)

target_include_directories(ImGui_bindings PRIVATE /Users/timo/Documents/coding/black-death-studios/the-cosmos-game-engine/deps/imgui_bindings/ImGui_docking/include)



