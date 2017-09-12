#![feature(attr_literals)]
#![recursion_limit="128"]
//#![feature(collections_range)]
//#![feature(slice_get_slice)]
#![allow(non_camel_case_types)]  /* TODO temporary becaues of pdf_derive */
#![allow(unused_doc_comment)] // /* TODO temporary because of err.rs */
#[macro_use]
extern crate pdf_derive;
#[macro_use]
extern crate error_chain;
extern crate num_traits;
extern crate inflate;
extern crate itertools;
extern crate memmap;
extern crate tuple;
extern crate chrono;

//#[macro_use]
//mod macros;
pub mod parser;
pub mod object;
pub mod types;
pub mod xref;
pub mod primitive;
pub mod stream;
pub mod file;
pub mod backend;

mod err;
// mod content;
mod enc;

// pub use content::*;
pub use err::*;

// hack to use ::pdf::object::Object in the derive
mod pdf {
    pub use super::*;
}

/// Prints the error if it is an Error
pub fn print_err<T>(err: Error) -> T {
    println!("\n === \nError: {}", err);
    for e in err.iter().skip(1) {
        println!("  caused by: {}", e);
    }
    println!(" === \n");

    if let Some(backtrace) = err.backtrace() {
        println!("backtrace: {:?}", backtrace);
    }

    println!(" === \n");
    panic!("Exiting");
}
