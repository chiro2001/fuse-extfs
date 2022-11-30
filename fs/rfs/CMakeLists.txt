cmake_minimum_required(VERSION 3.21.0)
project(rfs_drv VERSION 0.1)

include(CTest)
enable_testing()

set(CMAKE_EXPORT_COMPILE_COMMANDS true)

add_subdirectory(${PROJECT_SOURCE_DIR}/corrosion)

find_program(CXXBRIDGE cxxbridge PATHS "$ENV{HOME}/.cargo/bin/")
if (CXXBRIDGE STREQUAL "CXXBRIDGE-NOTFOUND")
    message("Could not find cxxbridge, trying to install with `cargo install cxxbridge-cmd'")
    find_program(CARGO cargo PATHS "$ENV{HOME}/.cargo/bin/")
    if (CARGO STREQUAL "CARGO-NOTFOUND")
        message(FATAL_ERROR "Requires cargo available in path, install via rustup https://rustup.rs/")
    endif ()
    execute_process(COMMAND ${CARGO} install cxxbridge-cmd)
    find_program(CXXBRIDGE cxxbridge PATHS "$ENV{HOME}/.cargo/bin/")
endif ()

add_compile_options(-D_FILE_OFFSET_BITS=64 -no-pie)
set(CMAKE_C_FLAGS_DEBUG "${CMAKE_C_FLAGS_DEBUG} -Wall --pedantic -g")
set(CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/CMake" ${CMAKE_MODULE_PATH})

set(DRIVER_PATH ${PROJECT_SOURCE_DIR}/user_ddriver)

add_subdirectory(${DRIVER_PATH})

find_package(FUSE REQUIRED)
include_directories(${FUSE_INCLUDE_DIR} ${PROJECT_SOURCE_DIR}/include)
aux_source_directory(${PROJECT_SOURCE_DIR}/src DIR_SRCS)
add_executable(${PROJECT_NAME} ${PROJECT_SOURCE_DIR}/src/rfs.cpp)
message("FUSE_INCLUDE_DIR ${FUSE_INCLUDE_DIR}")
message("FUSE_LIBRARIES ${FUSE_LIBRARIES}")
message("DIR_SRCS ${DIR_SRCS}")
message("CMAKE_GENERATOR: ${CMAKE_GENERATOR}")

configure_file(${PROJECT_SOURCE_DIR}/rfs_bind/build.rs.in.rs ${PROJECT_SOURCE_DIR}/rfs_bind/build.rs)

# cxxbridge for rust
corrosion_import_crate(MANIFEST_PATH ${PROJECT_SOURCE_DIR}/rfs_bind/Cargo.toml)
corrosion_add_cxxbridge(rfs_bind_lib CRATE rfs_bind MANIFEST_PATH rfs FILES lib.rs)

corrosion_link_libraries(rfs_bind ddriver)

target_link_directories(${PROJECT_NAME} PUBLIC ${CMAKE_BINARY_DIR} ${CMAKE_BINARY_DIR}/user_ddriver)
target_link_libraries(${PROJECT_NAME} librfs_bind_lib.a librfs_bind.a libddriver.a)
target_link_libraries(${PROJECT_NAME} ${FUSE_LIBRARIES} ddriver rfs_bind_lib)

set(MOUNT_POINT ${PROJECT_SOURCE_DIR}/tests/mnt)
set(DEVICE_PATH $ENV{HOME}/ddriver)

make_directory(${CMAKE_BINARY_DIR}/mnt)
make_directory($ENV{HOME}/mnt)
add_test(NAME rfs-run COMMAND ${CMAKE_BINARY_DIR}/${PROJECT_NAME} --device=${DEVICE_PATH} ${MOUNT_POINT} -d -s -f)
add_test(NAME rfs-run-home COMMAND ${CMAKE_BINARY_DIR}/${PROJECT_NAME} --device=${DEVICE_PATH} $ENV{HOME}/mnt -d -s -f)
add_test(NAME rfs-stop COMMAND fusermount -u ${MOUNT_POINT})