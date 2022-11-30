cmake_minimum_required(VERSION 3.21.0)
project(ddriver VERSION 0.1)

# Include
set(INCLUDE_DIRS
  "${PROJECT_SOURCE_DIR}"
  "${PROJECT_SOURCE_DIR}/include"
)
message(STATUS "Include dirs: ${INCLUDE_DIRS}")
include_directories(${INCLUDE_DIRS})

# Sources
add_library(ddriver STATIC "${PROJECT_SOURCE_DIR}/ddriver.cpp")

# Links
# link_libraries(m)
