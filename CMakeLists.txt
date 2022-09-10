cmake_minimum_required(VERSION 3.8)
project(ecc)

file(GLOB_RECURSE SOURCE_LIST LIST_DIRECTORIES false CONFIGURE_DEPENDS ${PROJECT_SOURCE_DIR}/src/*.c)
add_executable(
    ecc
    ${SOURCE_LIST}
)

enable_testing()

add_test(NAME sample_test COMMAND ${PROJECT_SOURCE_DIR}/test/sample_test.sh)
