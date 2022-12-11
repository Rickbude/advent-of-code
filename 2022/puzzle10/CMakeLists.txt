add_executable(puzzle10 main.cpp)
target_include_directories(puzzle10 PRIVATE ../include)

#copy input file to build directory
configure_file(input.txt input.txt)