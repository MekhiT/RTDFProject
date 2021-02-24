# cmake_minimum_required(VERSION <specify CMake version here>)
project(RTDFProject)

set(CMAKE_CXX_STANDARD 14)

include_directories(Communications)

add_executable(RTDFProject
        Communications/SDIO.cpp
        Communications/SDIO.h)
