add_executable(puzzle15 main.cpp)
target_include_directories(puzzle15 PRIVATE ../include)

#copy input file to build directory
configure_file(input.txt input.txt)