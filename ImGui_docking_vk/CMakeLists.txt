
cmake_minimum_required(VERSION 3.24.3)
project(ImGui_docking_vk)

set(CMAKE_CXX_STANDARD 23)
set(LIBRARY_OUTPUT_PATH ${PROJECT_SOURCE_DIR}/target)


add_library(ImGui_docking_vk STATIC ${PROJECT_SOURCE_DIR}/src/imgui_docking_vk.cpp)



