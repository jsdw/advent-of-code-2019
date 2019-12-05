use crate::error::Error;

pub fn part1(input: &str) -> Result<(),Error> {
    let ops = parse_input(input)?;
    Ok(())
}

pub fn part2(input: &str) -> Result<(),Error> {

    Ok(())
}

fn parse_input(input: &str) -> Result<Vec<i64>,Error> {
    let mut ns = vec![];
    for (idx,s) in input.split(",").enumerate() {
        let n = s
            .trim()
            .parse()
            .map_err(|e| err!("Cannot parse entry {} ('{}') into an integer", idx+1, s))?;
        ns.push(n)
    }
    Ok(ns)
}

mod intcode {

    use crate::error::Error;
    use crate::utils::digits;
    use self::instruction::Instruction;

    pub struct Intcode {
        position: usize,
    }

    impl Intcode {
        pub fn new() -> Intcode {
            Intcode { position: 0 }
        }
        pub fn step(&mut self, ops: &mut Vec<i64>) -> Result<Outcome,Error> {
            let pos = self.position;
            let instr = if let Some(&pos) = ops.get(pos) {
                Instruction::new(ops[pos as usize] as usize)
            } else {
                return Ok(Outcome::Finished)
            };

            match instr {
                Instruction::Add(c,b,a) => {
                    let c = c.get_pos(ops, pos+1).ok_or_else(pos_err(pos+1))?;
                    let b = b.get_pos(ops, pos+2).ok_or_else(pos_err(pos+2))?;
                    let a = a.get_pos(ops, pos+3)
                }
            }

        }
    }

    pub enum Outcome<'a> {
        Finished,
        NeedsInput(ProvideInput<'a>),
        Output(i64)
    }

    pub struct ProvideInput<'a> {
        ops: &'a mut Vec<i64>
    }

    fn pos_err(pos: usize) -> impl FnOnce() -> Error {
        || err!("can't get value at index {} in ops", pos+1)
    }

    mod instruction {

        pub enum Instruction {
            Add(VarType,VarType,VarType),
            Mul(VarType,VarType,VarType),
            Input(VarType),
            Output(VarType),
            Finish
        }

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
            pub fn get_from
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
                    _ => Instruction::Finish
                }
            }
        }

    }
}