#![feature(generators, generator_trait)]

use std::ops::Coroutine;

fn foo() -> impl Coroutine<Return = i32> {
    //~^ ERROR type mismatch
    || {
        if false {
            return Ok(6);
        }

        yield ();

        5 //~ ERROR mismatched types [E0308]
    }
}

fn main() {}
