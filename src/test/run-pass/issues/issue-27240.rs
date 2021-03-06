// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// run-pass
#![allow(unused_assignments)]
#![allow(unused_variables)]
use std::fmt;
struct NoisyDrop<T: fmt::Debug>(T);
impl<T: fmt::Debug> Drop for NoisyDrop<T> {
    fn drop(&mut self) {}
}

struct Bar<T: fmt::Debug>([*const NoisyDrop<T>; 2]);

fn fine() {
    let (u,b);
    u = vec![43];
    b = Bar([&NoisyDrop(&u), &NoisyDrop(&u)]);
}

struct Bar2<T: fmt::Debug>(*const NoisyDrop<T>, *const NoisyDrop<T>);

fn lolwut() {
    let (u,v);
    u = vec![43];
    v = Bar2(&NoisyDrop(&u), &NoisyDrop(&u));
}

fn main() { fine(); lolwut() }
