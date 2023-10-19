// run-pass
// needs-unwind


#![feature(generators, generator_trait)]

use std::ops::{CoroutineState, Coroutine};
use std::pin::Pin;
use std::panic;

fn main() {
    let mut foo = || {
        if true {
            return
        }
        yield;
    };

    match Pin::new(&mut foo).resume(()) {
        CoroutineState::Complete(()) => {}
        s => panic!("bad state: {:?}", s),
    }

    match panic::catch_unwind(move || Pin::new(&mut foo).resume(())) {
        Ok(_) => panic!("generator successfully resumed"),
        Err(_) => {}
    }
}
