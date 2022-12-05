My solutions to the Advent of Code coding challenges, [2022 edition](https://adventofcode.com/2022).

Most (all?) of the code in this folder is written in C++.
To compile it you will need `cmake` and a C++ compiler.

Below are the instructions to compile and run the code on Linux, 
but the process should be similar (and should work) on Windows (not tested).

First, clone the repository:

```bash
   git clone https://github.com/Rickbude/advent-of-code.git
```

Then, change directory to the 2022 edition, make a build folder, and build

```bash
   cd advent-of-code
   mkdir build
   cd build
   cmake ..
   make -j6
```

The samples can be run by changing directory to that specific puzzle.
This is needed because the file paths are relative. So to run puzzle 1:

```bash
   cd puzzle1
   ./puzzle1
```
