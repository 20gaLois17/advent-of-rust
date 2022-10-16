use std::str::FromStr;
pub fn part_one(input: &str) -> i64 {
    let mut console: GameConsole = GameConsole::new();
    println!("{:?}", console);
    return 0;
}
pub fn part_two(input: &str) -> i64 {
    return 0;
}

#[derive(Debug)]
struct GameConsole {
    program: [Instruction; 8],
    ip: usize, // instruction pointer
    acc: i64,
}
impl GameConsole {
    fn new() -> Self {
        GameConsole {
            program: [Instruction::nop(); 8],
            ip: 0,
            acc: 0,
        }
    }
    fn execute_instruction(&mut self) {
        match self.program[self.ip].operation {
            OpCode::Nop => {
                self.ip += 1;
            }
            OpCode::Acc => {
                // TODO: implement
            }
            OpCode::Jmp => {
                // TODO: implement
            }
        }
    }
}
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
struct Instruction {
    operation: OpCode,
    value: i64
}

impl Instruction {
    fn nop() -> Instruction {
        Instruction {
            operation: OpCode::Nop,
            value: 0
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
