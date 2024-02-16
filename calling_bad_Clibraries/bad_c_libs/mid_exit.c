#include <stdlib.h>
#include <stdio.h>
#include <math.h>
#include "mid_exit.h"

int mid_exit(int status)
{
    printf("mid_exit(): Calling exit() now...\n");
    exit(status);
    printf("mid_exit(): This will not print...\n");
    return(0);
}

int mid_divide_by_zero()
{
    int not_a_number = 0; // how do we assign a NaN in C?
    printf("mid_divide_by_zero(): about to divide by zero...\n");
    not_a_number = 1 / 0; // note that at least with clang (not sure of gcc), you will get a warning about division-by-zero
    if (isnan((float) not_a_number))
    {
        printf("mid_divide_by_zero(): This will print as expected...\n");
    }
    not_a_number = sqrt(-1); // but clang cannot warn about this...
    not_a_number = log(0);
    not_a_number = log(-1);
    not_a_number = asin(2);
    printf("mid_divide_by_zero(): This WILL print - In C/C++ NaN and INF are valid answer...\n");
    return(0);
}

/* NOTE: from what I understand, most "modern" systems
 *  will seg-fault writing to NULL ptr, but this
 *  is NOT 100% garuanteed and on some cases, it will NOT
 *  cause seg-faults */
int mid_access_violation()
{
    /* define NULL pointer as 'void *' */
    void *ptr = NULL;
    printf("mid_access_violation(): about to write some value to NULL pointer; seg-fault here we come...\n");
    // write a value 42 to address of where ptr is pointing
    *(int *)ptr = 42; // ptr is null, so we're trying to write to address 0x0000_0000_0000_0000
    printf("mid_access_violation(): This will not print...\n");
    return(0);
}
