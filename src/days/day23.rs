use crate::error::Error;
use self::machine::Machine;
use crate::support::intcode::parse_intcode_ops;

pub fn both_parts(input: &str) -> Result<(), Error> {

    let ops = parse_intcode_ops(input)?;

    // Send packets around until we see one sent to 255, then
    // return the Y value of that
    {
        let mut machines: Vec<Machine> = (0..50)
            .map(|address| Machine::boot(ops.clone(), address))
            .collect::<Result<_,_>>()?;
        let y = 'outer1: loop {
            for idx in 0..machines.len() {
                if let Some(packet) = machines[idx].step()? {
                    if packet.address == 255 { break 'outer1 packet.y }
                    machines[packet.address].push_input(packet.x, packet.y);
                }
            }
        };
        println!("Star 1: {}", y);
    }

    // Wait for machines to idle, then send last 255-addressed packet to
    // address 0. Stop when we send the same Y value twice in this way.
    {
        let mut machines: Vec<Machine> = (0..50)
            .map(|address| Machine::boot(ops.clone(), address))
            .collect::<Result<_,_>>()?;
        let mut packet_for_nat = None;
        let mut last_y = None;
        let y = 'outer2: loop {
            for idx in 0..machines.len() {
                if let Some(packet) = machines[idx].step()? {
                    if packet.address == 255 {
                        packet_for_nat = Some(packet)
                    } else {
                        machines[packet.address].push_input(packet.x, packet.y);
                    }
                }
            }
            if machines.iter().all(|m| m.is_idle()) {
                if let Some(packet) = packet_for_nat.take() {
                    if last_y == Some(packet.y) {
                        break 'outer2 last_y
                    } else {
                        last_y = Some(packet.y)
                    }
                    machines[0].push_input(packet.x, packet.y);
                }
            }
        };
        println!("Star 2: {}", y.unwrap());
    }

    Ok(())
}

pub mod machine {

    use std::collections::VecDeque;
    use crate::error::Error;
    use crate::support::intcode::{
        Intcode,
        Outcome as IntcodeOutcome
    };

    pub struct Machine {
        intcode: Intcode,
        input_queue: VecDeque<(i64,i64)>,
        current_input: VecDeque<i64>,
        current_output: Vec<i64>
    }

    impl Machine {
        pub fn boot(ops: Vec<i64>, address: usize) -> Result<Machine,Error> {
            let mut intcode = Intcode::new(ops);
            if let Some(IntcodeOutcome::NeedsInput(p)) = intcode.step()? {
                intcode.provide_input(p.value(address as i64))?;
            } else {
                return Err(err!("Could not boot; did not ask for address"));
            }
            Ok(Machine {
                intcode,
                input_queue: VecDeque::new(),
                current_input: VecDeque::new(),
                current_output: Vec::new()
            })
        }
        pub fn push_input(&mut self, x: i64, y: i64) {
            self.input_queue.push_back((x,y));
        }
        pub fn is_idle(&self) -> bool {
            self.input_queue.len() == 0
            && self.current_input.len() == 0
            && self.current_output.len() == 0
        }
        pub fn step(&mut self) -> Result<Option<Packet>,Error> {
            match self.intcode.step()? {
                Some(IntcodeOutcome::NeedsInput(p)) => {
                    let input = self.get_next_input();
                    self.intcode.provide_input(p.value(input))?;
                    Ok(None)
                },
                Some(IntcodeOutcome::Output(val)) => {
                    self.current_output.push(val);
                    if self.current_output.len() == 3 {
                        let packet = Packet{
                            address: self.current_output[0] as usize,
                            x: self.current_output[1],
                            y: self.current_output[2]
                        };
                        self.current_output.clear();
                        Ok(Some(packet))
                    } else {
                        Ok(None)
                    }
                }
                None => Ok(None)
            }
        }
        fn get_next_input(&mut self) -> i64 {
            if let Some(value) = self.current_input.pop_front() {
                value
            } else if let Some((x,y)) = self.input_queue.pop_front() {
                self.current_input.push_back(y);
                x
            } else {
                -1
            }
        }
    }

    #[derive(Debug,Clone,Copy)]
    pub struct Packet {
        pub address: usize,
        pub x: i64,
        pub y: i64
    }

}