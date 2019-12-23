use crate::error::Error;
use self::instruction::{ Instruction, VarType };
use self::ops::Ops;

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
    counter: usize,
    position: usize,
    relative_base: i64,
    ops: Ops
}

impl Intcode {
    pub fn from_str(input: &str) -> Result<Intcode,Error> {
        let ops = parse_intcode_ops(input)?;
        Ok(Intcode::new(ops))
    }
    pub fn new(ops: Vec<i64>) -> Intcode {
        Intcode { counter: 0, position: 0, relative_base: 0, ops: Ops::new(ops) }
    }
    pub fn get_op(&self, pos: usize) -> i64 {
        self.ops.get(pos)
    }
    pub fn provide_input(&mut self, input: ProvideInputValue) -> Result<(),Error> {
        if input.provider.counter != self.counter {
            return Err(err!("Input provided to intcode machine twice"))
        }
        self.ops.set(input.provider.pos, input.value);
        self.set_position(self.position + 2);
        Ok(())
    }
    fn set_position(&mut self, val: usize) {
        self.position = val;
        self.counter += 1;
    }
    pub fn step(&mut self) -> Result<Option<Outcome>,Error> {
        loop {
            let pos = self.position;
            let instr = Instruction::new(self.ops.get(pos) as usize);

            match instr {
                Instruction::Add(c,b,a) => {
                    let c = self.get_value(c,1);
                    let b = self.get_value(b,2);
                    let a = self.get_pos(a,3);
                    self.ops.set(a, b + c);
                    self.set_position(self.position + 4);
                },
                Instruction::Mul(c,b,a) => {
                    let c = self.get_value(c,1);
                    let b = self.get_value(b,2);
                    let a = self.get_pos(a,3);
                    self.ops.set(a, b * c);
                    self.set_position(self.position + 4);
                },
                Instruction::Input(c) => {
                    let c = self.get_pos(c,1);
                    // Computation is essentially suspended until
                    // this input provider is given input. If it's dropped
                    // without being given input, we'll be given another
                    // one on the next step to ask again.
                    break Ok(Some(Outcome::NeedsInput(ProvideInput {
                        counter: self.counter,
                        pos: c
                    })))
                },
                Instruction::Output(c) => {
                    let c = self.get_value(c,1);
                    self.set_position(self.position + 2);
                    break Ok(Some(Outcome::Output(c)))
                },
                Instruction::JumpIfTrue(c,b) => {
                    let c = self.get_value(c,1);
                    if c != 0 {
                        let b = self.get_value(b,2);
                        self.set_position(b as usize);
                    } else {
                        self.set_position(self.position + 3);
                    }
                },
                Instruction::JumpIfFalse(c,b) => {
                    let c = self.get_value(c,1);
                    if c == 0 {
                        let b = self.get_value(b,2);
                        self.set_position(b as usize);
                    } else {
                        self.set_position(self.position + 3);
                    }
                },
                Instruction::LessThan(c,b,a) => {
                    let c = self.get_value(c,1);
                    let b = self.get_value(b,2);
                    let a = self.get_pos(a,3);
                    self.ops.set(a, if c < b { 1 } else { 0 });
                    self.set_position(self.position + 4);
                },
                Instruction::Equals(c,b,a) => {
                    let c = self.get_value(c,1);
                    let b = self.get_value(b,2);
                    let a = self.get_pos(a,3);
                    self.ops.set(a, if c == b { 1 } else { 0 });
                    self.set_position(self.position + 4);
                },
                Instruction::AdjustRelativeBase(c) => {
                    let c = self.get_value(c,1);
                    self.relative_base += c;
                    self.set_position(self.position + 2);
                }
                Instruction::Finish => {
                    break Ok(None)
                }
            }
        }
    }
    fn get_pos(&self, ty: VarType, offset: usize) -> usize {
        let position = self.position + offset;
        match ty {
            VarType::Position => self.ops.get(position) as usize,
            VarType::Immediate => position,
            VarType::Relative => (self.ops.get(position) + self.relative_base) as usize
        }
    }
    fn get_value(&self, ty: VarType, offset: usize) -> i64 {
        let pos = self.get_pos(ty, offset);
        self.ops.get(pos)
    }
}

/// An outcome as a result of running a step of the Intcode
/// interpreter. We stop because we either need input or
/// have something to output.
pub enum Outcome {
    NeedsInput(ProvideInput),
    Output(i64)
}

/// This is handed back if the interpreter requires a value.
/// Once given a value, it can be handed back to the interpreter
/// to set the value. A value can only be provided exactly once.
pub struct ProvideInput {
    counter: usize,
    pos: usize
}
impl ProvideInput {
    pub fn value(self, value: i64) -> ProvideInputValue {
        ProvideInputValue { provider: self, value }
    }
}

/// An input provider is turned into this when it's given a value.
/// This can then be given back to the intcode machine to set the value.
pub struct ProvideInputValue {
    provider: ProvideInput,
    value: i64
}

/// Storage for ops that grows as necessary.
mod ops {

    #[derive(Clone)]
    pub struct Ops {
        ops: Vec<i64>
    }

    impl Ops {
        pub fn new(ops: Vec<i64>) -> Ops {
            Ops { ops }
        }
        pub fn get(&self, pos: usize) -> i64 {
            self.ops.get(pos).map(|p| *p).unwrap_or(0)
        }
        pub fn set(&mut self, pos: usize, value: i64) {
            if pos >= self.ops.len() {
                self.ops.resize(pos + 1, 0);
            }
            self.ops[pos] = value;
        }
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
        AdjustRelativeBase(VarType),
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
                9 => Instruction::AdjustRelativeBase(c),
                _ => Instruction::Finish
            }
        }
    }

    #[derive(Clone,Copy,Debug)]
    pub enum VarType {
        Position,
        Immediate,
        Relative
    }

    impl VarType {
        fn new(n: usize) -> VarType {
            match n {
                0 => VarType::Position,
                1 => VarType::Immediate,
                _ => VarType::Relative
            }
        }
    }

}