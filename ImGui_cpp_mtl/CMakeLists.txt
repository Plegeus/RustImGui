
cmake_minimum_required(VERSION 3.24.3)
project(ImGui_cpp_mtl)

set(CMAKE_CXX_STANDARD 23)
set(LIBRARY_OUTPUT_PATH ${PROJECT_SOURCE_DIR}/target)


add_library(ImGui_cpp_mtl STATIC ${PROJECT_SOURCE_DIR}/src/imgui_cpp_mtl.mm)

target_include_directories(ImGui_cpp_mtl PRIVATE ${PROJECT_SOURCE_DIR}/../deps/imgui-docking)



