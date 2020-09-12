use std::fmt::{self, Formatter};

use once_cell::sync::OnceCell;
use sharded_slab::{Guard, Slab};

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
        CONSTANTS
            .set(slab)
            .unwrap_or_else(|_| panic!("Constant pool was already initialized"));
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
    Const(usize),
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            OpCode::Return => write!(f, "{}\n", "Return")?,
            OpCode::Const(key) => {
                let value = Constants::get(*key).unwrap_or_else(|| {
                    panic!("We should always be debugging an instruction with an associated value")
                });
                write!(f, "{} {} {}", "Constant", key, *value)?;
            }
        };

        Ok(())
    }
}

#[derive(Debug)]
pub struct Chunk {
    lines: Vec<(usize, usize)>,
    code: Vec<OpCode>,
    name: String,
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "======={}========\n", self.name)?;
        let mut current = 0;

        for (idx, code) in self.code.iter().enumerate() {
            let line = self.search_line(idx).unwrap_or_else(|| {
                panic!("every bit of code should have a line associated with it")
            });

            if line == current {
                write!(f, "{:05}    | {}\n", idx, code)?;
            } else {
                write!(f, "{:05} {:04} {}\n", idx, line, code)?;
                current = line;
            }
        }

        Ok(())
    }
}

impl Chunk {
    pub fn new(name: &'static str) -> Self {
        Chunk {
            lines: Vec::new(),
            code: Vec::new(),
            name: String::from(name),
        }
    }

    fn add_chunk(&mut self, code: OpCode, line: usize) {
        self.code.push(code);

        if let Some(_) = self.lines.get(line) {
            self.lines[line].1 += 1;
        } else {
            self.lines.push((self.code.len() - 1, 1))
        }
    }

    pub fn add_return(&mut self, line: usize) {
        self.add_chunk(OpCode::Return, line);
    }

    pub fn add_constant(&mut self, value: Value, line: usize) {
        let key = Constants::set(value).unwrap_or_else(|| panic!("Looks like this shard if full"));
        self.add_chunk(OpCode::Const(key), line);
    }

    fn search_line(&self, idx: usize) -> Option<usize> {
        let mut first = 0;
        let mut last = self.lines.len();

        fn check_boundary(bounds: (usize, usize), idx: usize) -> bool {
            bounds.0 <= idx || bounds.0 + bounds.1 >= idx
        }

        while first < last {
            let pivot = (first + last) / 2;
            if check_boundary(self.lines[pivot], idx) {
                return Some(pivot);
            } else if idx < self.lines[pivot].0 {
                last = pivot - 1;
            } else {
                first = pivot + 1;
            }
        }

        None
    }
}
