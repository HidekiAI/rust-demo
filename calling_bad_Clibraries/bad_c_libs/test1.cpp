#include <iostream>
extern "C" {
    #include "mid_exit.h"
}

// call mid_exit() which calls exit() from C library
int test_OS_exit() {
    auto status = -666;
    std::cout << "main(0): About to call 'mid_exit(" << status << ")'" << std::endl;
    try
    {
        mid_exit(status);   // it calls O/S level library function `exit()` which will just exit THIS app right here and never fall through below
        std::cout << "main(1): You'll NEVER see this message" << std::endl;
    }
    catch(const std::exception& e)
    {
        std::cout << "main(catch): We'll never catch here either, so any open files are not closed" << std::endl;
        std::cerr << e.what() << '\n';
    }
    std::cout << "main(2): Because mid_exit() calls `exit()`, this will never be printed" << std::endl;
    return 0;
}

// call mid_divide_by_zero() which calls abort() from C library
int test_NaN_usage() {
    std::cout << "main(0): About to call 'mid_divide_by_zero()'" << std::endl;
    try
    {
        mid_divide_by_zero();   // it calls O/S level library function `abort()` which will just exit THIS app right here and never fall through below
        std::cout << "main(1): You'll NEVER see this message" << std::endl;
    }
    catch(const std::exception& e)
    {
        std::cout << "main(catch): We'll never catch here either, so any open files are not closed" << std::endl;
        std::cerr << e.what() << '\n';
    }
    std::cout << "main(2): Because mid_divide_by_zero() calls `abort()`, this will never be printed" << std::endl;
    return 0;
}

// call mid_access_violation() which calls abort() from C library
int test_segmentation_fault(){
    std::cout << "main(0): About to call 'mid_access_violation()'" << std::endl;
    try
    {
        mid_access_violation();   // it calls O/S level library function `abort()` which will just exit THIS app right here and never fall through below
        std::cout << "main(1): You'll NEVER see this message" << std::endl;
    }
    catch(const std::exception& e)
    {
        std::cout << "main(catch): We'll never catch here either, so any open files are not closed" << std::endl;
        std::cerr << e.what() << '\n';
    }
    std::cout << "main(2): Because mid_access_violation() calls `abort()`, this will never be printed" << std::endl;
    return 0;
}

int main() {
    test_NaN_usage();    // in C/C++, unlike other languages, division by zero does not throw exception, but it causes NaN (which, in mathmaatics, it's as valid as INF)
    test_segmentation_fault();
    test_OS_exit();  // this will cause exit, so comment if want to test next line
    return 0;
}