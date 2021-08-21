## What is this
This is an example of why adding trait method with a default implementation is still considered a breaking change.

(Disclaimer: I'm not a Rust expert by any means, and this was created only to present an example
for a book I was reading that mentioned otherwise)

## How to use
Notice how in [crate2/src/lib.rs](crate2/src/lib.rs), `crate2`, which depends on `crate1`, defines a trait `OtherTrait` that has a method `func2`.
`crate2` also creates a new struct `S` that implements both `OtherTrait` and a trait from `crate1` called `MyTrait` that has a method `func1`.

Finally, `crate2` defines a function called `my_func` that calls `s.func2()`, and since `S` implements `OtherTrait` this call is unambiguous.

However, if `crate1` adds a method called `func2` (even if it has a default implementation), this causes `crate2` to stop compiling.

To see this, un comment lines 7-9 on [crate1/src/lib.rs](crate1/src/lib.rs) and try to compile `crate2`
