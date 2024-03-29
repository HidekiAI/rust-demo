# Unsafe extern libraries and abnormal shutdown

The purpose of this demonstrations are to evaluate and strategize on solutions of what to do with unsafe C and C++ 3rd party libraries.  Mainly, those C-libraries that are closed sources or no-longer maintained libraries.

This all started way back when, when I was working on a project (game) in which we had to integrate Lua interpreter (to allow designers control the behaviours overriding the default A.I.) within our C++ application (game).  (Please bare weith me, I know this is Rust, but it will all tie together).

When Lua interpreter detects syntax error during execution (runtime) because we did not have a lexical-analyzer nor static-analyzer of bad Lua codes pasted into the game-console or bad .lua code checked into the source-controller, upon interpretation, it will try to play nice with the OS and call C library method [`exit()`](https://cplusplus.com/reference/cstdlib/exit/) (see also Rust library `std::process::exit()`).  I am not sure what this means for those who aren't familiar with this scary O/S function, but if a C++ application calls a C (or C++) library in which they call `exit()`, the C++ application will *IMMEDIATELY TERMINATE*!  And to make it worse, O/S call of `exit()` is not even considered `unhandled exception`, hence you will get:

- ALL C++ destructors will not be called
- ALL temporary files opened will not have the chance to close
- Log files that was about to log something and locked the file for writing, will not get to unlock the file, hence all others waiting for the log file to get write-permission will be blocked...  Hint: Log files are usually singleton so that in multi-threaded apartments, each threads can wait for its turns to log to the same file...
- Don't pass GO, go immediately to jail...

and so on... you get the point... yes, scary...  Keep in mind, that this is the same with C++, if C++ developer decides s/he wants to just use O/S C library `exit()` instead of throwing an exception, the same leaks will happen.

One way to fight this is that you can spawn/fork a thread, in which the main thread will still keep running, but the spawned thread will just exit (again, still scary, because that spawned/forked thread *DID NOT* get cleaned up!).

Somebody smart on Rust side wrote [this](https://doc.rust-lang.org/stable/std/process/fn.exit.html):

> ...
> This function will never return and will immediately terminate the current process. The exit code is passed through to the underlying OS and will be available for consumption by another process.
>
> Note that because this function never returns, and that it terminates the process, no destructors on the current stack or any other thread’s stack will be run. If a clean shutdown is needed it is recommended to only call this function at a known point where there are no more destructors left to run
> ...

And this is from [exit manual related to kernel](https://man7.org/linux/man-pages/man2/exit.2.html):

> ...
> Open stdio(3) streams are not flushed.  On the other hand, _exit() does close open file descriptors, and this may cause an unknown delay, waiting for pending output to finish.
> ...

And finally, [this](https://github.com/rust-lang/rust/issues/83994) in which, personally I agree with all the participants that says either not to use the C library or have it fixed on C library side.  I think there is an effort on `atexit` (see the kernel link above on `exit`), but personally, I really think if the original C programmer did it that way, it's "by design" and just accept that you're not able to inherit that design, so you'll have to use some other library that meets your design...

## How bad is it?

First the C code, but this is the same if it was C++, but because I want to demonstrate it as C, I am using `printf()` instead of `cout` (I once had a job interview where the person asked me to write a code "in C++" and so I used `cout` instead of `printf()` and was later suggested/corrected that I should have used `printf()`, in which I (probably arrogantly?) argued that "...but... `printf()` is C library, *not* C++..." and he said I am wrong (and then the test ended, and was told the interview is over)).

### mid_exit.c

First, the C code...

```cpp

/* somewhere in the other .h file, there will be the function signatures for the C++ side to extern "C" from... */
void mid_exit(int status)
{
    printf("mid_exit(): Calling exit() now...\n");
    exit(status);
    printf("mid_exit(): This will not print...\n");
}
```

### test1.cpp

And here's the C++ side that statically links to the C (i.e. `libmid_exit.a`):

```cpp
extern "C" {
    #include "mid_exit.h"
}

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

### mid_exit.c (part 2)

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

### Conclusion on C++

As you can see, it also acts in the similar ways as `exit()`, in that it will not fall through to the next line, and it will *NOT* be caught by `try{...}catch(){}`.

If it cannot be caught via `catch` block, there are no way to do cleanup (no destructor will be called, etc), so what can be done?

## Fork and Join

First and foremost, if you go to Stackoverflow, probably almost any questions inquired about `unsafe` call in Rust, at least one experts will recommend not to use `unsafe` in the first place!  I too agree, but that's just idealistic answer.  There are more C/C++ libraries (especially for server and backend developers) that are useful that has never been ported to Rust, hence I've not the time to port it myself, so I'd rather wrap it around `unsafe` and wait until the days when I have spare time to port it myself (which is same as never).

One solution I have done in the past (in the C# domain, not in Rust) was to fork a new thread, and within that thread, call the unsafe library function, and in the main thread, just wait for the child thread to finish (via join which blocks), and then do cleanup.  But also bare in mind, that this was for another C# library, and it did not terminate, and also, native .NET libraries (even back in .NET 3 days) were not as bad as C/C++ libraries in terms of bad behaviours...

There are few caveats to this.  First is it can only clean up resources that are allocated OUTSIDE the thread, in which means that for each thread we'd fork, we need to preallocate all the resources, `FILE *` pointer, etc in which either we can `delete` or `delete[]` (so each objects can call its `dtor`), `free()` (any heap that were `malloc`ed), or `close()` (files that were open exclusively for writing and blocking others from accessing it), outside the thread after all is joined back/unblocked.  What a maintenance hell, and besides that which C/C++ API do you know where you get to pass down a `VOID*` (that's the hint (instead of `void*`), I only know smart people from Microsoft that does that,) of pool of data in which *YOU* get to manage the resource rather than having them internally create their own.  And so much for the love of RAII since the resource allocated in constructor will never get cleande up because destructor will not have the chance to get triggered...

I think that is why the nice-kind-smart people at Microsoft for example will try to flush your toilet if you had written libraries that did not handle this case (but not all the time).  Even then, there are times when even the `kill -9 $(pidof my_app)` (in Linux) will not release the darn zombie-threads and zombie-processes.  But well, so much we can do, either we should avoid using such libraries or if it is still being maintained, participate in bug-report and cross your fingers that "they" will fix it (who are "they"?)...

My C++ test code will only demonstrate the consequences of `try-catch` block not being able to catch it, and it is not going to demonstrate the forking and joining of threads.

Also, one note to make on terms of there are some things that seems legal in Linux (I'm on Debian bookworm at the time of this writing) versus Windows when calling the C-library from Rust:

```rust
    // Output (same on both Linux and Windows):
    //      do_proc_exit(0): About to call mid_exit() from C library
    //      mid_exit(): Calling exit() now...
    do_proc_exit();

    // Output (Linux):
    //      do_div_by_zero(0): About to call mid_divide_by_zero() from C library
    //      mid_divide_by_zero(): about to divide by zero...
    //      mid_divide_by_zero(): This WILL print - In C/C++ NaN and INF are valid answer...
    //      do_div_by_zero(1): mid_divide_by_zero() from C library called successfully
    // Output (Windows):
    //      do_div_by_zero(0): About to call mid_divide_by_zero() from C library
    //      mid_divide_by_zero(): about to divide by zero...
    //      error: process didn't exit successfully: 'rust-demo\target\debug\calling_bad_Clibraries.exe' (exit code: 0xc0000094, STATUS_INTEGER_DIVIDE_BY_ZERO)
    do_div_by_zero();

    // Output (Linux):
    //      do_seg_fault(0): About to call mid_access_violation() from C library
    //      mid_access_violation(): about to write some value to NULL pointer; seg-fault here we come...
    //      Segmentation fault (core dumped)
    // Output (Windows):
    //      do_seg_fault(0): About to call mid_access_violation() from C library
    //      mid_access_violation(): about to write some value to NULL pointer; seg-fault here we come...
    //      error: process didn't exit successfully: `rust-demo\target\debug\calling_bad_Clibraries.exe` (exit code: 0xc0000005, STATUS_ACCESS_VIOLATION)
    //      Segmentation fault
    do_seg_fault();

```

The one that is most interesting is that on Windows (stable-x86_64-pc-windows-gnu) compiled using MinGW64 gcc/clang, it will actually catch (amazing!) division-by-zero in runtime!  Though MinGW64 compiler did NOT catch the issue of passing `int` type to math library (i.e. `sqrt(iValue)`) but Debian gcc did catch that warning and treated it as an error (-Wall)  so I had to type-cast it as `(float) iValue` to pass the compiler.  Lesson learnt here was/is that it's good idea to compile on both Windows AND Linux, and test on both as well for different behaviours.  Just because it compiles on one O/S, it doesn't mean the same exact gcc/clang flags will pass on another as well.

Another caveat to above, is that looking at the `exit code: 0xc00000xx`, which reminds me of the familiar runtime errors and memory checks that only gets triggered on DEBUG builds on Windows.  I'm not (at the moment) too interested in it, so I won't bother trying to test it in RELEASE, but maybe it won't catch it...  Again, I will mention that it is on my MinGW64, which I've no clue whether that is equivalent to MSVC compiler (nor does it matter to me).

## Rust (No) Solution

Rust (currently) acts similar to C++, and I think C# as well, though I've not had much hair-pulling-frustration experience of calling C/C++ libs from C# that caused issues, this is mainly because C# is VERY RICH with .NET/dotnet framework which was originally written by paid-professionals rather than as Open Source, hence I had rarely had to rely on falling back to C/C++ side for all the needed libraries were there as a "framework".  Sure, there were few that got me angry like Log4Net had issues when the log file had to roll to new file at end of the day (log file was setup to roll every 24 hrs), and the file somehow got locked due to race conditions and... yah, edge cases...

In any case in Rust calling C/C++ libraries (unsafely), if you get a seg-fault or a call to (O/S level) `exit()`, even on a separate thread, main thread will be signaled to terminate (without cleanup).  But IMHO, that's a good thing, because the real horror was if the thread did not clean itself but main and other threads kept running, that would most likely cause a more significant dead-lock/race-condition or even worse, B.S.o.D!  And you DO NOT want a B.S.o.D. on server, for most servers are locked into a cold room, and you cannot get in, so you'd have to `ssh` to it, but if it's B.S.o.D., you cannot `ssh`!!!!!  P/S: there were times when I have had kernel-panics that just hung the Linux as well, so I'm not talking about just Windows BSoD but also Linux kernel-panic.  P/S2: Who remembers Amiga's "guru meditation" (I love that!)

From a server/backend developer's view, I rather have the daemon-service go down ASAP rather than have it linger for days and suddenly, some crash or hang is detected and cannot do forensics...  At least, I can still `ssh` to the server...

From non-server developer's point of view, such as client-side video games (front-end), I'm sure a minor/small leak or delayed crash is preferred over sudden "poof" shutdown/close of the front-end application.

In any case, if you look at `main.rs`, there have been attempts made to demonstrate single-threaded apartment as well as multiple-threaded apartments, in which I'd `std::thread::spawn` a thread and within that thread, I'd even wrap it with `panic::catch_unwind` to demonstrate that there's no way in heck that you will be saved...

```rust
    let handle = std::thread::spawn(move || {
        // wrap it in catch_unwind() to demonstrate that you cannot catch it!
        let catch_result = panic::catch_unwind(|| {
            my_function()
        });
        match catch_result {
            Ok(_) => {
                println!("Thread completed successfully");
                Ok("Success")
            }
            Err(panic_value) => Err(anyhow!("Thread panicked - {:?}", panic_value)),
        }
    });
    let result = handle.join(); // should block until thread completes/returns...
    println!("THIS WILL NEVER GET PRINTED!!!!!");
```

The `println!()` after the `join()` will never be seen, mainly because the process has terminated...  You won't even see the panic message within `catch_unwind`...

Conclusion?  If the C/C++ library is still maintained, beg them to at least come up with ways to exit normally rather than abnormally;  but if that library is no longer maintained, best to write your own in Rust (and maybe share it on crates.io for they can benefit from your work), and above all, avoid that `unsafe` block!

## Caveats that bit me hard

It took me almost 3/4 of a day to finally figure this out, and I really think (and wish) it should be documented more explicitly...

This project originally was in format like so:

```bash
.
├── Cargo.toml
├── build.rs
└── src
    ├── hello.c
    ├── lib.rs
    └── main.rs

1 directory, 5 files
```

In which, I was/am getting constant rustc linker issues complaining that it cannot find the extern C functions that I am trying to call.  I repeat, the linker issue is on `rustc` not being able to locate the exported `extern "C"` methods, because it could not statically link with it.  This occurs both on Debian and Windows (MinGW64).

And then, I went to [Build Script Examples](https://doc.rust-lang.org/cargo/reference/build-script-examples.html) page for more hints, and spotted that their directory structure *DOES NOT* have `lib.rs`!

It turns out your C-library file *IS* the "lib" (so to speak)...

I'm so used to having both `main.rs` and `lib.rs` mixed that I did not see it as a concern.

All in all, if you were getting rustc linker errors, get rid of your `lib.rs` file! (This is one of the very few things I've disliked about Rust, in which it would have this default `main.rs`, `lib.rs`, `mod.rs`, etc - at first, I was annoyed of fixed filename of `Cargo.toml` but then again, I am used to default filename `Makefile` and `CMakeLists.txt` (case-sensitive))