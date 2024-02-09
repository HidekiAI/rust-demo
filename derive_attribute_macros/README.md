# Sample boilerplate of libs with and without proc-macro

"proc-macro" stands for [Procedural Macros](https://doc.rust-lang.org/reference/procedural-macros.html).  There is another kind of macros called [Macros By Example (MBE)](https://doc.rust-lang.org/reference/macros-by-example.html) but IMHO I do not mean to write lexical-analyzer desciplining the coder to follow the syntax I want (at least for now), so I will stick to the proc-macro for this demo.

Procedural macros are treated during compile time.  As a dotnet/.net developer, when you see these kind of syntax, it reminds us of something similar to [`[Attributes]`](https://learn.microsoft.com/en-us/dotnet/csharp/advanced-topics/reflection-and-attributes/) declarations, in which, it will make it harder to grasp the differences between code/compile-time and runtime.  Only similarities IMHO is  that when you write your own `[Attribute]` in C# (I've never written on in F#, but I think that should be one of the things I'd like to add to my checklist of "I've done it once" :grin: ) and proc-macro is that they both end up as an individual crate (or DLL) in which you'd link against and inherit its characteristics without coding anything.  Other than than, proco-macros are more complex compared to dotnet `[Attribute]`, and that dotnet is runtime reflection (for example, you write an app in which you give it a plug-in feature, that needs to inspect during runtime whether that DLL can be plugged in or not) in which you'd link against it and cross your finger during run-time (or unit-test time).

Procedural macros are useful for:

- auto generating code via derivations (i.e. `Debug` and `PartialEq`, etc - see [here](https://github.com/rust-lang/rust/blob/master/compiler/rustc_builtin_macros/src/lib.rs#L116)).  All in all, if you come from C{urly} languages (C|C++|C#), mainly languages that handles OOP to inherit (derive) "something", then sooner or later, you want to write a library that will inherit attributes (as mentioned above, most similar to dotnet `[Attribute]`).
- annotating behavior of the code during runtime (including test):
  - in code block such as `if cfg!(feature="my_feature") {...}`
  - in unit-test such as `#[test_case(-1, -42; "both are negative")] fn test_negative(l: i32, r: i32) {...}` in which `cargo test` will run the test.
- the standard "macros" we're used to from other languages (i.e. `println!`, `format!`, etc) known as [Function-Like Procedural Macros](https://doc.rust-lang.org/reference/procedural-macros.html#function-like-procedural-macros) (also see C/C++ [Function-Like macros](https://gcc.gnu.org/onlinedocs/cpp/Function-like-Macros.html)); note that the syntax is similar to MBE, so I've written one, but probably will not write one unless I really really need it.

All in all, one have to constantly remind ourselves that Rust is a compile-time language, and that the proc-macro is a compile-time feature.  Hence it is a bit more complicated compared to C/C++ "Function-Like Macros", but if you (like myself) have more than once declared "I'll NEVER USE MACROS EVER AGAIN!" after finding that hard-to-find RUNTIME bug/error in C++, just remember, it's not that kind of macros...  If you've written Rust for at least few weeks, you've already have appreciated the `#[derive(Debug)]`, or have seen compiliers complain (again, I want to emphasize, NOT RUNTIME) that you should add `Clone` but you don't need to `impl` the cloning logic because your `struct` is made off of all primitives (P.o.D), you're already using them and have appreciated how useful they are.

Directory structure will look like so:

```cargo
my_workspace/
├── Cargo.toml
├── my_lib/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── main.rs (optional)
│       ├── my_module1/
│       │   ├── mod.rs
│       │   └── sub_module1.rs
│       └── ...
└── my_macros/
    ├── Cargo.toml
    └── src/
        ├── lib.rs
        ├── my_module2/
        │   ├── mod.rs
        │   └── sub_module2.rs
        └── ...
```

- because you CANNOT share proc-macro sub-modules with other sub-modules, you must create a separate crate for your macros
- the "my_lib" is the crate (with its own Cargo.toml) that will not declare proc-macro related modules
- the "my_macros" is the crate (with its own Cargo.toml) that will declare proc-macro related modules (in mod.rs)

Remember, you *must* declare `proc-macro = true` in Cargo.toml as a crate (per crate).  Per-crate, because if you do not declare `#[proc_macro]` at the "root" level of crates, though you can define multiple `pub fn`s in the same crate, as long as they are all at the "root" of the crate.
