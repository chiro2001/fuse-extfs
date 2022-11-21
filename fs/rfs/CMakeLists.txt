cmake_minimum_required(VERSION 3.21.0)
project(rfs VERSION 0.1)

set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -D_FILE_OFFSET_BITS=64 -no-pie")
set(CMAKE_C_FLAGS_DEBUG "${CMAKE_C_FLAGS_DEBUG} -Wall --pedantic -g")
set(CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/CMake" ${CMAKE_MODULE_PATH})
# set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
# set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
# set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

set(CMAKE_EXPORT_COMPILE_COMMANDS 1)

set(DRIVER_PATH ${PROJECT_SOURCE_DIR}/user_ddriver)

add_subdirectory(${DRIVER_PATH})

find_package(FUSE REQUIRED)
include_directories(${FUSE_INCLUDE_DIR} ./include)
aux_source_directory(./src DIR_SRCS)
add_executable(rfs ${DIR_SRCS})
message("FUSE_INCLUDE_DIR ${FUSE_INCLUDE_DIR}")
message("FUSE_LIBRARIES ${FUSE_LIBRARIES}")
message("DIR_SRCS ${DIR_SRCS}")
message("!!!!!**CMAKE_GENERATOR** ${CMAKE_GENERATOR}")
target_link_libraries(rfs ${FUSE_LIBRARIES} ddriver)