
extern crate select;

use std::os::args;
use select::tokenizer;

fn main() {
    let arguments = args();

    if arguments.len() == 2 {
        println!("{}", tokenizer::tokenize(arguments[1].clone()));
    } else {
        println!("Usage: csspie <selector>");
    }



}
