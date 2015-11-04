use std::fmt;
use std::cell::RefCell;
use std::io::prelude::*;

thread_local!(pub static __SINK: RefCell<Option<Box<Write>>> = RefCell::new(None));

pub fn __print(args: &fmt::Arguments) {
    __SINK.with(|sink| {
        match *sink.borrow_mut() {
            Some(ref mut s) => s.write_fmt(*args).unwrap(),
            None => print!("{}", args),
        }
    })
}
