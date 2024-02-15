#!/bin/bash -x
# NOTE: This script is just a BASH version of build.rs for
#       compiling and generating the C library as well
#       as generating C++ main program to demonstrate the
#       effect of the C library.
#       The executable binary (i.e. main.exe) will NOT be
#       generated via build.rs so if you want to see the
#       effect in C++, you need to run this script.
# Alternatively, I could probably have build.rs call this
# script to generate the C library and the C++ main program
# but the issue of not knowing the location of OUT_DIR
# would cause linker problem (on rustc side); but I did try/explore
# the idea and in the end, I gave up and will directly build
# the C library (via clang/gcc/ar) in build.rs.
# Because there are TWO WAYS to build the C library, the authorative
# method is from build.rs, thus if there are conflicts on output
# names, etc, the changes/aproaches should FROM build.rs and it
# should be propagated/updated to this script as well.
# Again, in the end, use "cargo build" to build the C-lib via
# build.rs so that bind-gen can locate the library correctly.
_IS_NIX=1
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    echo -e "\n# Running on Linux"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    echo -e "\n# Running on macOS (Darwin)"
    _IS_NIX=0
elif [[ "$OSTYPE" == "cygwin" ]]; then
    echo -e "\n# Running on Cygwin (POSIX compatibility layer for Windows)"
    _IS_NIX=0
elif [[ "$OSTYPE" == "msys" ]]; then
    echo -e "\n# Running on MSYS (Lightweight shell and GNU utilities for Windows)"
    _IS_NIX=0
elif [[ "$OSTYPE" == "win32" ]]; then
    echo -e "\n# Running on Windows (MS-DOS prompt)"
    _IS_NIX=0
else
    echo -e "\n# Unknown operating system"
    exit -1
fi

#NOTE: On build.rs, it checks the mtime of mid_exit.h and mid_exit.c and
#      if the mtime is 0, it will recompile the C library.
# I can do that in Linux (I just use `find . -mtime 0`), but not too sure (or care) about Windows, so 
# I will just brute-force recompile the C library every time this script

# NOTE: On build.rs, we ASSUME the static library extension is ".a" even on Windows...
_LIB_SRC=mid_exit
#clang -v -c -target x86_64-pc-windows-gnu -o ${_LIB_SRC}.o ${_LIB_SRC}.c
clang -v -c -o ${_LIB_SRC}.o ${_LIB_SRC}.c
# create the library
ar rcs lib${_LIB_SRC}.a ${_LIB_SRC}.o

# We'll just create a shared object as well...  For build.rs, we don't care, we statically link always!
#clang -v -shared -target x86_64-pc-windows-gnu -o ${_LIB_SRC}.dll ${_LIB_SRC}.c
if [[ $_IS_NIX -eq 0 ]]; then
    clang -v -shared -o ${_LIB_SRC}.dll ${_LIB_SRC}.c
    chmod +x ${_LIB_SRC}.dll
else
    clang -v -shared -o ${_LIB_SRC}.so ${_LIB_SRC}.c
    chmod +x ${_LIB_SRC}.so
fi

# And now, the C++ main program for testing how it affects on C++ (good for comparing agains Rust)
# NOTE: Even on Linux, I'll just generate with the '.exe' extension for the sake of consistency
_BIN_SRC=test1
#clang++ -v -target x86_64-pc-windows-gnu -std=c++20 -L. -o ${_BIN_SRC}.exe ${_BIN_SRC}.cpp -l${_LIB_SRC} 
clang++ -v -std=c++20 -L. -o ${_BIN_SRC}.exe ${_BIN_SRC}.cpp -l${_LIB_SRC} 
chmod +x ${_BIN_SRC}.exe

# lastly/finally, if this is a Windows platform running in MinGW, you need:
echo '#$ rustup toolchain install stable-x86_64-pc-windows-gnu'
echo '#$ rustup default stable-x86_64-pc-windows-gnu'
