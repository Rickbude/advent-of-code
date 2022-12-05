add_executable(puzzle5_part1 main.cpp)
target_compile_definitions(puzzle5_part1 PRIVATE DO_PART1)
add_executable(puzzle5_part2 main.cpp)
target_compile_definitions(puzzle5_part2 PRIVATE DO_PART2)

#copy input file to build directory
configure_file(input.txt input.txt)