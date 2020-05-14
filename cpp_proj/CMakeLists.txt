cmake_minimum_required (VERSION 3.9)
cmake_policy(VERSION 3.9)

project(cpp_proj)

set(CMAKE_POSITION_INDEPENDENT_CODE ON)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_STANDARD 11)

add_library(cpp_proj STATIC lib.cpp)
