add_executable(puzzle21 main.cpp)
target_include_directories(puzzle21 PRIVATE ../include)

#copy input file to build directory
configure_file(input.txt input.txt)