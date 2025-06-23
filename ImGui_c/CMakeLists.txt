cmake_minimum_required(VERSION 4.0.2)
project(ImGui_c)

set(CMAKE_CXX_STANDARD 23)
set(LIBRARY_OUTPUT_PATH  ${PROJECT_SOURCE_DIR}/target/)

add_library(ImGui_c STATIC src/imgui_c.c)

#target_include_directories(ImGui_c PRIVATE /Users/timo/Documents/coding/black-death-studios/the-cosmos-game-engine/deps/imgui_c/ImGui_docking/include)
target_include_directories(ImGui_c PRIVATE ${PROJECT_SOURCE_DIR}/../ImGui_cpp/include)



