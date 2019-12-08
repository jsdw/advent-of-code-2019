use crate::error::Error;
use self::instruction::{ Instruction, VarType };

pub fn parse_intcode_ops(input: &str) -> Result<Vec<i64>,Error> {
    let mut ns = vec![];
    for (idx,s) in input.split(",").enumerate() {
        let n = s
            .trim()
            .parse()
            .map_err(|_| err!("Cannot parse intcode string op {} ('{}') into an integer", idx+1, s))?;
        ns.push(n)
    }
    Ok(ns)
}

#[derive(Clone)]
pub struct Intcode {
    position: usize,
    ops: Vec<i64>
}

impl Intcode {
    pub fn new(ops: Vec<i64>) -> Intcode {
        Intcode { position: 0, ops }
    }
    pub fn ops(&self) -> &[i64] {
        &self.ops
    }
    pub fn step(&mut self) -> Result<Option<Outcome>,Error> {
        let pos = self.position;
        let instr = if let Some(&pos) = self.ops.get(pos) {
            Instruction::new(pos as usize)
        } else {
            return Err(err!("Out of bound access of position {}", pos));
        };

        match instr {
            Instruction::Add(c,b,a) => {
                let c = self.try_get_value(c, pos+1)?;
                let b = self.try_get_value(b, pos+2)?;
                let a = self.try_get_pos(a, pos+3)?;
                self.ops[a] = b + c;
                self.position += 4;
                Ok(Some(Outcome::StepComplete))
            },
            Instruction::Mul(c,b,a) => {
                let c = self.try_get_value(c, pos+1)?;
                let b = self.try_get_value(b, pos+2)?;
                let a = self.try_get_pos(a, pos+3)?;
                self.ops[a] = b * c;
                self.position += 4;
                Ok(Some(Outcome::StepComplete))
            },
            Instruction::Input(c) => {
                let c = self.try_get_pos(c, pos+1)?;
                // Computation is essentially suspended until
                // this input provider is given input. If it's dropped
                // without being given input, we'll be given another
                // one on the next step to ask again.
                Ok(Some(Outcome::NeedsInput(ProvideInput {
                    intcode: self,
                    pos: c
                })))
            },
            Instruction::Output(c) => {
                self.position += 2;
                let c = self.try_get_value(c, pos+1)?;
                Ok(Some(Outcome::Output(c)))
            },
            Instruction::JumpIfTrue(c,b) => {
                let c = self.try_get_value(c, pos+1)?;
                if c != 0 {
                    let b = self.try_get_value(b, pos+2)?;
                    self.position = b as usize;
                } else {
                    self.position += 3;
                }
                Ok(Some(Outcome::StepComplete))
            },
            Instruction::JumpIfFalse(c,b) => {
                let c = self.try_get_value(c, pos+1)?;
                if c == 0 {
                    let b = self.try_get_value(b, pos+2)?;
                    self.position = b as usize;
                } else {
                    self.position += 3;
                }
                Ok(Some(Outcome::StepComplete))
            },
            Instruction::LessThan(c,b,a) => {
                let c = self.try_get_value(c, pos+1)?;
                let b = self.try_get_value(b, pos+2)?;
                let a = self.try_get_pos(a, pos+3)?;
                self.ops[a] = if c < b { 1 } else { 0 };
                self.position += 4;
                Ok(Some(Outcome::StepComplete))
            },
            Instruction::Equals(c,b,a) => {
                let c = self.try_get_value(c, pos+1)?;
                let b = self.try_get_value(b, pos+2)?;
                let a = self.try_get_pos(a, pos+3)?;
                self.ops[a] = if c == b { 1 } else { 0 };
                self.position += 4;
                Ok(Some(Outcome::StepComplete))
            },
            Instruction::Finish => {
                Ok(None)
            }
        }

    }
    fn try_get_pos(&self, ty: VarType, pos: usize) -> Result<usize,Error> {
        ty.get_pos(&self.ops, pos)
            .ok_or_else(|| err!("can't get value at index {} in ops", pos))
    }
    fn try_get_value(&self, ty: VarType, pos: usize) -> Result<i64,Error> {
        ty.get_value(&self.ops, pos)
            .ok_or_else(|| err!("can't get value at index {} in ops", pos))
    }
}

/// An outcome as a result of running a step of the Intcode
/// interpreter. No outcome is returned if the interpreter
/// is finished.
pub enum Outcome<'a> {
    StepComplete,
    NeedsInput(ProvideInput<'a>),
    Output(i64)
}

/// Sometimes the interpreter will ask for input. It can
/// be provided via this.
pub struct ProvideInput<'a> {
    intcode: &'a mut Intcode,
    pos: usize
}
impl <'a> ProvideInput<'a> {
    pub fn provide(self, value: i64) {
        // Assign the value we asked for:
        self.intcode.ops[self.pos] = value;
        // Finally, progress to the next instruction:
        self.intcode.position += 2;
    }
}

/// This module contains code for parsing the instruction
/// ops, taking into account the mode of each value.
mod instruction {

    #[derive(Clone,Copy,Debug)]
    pub enum Instruction {
        Add(VarType,VarType,VarType),
        Mul(VarType,VarType,VarType),
        Input(VarType),
        Output(VarType),
        JumpIfTrue(VarType,VarType),
        JumpIfFalse(VarType,VarType),
        LessThan(VarType,VarType,VarType),
        Equals(VarType,VarType,VarType),
        Finish
    }

    impl Instruction {
        pub fn new(n: usize) -> Instruction {
            let op = n % 100;
            let c  = VarType::new((n / 100) % 10);
            let b  = VarType::new((n / 1000) % 10);
            let a  = VarType::new((n / 10000) % 10);
            match op {
                1 => Instruction::Add(c,b,a),
                2 => Instruction::Mul(c,b,a),
                3 => Instruction::Input(c),
                4 => Instruction::Output(c),
                5 => Instruction::JumpIfTrue(c,b),
                6 => Instruction::JumpIfFalse(c,b),
                7 => Instruction::LessThan(c,b,a),
                8 => Instruction::Equals(c,b,a),
                _ => Instruction::Finish
            }
        }
    }

    #[derive(Clone,Copy,Debug)]
    pub enum VarType {
        Immediate,
        Position
    }

    impl VarType {
        fn new(n: usize) -> VarType {
            if n == 0 {
                VarType::Position
            } else {
                VarType::Immediate
            }
        }
        pub fn get_pos<'a>(&self, ns: &'a [i64], position: usize) -> Option<usize> {
            match self {
                VarType::Immediate => Some(position),
                VarType::Position => ns.get(position).map(|&p| p as usize)
            }
        }
        pub fn get_value<'a>(&self, ns: &'a [i64], position: usize) -> Option<i64> {
            self.get_pos(ns, position).and_then(|p| ns.get(p)).map(|p| *p)
        }
    }

}