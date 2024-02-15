#!/bin/bash -x
_LIB_SRC=mid_exit
#clang -v -c -target x86_64-pc-windows-gnu -o ${_LIB_SRC}.o ${_LIB_SRC}.c
clang -v -c -o ${_LIB_SRC}.o ${_LIB_SRC}.c
# create the library
ar rcs lib${_LIB_SRC}.a ${_LIB_SRC}.o

# NOTE: Even on Linux, I'll just generate with the '.exe' extension for the sake of consistency
_BIN_SRC=test1
#clang++ -v -target x86_64-pc-windows-gnu -std=c++20 -L. -o ${_BIN_SRC}.exe ${_BIN_SRC}.cpp -l${_LIB_SRC} 
clang++ -v -std=c++20 -L. -o ${_BIN_SRC}.exe ${_BIN_SRC}.cpp -l${_LIB_SRC} 
chmod +x ${_BIN_SRC}.exe

# lastly/finally, if this is a Windows platform running in MinGW, you need:
echo '#$ rustup toolchain install stable-x86_64-pc-windows-gnu'
echo '#$ rustup default stable-x86_64-pc-windows-gnu'
