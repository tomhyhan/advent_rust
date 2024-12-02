cmake_minimum_required(VERSION 3.10)
project(AdventOfCode)

set(CMAKE_CXX_STANDARD 20)

file(GLOB_RECURSE SOURCES "src/*.cpp")

add_executable(aoc ${SOURCES}) 

target_include_directories(aoc PRIVATE src)