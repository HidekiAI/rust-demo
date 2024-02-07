# Nested lambdas

A simple demonstrations to use for-in-loops with lambdas (as well as lambdas within lambdas) results to step into inner loops and lambdas.

A very nice concept especially for treating ```Option<T>::None``` as 0 and ```Option<T>::Some``` as 1 element of an array (more-so, it's actaually [```Option<T>::iter()```](https://doc.rust-lang.org/std/option/enum.Option.html#method.iter))

This sample includes [```map()```](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map) and [```fold()```](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold) as well, to represent that you can reduce all the if and match into just 1 line

IMHO, you should learn to make at least ```map()``` as one of your tools in your toolbox if/when you're iterating collections and/or dealing with ```Option<T>``` and/or ```Result<T>```.  Overall, if you can avoid using ```if``` statements and/or avoid using ```_``` in your ```match``` blocks, you'd generate less buggy code AND you can (most of the time) assume that what you've coded has been valided by the compiler and so, it will do what you've coded (of course, if your intentions mismatched what it was coded, that is a human bug, not a logic bug).

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

Although (next) Demo 2 is probably the most common approach that developers coming from C{urly language} (C/C++/C#) will deal with mainly because we/they are used to ```switch()/case``` logic (including ```default``` as ```_```) I wanted to demonstrate this one first to get used to the idea that we are using ```Option<T>``` and ```Result<T>``` as iteratable traits in which you'd treate ```Some<T>()``` and ```Ok<T>()``` as array of 1 element, while ```None<T>()``` and ```Err<T>()``` as 0-element/empty array.

With that in mind, you just need to write ONE logic in which you avoid the ```if-if else-else``` logic where you'd keep adding more and more conditions later on (image the huge block of ```switch/case```) and you later realized you forget to add one more ```if else``` logic during debugging.   All in all, that pattern is error prone in which you leave the block of code to be potentially because later updated to introduce bugs that may not have been anticipated (how many of use remember writing logic 4 weeks ago and come back trying to remember what our original intentions were, mainly because comments added were stale and does not match the logic anymore?)

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

and because of that, all logic will have to be execuated to get there.  There are NO hidden part of the iceberg because it's all there on same block, doing EXACTLY what was intended for THAT specific purpose.  For developers who are used to templates/generics/traits in which we are lazy, want to reuse the code is very hard to grasp at first, mainly because we start thinking "this logic is so useful, I want to make it into a function/genric/trait/template so that I can use it over and over again" or "hey I've written this logic once before, rather than copy-and-paste from there, I should..." temptation.

The problem with this is that once the logic/code goes onto production, unless it is broken, you should never touch it!  If you shared a function, you suddenly may break the original intention and design!  Hence, it's always safer to duplicate the code _IF_ the logic has already gone out to production.  Alternatively, for modules/class/crates in which it becomes a shared library (.so, .a, .dll, .lib, etc) I really like the Microsoft way, in which they would create another method with "Ex" (i.e. ```Foo(...)``` and ```FooEx(...)```) so that it is backwards compatible.  If it was an interface/method, another approach is to append the arg list with ```Option<T>``` so that all previous users just need to add ```None``` in their args (i.e. ```Foo(arg1: i32)``` evolves to ```Foo(arg1: i32, possible_arg2: Option<&str>, possible_arg3: Option<Vec<i32>>)```).  Of course, if ```Foo() -> ()``` originally returned void, in which now returns Result (```FooEx() -> Result<i32, String>```) then it breaks a lot, so you should extended with "Ex" instead...

OK, ranted a bit too long on something outside the demo intension, apologies...  I'm not an architect by trade, but I am opinionated about this...

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
        possible_foo .iter() .map(|foo| { foo.iter() .fold(vec!["foo is valid".into()], |mut acc, result| { acc.push(format!("{}", result)); acc }) }) .flatten() .collect() };
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

## Demo 4: Iterate ```Result<T>```

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

## Demo X: xyz

```rust
fn main() {
    let arry1 = vec![42, 86];
    let arry2 = vec![];
    let possible_result = Some(arry1); // Yes I know, why do Option<Vec<T>> when you can assume None to mean empty array and Some to mean array of 1 or more...
    let empty_result: Option<Vec<i32>> = Some(arry2);
    let no_result: Option<Vec<i32>> = None;

}
```
