# Nested lambdas

A simple demonstrations to use for-in-loops with lambdas (as well as lambdas within lambdas) results to step into inner loops and lambdas.

A very nice concept especially for treating ```Option<T>::None``` as 0 and ```Option<T>::Some``` as 1 element of an array (more-so, it's actaually [```Option<T>::iter()```](https://doc.rust-lang.org/std/option/enum.Option.html#method.iter))

This sample includes [```map()```](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map) and [```fold()```](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold) as well, to represent that you can reduce all the if and match into just 1 line

IMHO, you should learn to make at least ```map()``` as one of your tools in your toolbox if/when you're iterating collections and/or dealing with ```Option<T>``` and/or ```Result<T>```.  Overall, if you can avoid using ```if``` statements and/or avoid using ```_``` in your ```match``` blocks, you'd generate less buggy code AND you can (most of the time) assume that what you've coded has been valided by the compiler and so, it will do what you've coded (of course, if your intentions mismatched what it was coded, that is a human bug, not a logic bug).
