mod common;

use common::{Chunk, Value};

fn main() {
    // initialize constant pool
    common::Constants::init();

    let mut chunk = Chunk::new("test");
    chunk.add_constant(Value(19.0), 1);
    chunk.add_return(2);

    println!("We got chunks:\n {}", chunk);
}
