cmake_minimum_required(VERSION 3.21.0)
project(rfs_drv VERSION 0.1)

include(CTest)
enable_testing()

set(CMAKE_EXPORT_COMPILE_COMMANDS true)
#set(CMAKE_CXX_STANDARD 17)

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

#set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -D_FILE_OFFSET_BITS=64 -no-pie")
add_compile_options(-D_FILE_OFFSET_BITS=64 -no-pie)
set(CMAKE_C_FLAGS_DEBUG "${CMAKE_C_FLAGS_DEBUG} -Wall --pedantic -g")
set(CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/CMake" ${CMAKE_MODULE_PATH})
# set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
# set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
# set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

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

#set(RS_BIND_OUT ${PROJECT_SOURCE_DIR}/rfs_bind/bindings.rs)

#if(NOT EXISTS ${RS_BIND_OUT})
#    message(STATUS "Generate rust-c bindings...")
#    execute_process(COMMAND bindgen ${DRIVER_PATH}/include/ddriver.h -o ${RS_BIND_OUT})
#endif()

configure_file(${PROJECT_SOURCE_DIR}/rfs_bind/build.rs.in ${PROJECT_SOURCE_DIR}/rfs_bind/build.rs)

# cxxbridge for rust
corrosion_import_crate(MANIFEST_PATH ${PROJECT_SOURCE_DIR}/rfs_bind/Cargo.toml)
#corrosion_add_cxxbridge(rfs_bind CRATE rfs_bind MANIFEST_PATH rfs_bind FILES lib.rs)
corrosion_add_cxxbridge(rfs_bind_lib CRATE rfs_bind MANIFEST_PATH rfs FILES lib.rs)

#corrosion_import_crate(MANIFEST_PATH ${PROJECT_SOURCE_DIR}/rfs_bind/Cargo.toml)

# Link C/C++ libraries with your Rust target
#corrosion_link_libraries(rfs_bind ddriver)
corrosion_link_libraries(rfs_bind ddriver)
# Optionally explicitly define which linker to use.
# corrosion_set_linker(rfs your_custom_linker)
# Optionally set linker arguments
#corrosion_add_target_local_rustflags(rfs_bind "-Clink-args=-lddriver")
#corrosion_add_target_local_rustflags(rfs_bind "-Clink-args=-lddriver")
# corrosion_add_target_local_rustflags(rfs "-Clink-args=<linker arguments>")
# Optionally tell CMake that the rust crate depends on another target (e.g. a code generator)
# add_dependencies(cargo-prebuild_rfs custom_bindings_target)

#target_link_libraries(${PROJECT_NAME} ${FUSE_LIBRARIES} ddriver rfs_bind rfs_bind)
#target_link_libraries(${PROJECT_NAME} ${FUSE_LIBRARIES} ddriver rfs_bind_lib)
#target_link_libraries(${PROJECT_NAME} ${FUSE_LIBRARIES} rfs_bind_lib ddriver)

target_link_directories(${PROJECT_NAME} PUBLIC ${CMAKE_BINARY_DIR} ${CMAKE_BINARY_DIR}/user_ddriver)
target_link_libraries(${PROJECT_NAME} librfs_bind_lib.a librfs_bind.a libddriver.a)
target_link_libraries(${PROJECT_NAME} ${FUSE_LIBRARIES} ddriver rfs_bind_lib)
#target_link_libraries(${PROJECT_NAME} ${FUSE_LIBRARIES} rfs_bind)

set(MOUNT_POINT ${PROJECT_SOURCE_DIR}/tests/mnt)
set(DEVICE_PATH $ENV{HOME}/ddriver)

make_directory(${CMAKE_BINARY_DIR}/mnt)
add_test(NAME rfs-run COMMAND ${CMAKE_BINARY_DIR}/${PROJECT_NAME} --device=${DEVICE_PATH} ${MOUNT_POINT})
add_test(NAME rfs-stop COMMAND fusermount -u ${MOUNT_POINT})