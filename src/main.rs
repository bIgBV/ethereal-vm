mod common;

use common::{Chunk, Value};

fn main() {
    // initialize constant pool
    common::Constants::init();

    let mut chunk = Chunk::new("test");
    chunk.add_return();
    chunk.add_constant(Value(19.0));

    println!("We got chunks:\n {}", chunk);
}
