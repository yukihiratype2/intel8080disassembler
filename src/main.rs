use std::fs;
mod lib;

fn main() {
    let raw = fs::read("./file").expect("E");
    // lib::disassdembler(&source);
    let res = lib::disassembler(&raw);
    println!("{:x?}", res);
}