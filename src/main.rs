use z3::*;
mod cryptarithmetic;
use cryptarithmetic::maincryptarithmetic;
mod simple_example;
use simple_example::main_simple;

/// Simple test to get a model for a satisfiable problem:
/// x -> y
/// y -> x
fn main() {
    maincryptarithmetic();
}