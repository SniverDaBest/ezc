[![Builds](https://github.com/SniverDaBest/ezc/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/SniverDaBest/ezc/actions/workflows/rust.yml)

# How to use
`ezc` is supposed to be really easy to use, as `YAML` is a very user friendly, easy to use configuration format.\
Here's an example of an `ezc.yml` file in your project:
```
cc: gcc
src:
  - src/main.c
output: someapp
std: c99
flags:
  - -Wall
  - -Wextra
extra_libs:
  - m
```
Here's a walkthrough of what it does:
1. `cc` defines the compiler to use. It usually will be `gcc`, or `g++`, depending if you use C or C++.
2. `src` defines the source files to compile.
3. `output` labels the output file. In this example, it's `someapp`.
4. `std` is optional, and is used to define the version of the standard library you want to use. For example, on C, you could use `c99`, which is the C99 standard.
5. `flags` defines optional flags to compile with. It could include things like `-Wall`, `-m64`, and other things.
6. `extra_libs` defines extra libraries to link with. `m` is the mathematical library for C/C++, which is used in many cases.
<!--><!-->
You also could do the following:
```
cc: g++
src:
  - src/*.cpp
output: otherapp
flags:
  - -Wall
  - -Wextra
  - -m64
```
Here's what that config uses:
1. `cc` is `g++`. This means it's a C++ based project.
2. In the `src` files, we do `src/*.cpp`, which gets all files in the `src` directory that end with `.cpp`.
3. We set the output file to be `otherapp`, to differ from the previous example.
4. We don't define the standard, as it's optional, and is set by the compiler by default.
5. We use the same flags as before, but we addded `-m64`, to make the compiler compile a 64-Bit application.
6. We don't actually need to define extra libraries. They aren't used in some apps.
