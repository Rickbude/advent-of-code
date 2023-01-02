add_executable(puzzle20 main.cpp)
target_include_directories(puzzle20 PRIVATE ../include)

#copy input file to build directory
configure_file(input.txt input.txt)