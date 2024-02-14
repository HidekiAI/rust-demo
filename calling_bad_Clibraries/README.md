# Unsafe extern libraries

The purpose of this demonstrations are to evaluate and strategize on solutions of what to do with unsafe C and C++ 3rd party libraries.

This all started way back when, when I was working on a project in which we had to integrate Lua interpreter within our C++ application.  (Please bare weith me, I know this is Rust, but it will all tie together).

When Lua interpreter has issues, it will try to play nice and call C library method `exit()`.  I am not sure what this means for those who aren't familiar with this scary O/S function, but if a C++ application calls a C (or C++) library in which they call `exit()`, the C++ application will exit!  And to make it worse, O/S call of `exit()` is not even considered `unhandled exception`, hence you will get:

- ALL C++ destructors will not be called
- ALL temporary files opened will not have the chance to close

and so on... you get the point... yes, scary...  Keep in mind, that this is the same with C++, if C++ developer decides s/he wants to just use O/S C library `exit()` instead of throwing an exception, the same leaks will happen.

One way to fight this is that you can spawn/fork a thread, in which the main thread will still keep running, but the spawned thread will just exit (again, still scary, because that spawned/forked thread *DID NOT* get cleaned up!).

TODO: Demonstrate whether the C++ today differs from C++ "back then", in which back-then if I wrapped with `try{...}catch(){}` block, it will not be caught!

