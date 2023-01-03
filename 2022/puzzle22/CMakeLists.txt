add_executable(puzzle22 main.cpp)
target_include_directories(puzzle22 PRIVATE ../include)

#copy input file to build directory
configure_file(input.txt input.txt)