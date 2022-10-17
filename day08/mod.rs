use std::str::FromStr;
use std::collections::HashSet;

pub fn part_one(input: &str) -> i64 {
    let mut console: GameConsole = GameConsole::init();

    for (index, line) in input.trim().split('\n').enumerate() {
        let instr: Vec<&str> = line.split(' ').collect();
        console.program[index] = Instruction::new(instr[0], instr[1]);
    }

    while !console.is_loop() {
        console.run_instruction();
    }

    println!("day08 -> part one: {}", console.acc);
    return console.acc;
}
pub fn part_two(input: &str) -> i64 {
    return 0;
}

#[derive(Debug)]
struct GameConsole {
    program: [Instruction; 1024],
    ip: usize, // instruction pointer
    acc: i64,  // accumulator
    visited_ip: HashSet<usize>
}

impl GameConsole {
    fn init() -> Self {
        GameConsole {
            program: [Instruction::new("nop", "0"); 1024],
            ip: 0,
            acc: 0,
            visited_ip: HashSet::new()
        }
    }
    fn run_instruction(&mut self) {
        self.visited_ip.insert(self.ip);
        match self.program[self.ip].operation {
            OpCode::Nop => {
                self.ip += 1;
            }
            OpCode::Acc => {
                self.acc += self.current_instr_val();
                self.ip  += 1;
            }
            OpCode::Jmp => {
                let jump_offset: usize = self.current_instr_val().abs().try_into().unwrap();
                if self.current_instr_val() > 0 {
                    self.ip = self.ip.checked_add(jump_offset).unwrap();
                } else {
                    self.ip = self.ip.checked_sub(jump_offset).unwrap();
                }
            }
        }
    }
    fn is_loop(&self) -> bool {
        return self.visited_ip.contains(&self.ip);
    } 
    fn current_instr_val(&self) -> i64 {
        return self.program[self.ip].value;
    }
}
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
struct Instruction {
    operation: OpCode,
    value: i64
}

impl Instruction {
    fn new(op_code: &str, value: &str) -> Instruction {
        Instruction {
            operation: OpCode::from_str(op_code).unwrap(),
            value: value.parse::<i64>().unwrap()
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum OpCode {
    Nop,
    Acc,
    Jmp,
}

impl FromStr for OpCode {
    type Err = ();
    fn from_str(input: &str) -> Result<OpCode, Self::Err> {
        match input {
            "nop" => Ok(OpCode::Nop),
            "acc" => Ok(OpCode::Acc),
            "jmp" => Ok(OpCode::Jmp),
            _     => Err(()), 
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("testinput")), 5);
    }
    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("testinput")), 1);
    }
}
