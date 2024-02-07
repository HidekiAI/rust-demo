# Nested lambdas

A simple demonstrations to use for-in-loops with lambdas (as well as lambdas within lambdas) results to step into inner loops and lambdas.

A very nice concept especially for treating `Option<T>::None` as 0 and `Option<T>::Some` as 1 element of an array (more-so, it's actaually [`Option<T>::iter()`](https://doc.rust-lang.org/std/option/enum.Option.html#method.iter))

This sample includes [`map()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map) and [`fold()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold) as well, to represent that you can reduce all the if and match into just 1 line

IMHO, you should learn to make at least `map()` as one of your tools in your toolbox if/when you're iterating collections and/or dealing with `Option<T>` and/or `Result<T>`.  Overall, if you can avoid using `if` statements and/or avoid using `_` in your `match` blocks, you'd generate less buggy code AND you can (most of the time) assume that what you've coded has been valided by the compiler and so, it will do what you've coded (of course, if your intentions mismatched what it was coded, that is a human bug, not a logic bug).

## Demo 1: Loop based

```rust
fn main() {
    let arry1 = vec![42, 86];
    let arry2 = vec![];
    let possible_result = Some(arry1); // Yes I know, why do Option<Vec<T>> when you can assume None to mean empty array and Some to mean array of 1 or more...
    let empty_result: Option<Vec<i32>> = Some(arry2);
    let no_result: Option<Vec<i32>> = None;

    // demo 1: nested for-in-loop based
    // NOTE: foo is read-only, in functional programming, it's a good practice not pass mutable reference
    let my_lambda = |possible_foo: &Option<Vec<i32>>| -> Vec<String> {
        let mut ret: Vec<String> = vec![]; // need to return an instance of string rather than &str
        // this is because Option::iter() (https://doc.rust-lang.org/std/option/enum.Option.html#method.iter)
        // can flatten the Option<T> into 0 or 1 elements
        for result1 in possible_foo.iter() {
            // If here, it means possible_foo is Some<T>
            ret.push("foo is valid...".into());
            for result2 in result1 {
                ret.push(format!("{}", result2).into());
            }
        }
        ret
    };

    // Output:
    // Result 1: ["foo is valid... ", "42 ", "86 "]
    // Result 2: ["foo is valid... "]
    // Result 3: []
    let result1 = my_lambda(&possible_result); // should list the i32s
    let result2 = my_lambda(&empty_result); // should print "foo is valid", but no i32's will be printed
    let result3 = my_lambda(&no_result); // should print "No data available."
    println!("Result 1: {:?}", result1);
    println!("Result 2: {:?}", result2);
    println!("Result 3: {:?}", result3);
}
```

Although (next) Demo 2 is probably the most common approach that developers coming from C{urly language} (C/C++/C#) will deal with mainly because we/they are used to `switch()/case` logic (including `default` as `_`) I wanted to demonstrate this one first to get used to the idea that we are using `Option<T>` and `Result<T>` as iteratable traits in which you'd treat `Some<T>()` and `Ok<T>()` as array of 1 element, while `None<T>()` and `Err<T>()` as 0-element/empty array.

With that in mind, you just need to write ONE logic in which you avoid the `if-if else-else` logic where you'd keep adding more and more conditions later on (imagine the huge block of `switch/case`) and you later realized you forget to add one more `if else` logic during debugging.   All in all, that pattern is error prone in which you leave the block of code to be potentially possible for inroducing future bugs, because another developer or future-self may later insert an `if-else` that may not have been anticipated (how many of us remember writing logic 4 weeks ago and come back trying to remember what our original intentions were, mainly because comments added were stale and does not match the logic anymore, or the variables are meaningless or now means different?)

Speaking of variables no longer match the logic, I still like Hungarian notation such as C++:`auto iMyVar = 3;` or C#:`var iMyVar = 3` or F#:`let iMyVar: int = 3` or Rust:`let i_my_var: i32 = 3;` (note that for both Rust and F#, I am explicitly declaring type but not in C++(auto) and C#(var), mainly because peer-pressure I've had from other workplace - In fact, I am frowned upon for F# implicit type declaration as well).  Truth be told, nobody likes (or uses) Hungarian notation anymore with invention of `auto` and `var` and other dynamic type languages that can implicitly determine the type during compile-time.  Also, we're lazy and want to make sure that if I decide today that it's an `i32`, but maybe tomorrow, it is a `&str`, it makes my Hungarian notated variable no longer an `iMyVar` but now a `strMyVar` and just a pain, so as much as I do like it, I've stopped using it (even the idea of `mMyModule` or `kMyConstant`), but I still implicitly declare my variables for one simple reason...  It prevents static-analyser (and also compiler) to "guess" and when it failed (or gave up) to guess, it'll error on the compiler suggesting that I'd explictly declare it.  Also, IntelliSense/IntelliCode (editor-code-time static analyser) gets confused at times and refuses to complete my code because it cannot guess.  I'm too spoiled on this, and I do not think I could code without it anymore...

Another thing about the practice of nested lambdas and iterations are that what you write will always be used.  The [Iceberg Model](https://ieeexplore.ieee.org/abstract/document/8906782) can be avoided because your intentions is always to get to the final inner loop/lambda result:

```rust
   let possible_some_result_of_many_evaluations =
     lambda1 {
        lambda2 {
            ...
            lambdaN {
                ...final result we want
            }
        }
     }
```

and because of that, all logic will have to be execuated to get there.  There are NO hidden part of the iceberg because it's all there on same block, doing EXACTLY what was intended for THAT specific purpose.  For developers who are used to templates/generics/traits/inheritance/derivations in which we are lazy and rely a lot on reusable code, is very hard to grasp at first, mainly because we start thinking "this logic is so useful, I want to make it into a function/genric/trait/template so that I can use it over and over again" or "hey I've written this logic once before, rather than copy-and-paste from there, I should..." temptation.  In general, I no longer attempt to write logic of "IsA" but have the tendencies to write a "HasA" logic.

To clarify, the problem of sharing or reusing code is that once the logic/code goes onto production, unless it is broken, you should never touch it!  If you shared a function, you suddenly may break the original intention and design in production!  Hence, it's always safer to duplicate the code *IF* the logic has already gone out to production.

Alternatively, for modules/class/crates in which it becomes a shared library (.so, .a, .dll, .lib, etc) I really like the Microsoft way, in which they would create another method with "Ex" (i.e. `Foo(...)` and `FooEx(...)`) so that it is backwards compatible.  If it was an interface/method, another approach is to append the arg list with `Option<T>` so that all previous users just need to add `None` in their args (i.e. `Foo(arg1: i32)` evolves to `Foo(arg1: i32, possible_arg2: Option<&str>, possible_arg3: Option<Vec<i32>>)`).  Of course, if `Foo() -> ()` originally returned void, in which now returns Result (`FooEx() -> Result<i32, String>`) then it breaks a lot, so you should extended with "Ex" instead...

I also like the version based package management of Nuget way, probably, Apache Ant did this way before invention of Nuget, but well, I have more bias towards Nuget (and the smart people at Microsoft) so I speak in Nuget-terms, but one thing I really love is that for each DLL, I can tell Nuget to use differen versions, yet they all can reside on same Solution (.sln) as long as they stay module.  Same goes with Rust `Cargo.toml` (actually, the `.lock` file), in which you can declare which version to lock your project on for each crates you work on.  This version locking is quite important mainly for continous evoluation of Open Source libraries in which, the API could differ each new push and breaks you every day you `git` their latest HEAD...

I appreciate these librarians' hard work, but as a user of their library, I do not have the time to revise my logic every time their API changes (time is money) unless it's for hobby usage (in which case, I usually just drop my whole project and move to new until they stablize :grin:)

OK, ranted a bit too long on something outside the demo intension, apologies...  I'm not an architect by trade, but I am (ebmarassingly) quite opinionated about this...

## Demo 2: match-based

```rust
fn main() {
    let arry1 = vec![42, 86];
    let arry2 = vec![];
    let possible_result = Some(arry1); // Yes I know, why do Option<Vec<T>> when you can assume None to mean empty array and Some to mean array of 1 or more...
    let empty_result: Option<Vec<i32>> = Some(arry2);
    let no_result: Option<Vec<i32>> = None;

    // demo 2: match based
    // trick here is to use return for each lambdas;
    // in this example, we'll return the accumulated log string
    // NOTE: foo is read-only, in functional programming, it's a good practice not pass mutable reference
    let my_lambda2 = |possible_foo: &Option<Vec<i32>>| -> Vec<String> {
        // I love match statement, in which I can write two lamdas for each condition
        // and in this case, only two condtions of either Some<T> or _ (None)
        let ret = match possible_foo {
            Some(foo) => {
                let mut ret: Vec<String> = vec![];
                ret.push("foo is valid...".into());
                for result in foo {
                    ret.push(format!("{}", result).into());
                }
                ret
            }
            None /* can use '_' here instead, but I like explicit*/ => vec!["No data available.".into()],
        };
        ret
    };

    // output:
    // Result 4: ["foo is valid... ", "42 ", "86 "]
    // Result 5: ["foo is valid... "]
    // Result 6: ["No data available."]
    let result4 = my_lambda2(&possible_result); // should list the i32s
    let result5 = my_lambda2(&empty_result); // should print "foo is valid", but no i32's will be printed
    let result6 = my_lambda2(&no_result); // should print "No data available."
    println!("Result 4: {:?}", result4);
    println!("Result 5: {:?}", result5);
    println!("Result 6: {:?}", result6);
}
```

As mentioned on first demo, this is probably the most common way a C/C++/C# developers may approach, mainly because we're used to `switch/case` statement...

In C++, it may look like:

```cpp
auto retvec = std::vector<std::string>();
// Assume possible_foo is a pointer or smartpointer, whatever that presends Option<T> template...
// Although, if I was doing this, I'd just use std::vector<int> in which
// I'd check/test if possible_foo is empty or not...
bool is_some = possible_foo.IsSome();
switch(is_some) {
    case true:  // Option<T>::Some case
    {
        auto fooList = possible_foo.GetValue();
        std::for_each(fooList.begin(), fooList.end(), [](foo) {
            //... setup for converting foo to string, etc...
            auto fooAsString = std::string(foo);
            retvec.push_back(fooAsString);
        })
    }
    case false: // Option<T>::None case
    default:    // same as "_"
    {
        ...
    }
}
```

You get the point, probably won't compile, but that is the gist of C++ based using `switch/case`.  I could probably do it in `if/else` block based, but if I was to really do it with least amount of risk in errors, I'd probably use trinary (`bool ? block1 : block1`) approach to be more explicit that there are only 2 branches.  Using both `switch` and/or `if` based causes the assumptions that there are more than 2 options (i.e. `Some`, `None`, or `NULL`/`null`/`nullptr` (whatta heck does that mean?!?!? - I think it means, it's an Option based variable in which, if it is `NULL`, then it was never initialized :shrug: )).

One BIG difference between Rust `match` and C/C++/C# `switch` statement is that `match` does not require `_` (in newer C++ and C#, I think it requires `default` in the `switch` statement or compiler fails).  The `match` is discriminated union based (whether it is enum, boolean, option), in which, if you only tested for partial, you'd get a compiler error!  Basically, you catch it during compile time, not during runtime;  Realization of "Oh, I forgot to handle the `false` block of my `switch(is_valid)` logic" or "My `default:` logic block is not really my `case false:`" during runtime debugging.  Of course in Rust, the lazy thing to do is use the `_` to catch-all (same as `default` in C/C++/C#) but if I did a code-review in which I saw a `_` in the `match` statement, I'd flag and inquire if the programmer was absolutely sure of it.

Incidentally, that goes the same for the lazy [partial struct updates](https://doc.rust-lang.org/book/ch05-01-defining-structs.html), for example:

```rust
fn main() {
    // the right way, is to be explicit and not be lazy...  assign ALL the elements:
    let user_update_email_keeping_rest_as_original = User {
        active: original_user.active,
        username: original_user.username,
        email: String::from("new_email@example.com"),
        sign_in_count: original_user.sign_in_count,
    };

    // the error-prone way in which 
    let user_update_email_lazy_way = User {
        email: String::from("new_email@example.com"),
        ..original_user // update/copy the rest from original_user
    };
}
```

When a devleoper is used to SQL method of thinking, or to think in terms of "in-place" mentality, the second (lazy) method using the `..` makes it very attractive.  It seems perfectly benign since:
- If any of the variable's type changes, should not be an issue since it's a copy of same type (benign)
- When doing rename-refactor, if the variable name changed from "email" to "e_mail", it should catch it, as well as other variables that is not changed will implicitly be fine (benign)
- If I add, update/rename, or remove any variables that were implicitly updated (other than "email"), it will not affect anything (benign)
- If I am grepping for all logic that references "username", as long as I grep for struct "User" instead of "username", I'll spot it (semi-benign, but possibly error-prone)

I am lazy, so I usually do use the `..` to do partial-struct-update but I do recall (at least twice) getting my butt bitten pretty hard in F# looking for a bug and was not able to spot it, because I was lazy...  And because of that experience (basically, it happened more than once) I have decided to make sure not to do partial update unless I am absolutely sure (or it's a prototype logic or unit-tests, that will never hit production).

## Demo 3: map() and fold() method (my favorite)

```rust
fn main() {
    let arry1 = vec![42, 86];
    let arry2 = vec![];
    let possible_result = Some(arry1); // Yes I know, why do Option<Vec<T>> when you can assume None to mean empty array and Some to mean array of 1 or more...
    let empty_result: Option<Vec<i32>> = Some(arry2);
    let no_result: Option<Vec<i32>> = None;

    // last final method, is with traditional map and fold methods common to functional programming
    // this is by far, the most practical way IMHO to deal with these kinds of problems
    // though, most likely harder to read for developers who's not used to functional programming
    let my_lambda3 = |possible_foo: &Option<Vec<i32>>| -> Vec<String> {
        // Tip: though you can do:
        // return possible_foo.iter().map(|foo| {...
        // by forcing it to a variable, you can check whether you've got the .map() return
        // correctly.  For example, the map() returns as 'Iterator<Vec<String>>' and not 'Vec<String>'
        let ret = possible_foo
            .iter() // again, the wonderful Option.iter() method
            .map(|foo| {
                // if here, we KNOW foo is Some<T>, so make sure we start off with "foo is valid" as our accumulator
                // and then, for each elements in foo, build the accumulator Vec<String>
                // unsure why, but I have to iter() foo, even though we KNOW it's Vec<i32> here...
                foo.iter()
                    .fold(vec!["foo is valid".into()], |mut acc, result| {
                        // Only enters this block if possible_foo is Some
                        acc.push(format!("{}", result));
                        // return accumulator for next iter
                        acc // though, I've always wondered, is this a clone/copy or a a ref?
                    }) // fold
            }); // map
        ret.flatten().collect() // TODO: Investigate why I have to call flatten() here when I just want to .collect()
    };
    // If the above was made into a single line:
    let my_lambda3_single_line = |possible_foo: &Option<Vec<i32>>| -> Vec<String> {
        possible_foo.iter().map(|foo| { foo.iter().fold(vec!["foo is valid".into()], |mut acc, result| { acc.push(format!("{}", result)); acc }) }).flatten().collect() };
    // output:
    // Result 7: ["foo is valid", "42", "86"]
    // Result 8: ["foo is valid"]
    // Result 9: []
    let result7 = my_lambda3(&possible_result); // should list the i32s
    let result8 = my_lambda3(&empty_result); // should print "foo is valid", mainly because acc was initialized that way
    let result9 = my_lambda3(&no_result); // should print empty array (acc is empty)
    println!("Result 7: {:?}", result7);
    println!("Result 8: {:?}", result8);
    println!("Result 9: {:?}", result9);

}
```

Firstly, as a side note, I just want to say that I dislike (OK, maybe more than dislike) reading somebody else's functional programming logic.  I spend more time staring at the comment-less logic with variables `i` and `j` (I've asked few programmers in the past whether they come from [Fortran](https://en.wikipedia.org/wiki/Fortran#FORTRAN_77) (if you don't know the references of where i, j, and k come from there you go) and they usually ignore my sarcasm, or even heard defense on "lambda calculus" is mathmatical, so i, j, k is used) and wonder whatta heck their intentions are.  IMHO when they say that "F" in "F#" is for "Fun(c)" I cringe and wonder if they mean solo-programmer or for team.

OK, well, I'm not as smart as they are so maybe it's an envy thing...  But from a regular programmer, I would ask.. whatta heck is:

```lisp
List a = Empty | Elem a (List a)
```

- [Linked list](https://rust-unofficial.github.io/too-many-lists/first-layout.html?highlight=Elem#basic-data-layout)

or:

```haskell
map:: (a -> b) -> [a] -> [b] 
```

- [map](https://hoogle.haskell.org/?hoogle=map)

and maybe I'll be told to go to YouTube or MSDN Channel9 and watch some Haskell videos :stuck_out_tongue:

In any case, the point is, most functional programming practice is:
- Write once and if compiler isn't angry (error) or hinting it may not be what you've intended (warn), then you're done
- You only need to understand your intention at the time you write the logic, because you never have to come back to look at it ever again if all is good
- You don't write unit-tests, if you quick-tested (i.e. via interpreter terminal window or something) and it gave you result you've expected, that's your quick-test and don't bother checking that quick-test in as unit-test because you can assume you'd never touch it again, so unit-test (which is a very good way that validates that somebody changed your logic or data-model, and your result changed, you expected 5 to always be returned, but it now returns 2, so on) will never happen.
  - Note that I love unit-tests, it helps me think from the user of the API side perspectives and understand how klunky or well the interface (function signatures, etc) is or not, so I write a lot of unit-tests.
  - Note that if the unit-test was written with poor understanding of the design/goals, whether it is for functional or OOP based language, it will get in your way while debugging and/or reimplementing.  Unit-test should be written not for sake of testing how klunky the API feels, but rather, to validate the output is based on the design.

One issue I want to point out that are common in functional programming is type evaluations of functions/lambdas based on input and guessing...

```fsharp
// NOTE: "_" to implicitly declare result type is unnecessary, 
// it's here just to explain that it's typeless until type is 
//implicitly passed
let sum l r : _ = l + r 
// Correct way to delcare is:
//      let sum l r = l + r

let result_int = sum 5 3
let result_float = sum 2.5f 3.3f
```

Neat feature!  Though in C++, you do `template <typename T> T sum(T l, T b){...}` and for Rust, you do `fn sum<T>(l: T, r: T) -> T where T: std::ops::Add<Output=T>,{...}`, it's just that it's just easier to comprehend and straight forward in F# (and other functional languages).  I can probably say, it's more ellegant (for this usage) compared to OOP languages...  (If I was to justify why I LOVE Rust, is that it's got a functional programming aspect with performance of significant amount better than F# - see my quick (probably not too accurate nor fair comparision) [rust vs F#](https://github.com/HidekiAI/mandelbrot-fs/blob/main/mandelbrot_Rust-vs-FSharp.png))

I do not wish to give a wrong (bad) impression about functional programming (or rather, programmers), for I am convinced that functional programming pattern and aspects help significantly reduce runtime bugs, in which, once you release it to production, you never have to touch it again!  Nor do I think bad about OOP programming, it's just a tool and at the current time, my choice of tools are functional oriented until better tools are discovered.

In any case apologies once again for my opinionated rant, but from this demo part, all I'm doing is using `map()` and `fold()` which are (I think) quite common to see in functional programming libraries.  

``` rust
    let my_lambda3_single_line = 
        |possible_foo: &Option<Vec<i32>>| -> Vec<String> {
            possible_foo.iter().map(|foo| { 
                foo.iter().fold(
                    vec!["foo is valid".into()], 
                    |mut acc, result| { 
                        acc.push(format!("{}", result)); 
                        acc 
                    }) 
                }).flatten().collect() };
```

IMHO, the most ellegant way to solve this kind of problems in which, if the compiler says it's good (no errors, no warnings), you probably can assume it'll run without failure on first time.  I've heard programmers claim (more than once) "I've spent X days writing this module without running, each day was just fighting and fixing what the compiler complained, but once compilers cleared all errors, it ran the first time..."  and that isn't something a C++/C#/C programmer can believe;  The first thing we'll claim is "I call B.S. on that statement!  In your wet dreams!"  Although I've never come close to such dreams (mainly because I probably lack the confidence to just cycle around code-compile phase, and I have the tendencies to add `println!()` breadcrumbs to validate whether I'm approaching correctly and have the correct/expected values), I have somewhat come near that dreams once or twice and have to agree that I'm now more reliant on compiler to find my bugs than runtime-debugger with breakpoints and `println!()`.

In fact, the above `map()` and `fold()` code block is the ONLY demo in which I did not have run and verify that output is correct, I really trusted the rust-analyzer to help me guide through the compiler errors on setting up my `fold()` accumulator correctly, or why my `map()` is returning result in which I would have to `flatten()` first prior to `collect()`ing the result.  But once it all compiled, I just trusted that it WILL be the output I'd expect (and it was).

In conclusion, I really think that it's ellegant when I write in functional aspect, but from others who's looking at my logic, they would probably be complaining how ugly and hard to read my logic is, wondering what my intentions were when I wrote it.  That's the impression I get with functional programs, it's just not meant to be a team-based language, in which others have to "suffer" reading your hard-to-read (trace/step-into) logic to runtime-debug.

Again, most functional programmers will claim that once written, we'd never have to look at the logic again, so if I don't have to look at it again, why are you looking at it?  (Answer is simple, I want a job where I can continue to learn from others, and I love to tell others how elegant they've done this part and that.)

My suggestion is that if you can get away with writing a complete black-box where majority of the methods are private, and users of that black-box NEVER have to debug INTO your almost-impossible-to-comprehend code, then use functional all you want...

In the meanwhile, all the functional logics I write, I'll keep bragging that it's ellegant :smirk: :stuck_out_tongue:

## Demo 4: Iterate `Result<T>`

```rust
fn main() {
    let arry1 = vec![42, 86];
    let arry2 = vec![];
    let possible_result = Some(arry1); // Yes I know, why do Option<Vec<T>> when you can assume None to mean empty array and Some to mean array of 1 or more...
    let empty_result: Option<Vec<i32>> = Some(arry2);
    let no_result: Option<Vec<i32>> = None;

    // OK, one last one...  I was thinking about "result" in general last night and I forgot that
    // Result<T>.iter() existed...  So going back to for-in-loop:
    // first, we need to create a separate lambda which will explicitly return Result<T> so that we can
    // utiize it in for-in-loop.
    let is_valid =
        |possible_foo: &Option<Vec<i32>>| -> Result<Vec<i32>, String> {
            match possible_foo {
            Some(foo) => Ok(foo.clone()), // clone() because we're returning a new instance of Vec<i32>
            None => Err("No data available.  In fact, you'll not see this message if Result.iter() is used".into()),   // when Result<T>.iter() is called, Err() WILL BE IGNORED!
        }
        };
    // and then, we can use the is_valid() lambda in the for-in-loop
    let my_lambda4 = |possible_foo: &Option<Vec<i32>>| -> Vec<String> {
        let mut ret: Vec<String> = vec![];
        for results in is_valid(possible_foo).iter() {
            ret.push("foo is valid".into());
            for result in results {
                ret.push(format!("{}", result));
            }
        }
        ret
    };
    // Output:
    // Result 10: ["foo is valid", "42", "86"]
    // Result 11: ["foo is valid"]
    // Result 12: []
    let result10 = my_lambda4(&possible_result); // should list the i32s
    let result11 = my_lambda4(&empty_result); // should print "foo is valid", but no i32's will be printed
    let result12 = my_lambda4(&no_result); // should print "No data available."
    println!("Result 10: {:?}", result10);
    println!("Result 11: {:?}", result11);
    println!("Result 12: {:?}", result12);
}
```

Lastly, this is similar to `Option<T>::iter()` method, in which I'd treat `Result<T>::iter()` as either `Ok` or `Err`, as an iteratable list of either 0 or 1 element.  Mainly to express that ONLY when it's `Ok<T>(result)`, you get to go inside the nest.

Though the sample only presents outer and inner, one can go as deep as one wants as in:

```rust
for result1 in do_func1.iter() {
    for result2 in result1.iter() {
        ...
        for result_y in result_x.iter() {
            ... final result that we really want is here
        }
    }
}
```

Each and every line of logic are "used code" (no iceberg model), and I personally like it better than nested `if` blocks (without the `else`) mainly because there are no room for others to inject an `else` statement behind my back to change the flow of how it gets branched off in unexpected directions.

One caveat I can think of is that it will lack error handling, mainly because it just opts out on any place that returns `Result<T>::Err()`.  But as mentioned elsewhere, if you think of it more like a SQL result "set", where a set is either empty or has one (or more) rows, you then either `JOIN` and/or `SELECT` more row-sets to it and expect the external "thing" to panic/throw/assert, all you have to worry about is whether it returned final result row-set of one ore more rows, or none (empty set).  Treat an empty-set not as an error, but rather, as filter/map-reduce in which when reduced, there were nothing that met the entire condition...

And as a bonus, you DO NOT need to check whether the result row-sets were empty or not because of the nature/characterists of `for-in` (as well as just plain old `for()` and `std::for_each()` loop) because if the list is empty, it'll just not go into the inner block logic!

```rust
// using IF checks...
let result_rowset1 = do_something();
if result_rowset1.len() > 0 {
    // iterate next
    for result2 in result_rowset1 {
        if result3 = let result2.is_some()  {
            ... do something
        }
        else
        {
            ... error handling
        }
    }
}
else
{
    ... some error handling
}


// just use for-in, caveat is no error handling
for result1 in do_something().iter() {
    for result2 in result1.iter()
    {
        for result3 in result2.iter() {
            ... do stuff
        }
    }
}
```

Unless I really need to handle errors (in which case, I'd use `match` without using `_` so that it's more explicit), I will deal with it the 2nd ways for it is much more cleaner and easier to read (and bugless).  Otherwise, I have now started to obey the YAGTNI rule.
