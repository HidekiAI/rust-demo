# rust-demo

Highlights of what I think are great about rust; I will add/share them as I remember as well as I learn...

- [nested_lambdas](nested_lambdas/README.md) - A simple demonstrations to use loops and lambda results to step into inner loops and lambdas; a very nice concept especially for treating ```Option<T>``` None as 0 and ```Option<T>``` Some as 1 element of an array.  This sample includes map() and fold() as well, to represent that you can reduce all the if and match into just 1 line
- [procedural macros](derive_attribute_macros/README.md) - a boilerplate like project in which it demonstrates having regular lib and bin with proc-macro library into a workspace.  Most examples you find is just standalone proc-macros, and not a real-world example in which you have a project in which you needed to write a proc-macro for your app/bin and libs to use.
- [unsafe C libraries](calling_bad_Clibraries/README.md) - a write up on the issue of our favorite C and/or C++ libraries that can possibly terminate (either via segmentation fault or even process-exit() call) leaving the user (rust and/or C++) of the library leaking memory and/or application-termination triggering WITHOUT resource cleanup (no destructors gets called, no temp files gets closed, MUTEX not getting released, etc).
Last updated: 2024-02-09
