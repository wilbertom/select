
extern crate csspie;

use std::os::args;
use csspie::tokenizer;

fn main() {
    let arguments = args();

    if arguments.len() == 2 {
        println!("{}", tokenizer::tokenize(arguments[1].clone()));
    } else {
        println!("Usage: csspie <selector>");
    }


}
