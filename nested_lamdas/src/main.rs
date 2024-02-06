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
    let arry1 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let arry2 = [11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    let possible_result = Some(vec![42, 86]);   // Yes I know, why do Option<Vec<T>> when you can assume None to mean empty array and Some to mean array of 1 or more...
    let empty_result: Option<Vec<i32>> = Some(vec![]);
    let no_result: Option<Vec<i32>> = None;

    // demo 1: loop based
    // NOTE: foo is read-only, in functional programming, it's a good practice not pass mutable reference
    let my_lambda = |possible_foo: &Option<Vec<i32>>| {
        if let Some(foo) = possible_foo {
            print!("foo is valid...");
            // Only enters this block if possible_foo is Some
            for result in foo {
                // ... do something with result
                print!("{} ", result);
            }
            println!();
        } else {
            // possible_foo is None, so no need to enter the loop
            println!("No data available.");
        }
    };

    // Output:
    // foo is valid...42 86
    // foo is valid...
    // No data available.
    let result1 = my_lambda(&possible_result);   // should list the i32s
    let result2 = my_lambda(&empty_result);       // should print "foo is valid", but no i32's will be printed
    let result3 = my_lambda(&no_result); // should print "No data available."


    // demo 2: lambda based
    // trick here is to use return for each lambdas;
    // in this example, we'll return the accumulated log string
    // NOTE: foo is read-only, in functional programming, it's a good practice not pass mutable reference
    let my_lambda2 = |possible_foo: &Option<Vec<i32>>| -> Vec<String> {
        return if let Some(foo) = possible_foo {
            let mut log = vec![];
            log.push(format!("foo is valid... "));
            // Only enters this block if possible_foo is Some
            for result in foo {
                // ... do something with result
                log.push(format!("{} ", result));
            }
            log
        } else {
            // possible_foo is None, so no need to enter the loop
            vec![String::from("No data available.")]
        }
    };

    // output: 
    // Result 4: ["foo is valid... ", "42 ", "86 "]
    // Result 5: ["foo is valid... "]
    // Result 6: ["No data available."]
    let result4 = my_lambda2(&possible_result);   // should list the i32s
    let result5 = my_lambda2(&empty_result);       // should print "foo is valid", but no i32's will be printed
    let result6 = my_lambda2(&no_result); // should print "No data available."
    println!("Result 4: {:?}", result4);
    println!("Result 5: {:?}", result5);
    println!("Result 6: {:?}", result6);
}
