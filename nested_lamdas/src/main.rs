// a demonstrations of nested lambdas in Rust in such so that
// what you write is all you want (the "YAGNI" principle)
// * Because they are lambdas, they are all in one place and reduces the
//   issue of switching from one file to another
// * The nesting will assure that inner nests will not be called if
//   conditions are not met, helping to opt out early
// There will be few kinds of demonstration:
// * a for-loop represented similar to how one may do select statement in SQL,
//   in which if the select results to 0 rows, it'll not do the inner select
//   but if the outer select results to 1 or more rows, it'll do the inner select
// * a lambda which based on result, will pass that to another (inner) lambda
//   again, in similar concept as the SQL select statement in which the inner
//   lambda will only get triggered if the outer lambda has a result

fn main() {
    let arry1 = vec![42, 86];
    let arry2 = vec![];
    let possible_result = Some(arry1); // Yes I know, why do Option<Vec<T>> when you can assume None to mean empty array and Some to mean array of 1 or more...
    let empty_result: Option<Vec<i32>> = Some(arry2);
    let no_result: Option<Vec<i32>> = None;

    // demo 1: loop based
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

    // demo 2: lambda based
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

    // last final method, is with traditional map and fold methods common to functional programming
    // this is by far, the most practical way IMHO to deal with these kinds of problems
    // though, most likely harder to read for developers who's not used to functional programming
    let my_lambda3 = |possible_foo: &Option<Vec<i32>>| -> Vec<String> {
        // nice thing about let method for Option, is that you don't have to write the None case

        // Tip: though you can do:
        // return possible_foo.iter().map(|foo| {...
        // by forcing it to a variable, you can check whether you've got the .map() return
        // correctly.  For example, the map() returns as 'Iterator<Vec<String>>' and not 'Vec<String>'
        let ret  = possible_foo
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
    // output:
    // Result 7: ["foo is valid", "42", "86"]
    // Result 8: ["foo is valid"]
    // Result 9: ["No data available."]
    let result7 = my_lambda2(&possible_result); // should list the i32s
    let result8 = my_lambda2(&empty_result); // should print "foo is valid", but no i32's will be printed
    let result9 = my_lambda2(&no_result); // should print "No data available."
    println!("Result 7: {:?}", result7);
    println!("Result 8: {:?}", result8);
    println!("Result 9: {:?}", result9);
}
