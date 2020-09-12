use std::fmt::{self, Formatter};

use once_cell::sync::OnceCell;
use sharded_slab::{ Slab, Guard };

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub struct Value(pub f64);

impl fmt::Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

static CONSTANTS: OnceCell<Slab<Value>> = OnceCell::new();

pub struct Constants;

impl Constants {
    pub fn init() {
        let slab = Slab::new();
        CONSTANTS.set(slab).unwrap_or_else(|_| panic!("Constant pool was already initialized"));
    }

    pub fn get(key: usize) -> Option<Guard<'static, Value>> {
        CONSTANTS.get().and_then(|slab| slab.get(key))
    }

    pub fn set(value: Value) -> Option<usize> {
        CONSTANTS.get().and_then(|slab| slab.insert(value))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    Return,
    Const(usize)
}

#[derive(Debug)]
pub struct Chunk {
    code: Vec<OpCode>,
    name: String
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "======={}========\n", self.name)?;

        for code in self.code.iter() {
            match code {
                OpCode::Return => write!(f, "{:?}\n", code)?,
                OpCode::Const(key) => {
                    let value = Constants::get(*key).unwrap_or_else(|| panic!("We should always be debugging an instruction with an associated value"));
                    write!(f, "{} {}", "Constant", *value)?;
                }
            }
        }

        Ok(())
    }
}


impl Chunk {
    pub fn new(name: &'static str) -> Self {
        Chunk {
            code: Vec::new(),
            name: String::from(name)
        }
    }

    fn add_chunk(&mut self, code: OpCode) {
        self.code.push(code)
    }

    pub fn add_return(&mut self) {
        self.add_chunk(OpCode::Return);
    }

    pub fn add_constant(&mut self, value: Value) {
        let key = Constants::set(value).unwrap_or_else(|| panic!("Looks like this shard if full"));
        self.add_chunk(OpCode::Const(key));
    }
}
