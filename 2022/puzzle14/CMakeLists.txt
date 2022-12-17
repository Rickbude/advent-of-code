add_executable(puzzle14 main.cpp)
target_link_libraries(puzzle14 Eigen3::Eigen)
target_include_directories(puzzle14 PRIVATE ../include)

#copy input file to build directory
configure_file(input.txt input.txt)