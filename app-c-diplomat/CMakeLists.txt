cmake_minimum_required(VERSION 3.15)

project(NETB)

add_executable(netb main.c)

# Including Corrosion
include(FetchContent)
FetchContent_Declare(Corrosion GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git GIT_TAG v0.5.0)
FetchContent_MakeAvailable(Corrosion)

# Importing the Rust library as a CMake target "rustui"
corrosion_import_crate(MANIFEST_PATH rustui/Cargo.toml)

target_link_libraries(netb PRIVATE rustui)