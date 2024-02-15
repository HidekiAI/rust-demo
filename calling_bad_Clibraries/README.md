# Unsafe extern libraries and abnormal shutdown

The purpose of this demonstrations are to evaluate and strategize on solutions of what to do with unsafe C and C++ 3rd party libraries.

This all started way back when, when I was working on a project in which we had to integrate Lua interpreter within our C++ application.  (Please bare weith me, I know this is Rust, but it will all tie together).

When Lua interpreter has issues, it will try to play nice and call C library method [`exit()`](https://cplusplus.com/reference/cstdlib/exit/).  I am not sure what this means for those who aren't familiar with this scary O/S function, but if a C++ application calls a C (or C++) library in which they call `exit()`, the C++ application will *IMMEDIATELY TERMINATE*!  And to make it worse, O/S call of `exit()` is not even considered `unhandled exception`, hence you will get:

- ALL C++ destructors will not be called
- ALL temporary files opened will not have the chance to close
- Don't pass GO, go immediately to jail...

and so on... you get the point... yes, scary...  Keep in mind, that this is the same with C++, if C++ developer decides s/he wants to just use O/S C library `exit()` instead of throwing an exception, the same leaks will happen.

One way to fight this is that you can spawn/fork a thread, in which the main thread will still keep running, but the spawned thread will just exit (again, still scary, because that spawned/forked thread *DID NOT* get cleaned up!).

## How bad is it?

TODO: Demonstrate whether the C++ today differs from C++ "back then", in which back-then if I wrapped with `try{...}catch(){}` block, it will not be caught!

First the C code, but this is the same if it was C++, but because I want to demonstrate it as C, I am using `printf()` instead of `cout` (I once had a job interview where the person asked me to write a code "in C++" and so I used `cout` instead of `printf()` and was later suggested/corrected that I should have used `printf()`, in which I (probably arrogantly?) argued that "...but... `printf()` is C library, *not* C++..." and he said I am wrong (and then the test ended, and was told the interview is over)).

```cpp
void mid_exit(int status)
{
    printf("mid_exit(): Calling exit() now...");
    exit(status);
    printf("mid_exit(): This will not print...");
}
```

```cpp
int main() {
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
```

And the output is:

```bash
main(0): About to call 'mid_exit(-666)'
mid_exit(): Calling exit() now...
```

OK, what if that call to `exit()` was something more common, such as an access violation to a pointer address that should not be executable, or writing to an address that should never be written, most common is NULL pointer:

```cpp
void mid_access_violation()
{
    /* define NULL pointer as 'void *' */
    void *ptr = NULL;
    printf("mid_access_violation(): about to write some value to NULL pointer; seg-fault here we come...\n");
    // write a value 42 to address of where ptr is pointing
    *(int *)ptr = 42;   // ptr is null, so we're trying to write to address 0x0000_0000_0000_0000
    printf("mid_access_violation(): This will not print...\n");
}
```

And the output is:

```bash
main(0): About to call 'mid_access_violation()'
mid_access_violation(): about to write some value to NULL pointer; seg-fault here we come...
Segmentation fault
```

As you can see, it also acts in the similar ways as `exit()`, in that it will not fall through to the next line, and it will not be caught by `try{...}catch(){}`.

If it cannot be caught via `catch` block, there are no way to do cleanup, so what can be done?

## Fork and Join

One solution I have was to fork a new thread, and in that thread, call the unsafe library function, and in the main thread, just wait for the child thread to finish (via join which blocks), and then do cleanup.

There are few caveats to this.  First is it can only clean up resources that are allocated OUTSIDE the thread, in which means that for each thread we'd fork, we need to preallocate all the resources, `FILE *` pointer, etc in which either we can `delete` or `delete[]` (so each objects can call its `dtor`), `free()` (any heap that were `malloc`ed), or `close()` (files that were open exclusively for writing and blocking others from accessing it), outside the thread after all is join`ed back/unblocked.  What a maintenance hell!

And of course, the elephant in the room is the puzzle about whether that C/C++ library that caused seg-fault or `exit()` probably (OK, definitely) did not clean itself up.  I think that is why the nice-kind-smart people at Microsoft for example will try to flush your toilet if you had not done so when the application exits when possible (but not all the time).  Even then, there are times when even the `kill -9 $(pidof my_app)` (in Linux) will not release the darn zombie-threads and zombie-processes.  But well, so much we can do, either we should avoid using such libraries or if it is still being maintained, participate in bug-report and cross your fingers...

