use std::convert::TryFrom;
use std::fs::{read_to_string};
use std::env::args;

type RegisterIndex = usize;

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    Stop,
    Inc { cell: RegisterIndex },
    Dec { cell: RegisterIndex },
    Goto { cell: RegisterIndex },
    GotoZ { condition_cell: RegisterIndex, goto_cell: RegisterIndex },
}

fn parse_nr(text: &str) -> Result<RegisterIndex, String> {
    text.parse::<RegisterIndex>()
        .map_err(|e| format!("{} is not a number (reason: {:?})", text, e))
}

impl TryFrom<String> for Instruction {
    type Error = String;

    fn try_from(value: String) -> Result<Instruction, Self::Error> {
        let tokens: Vec<_> = value.split(" ").filter(|t| t.len() > 0).collect();
        if tokens.len() > 0 {
            let instruction_token = tokens[0];
            match instruction_token {
                "STOP" => Result::Ok(Instruction::Stop),
                "INC" | "DEC" | "GOTO" => {
                    if tokens.len() == 2 {
                        let cell = parse_nr(tokens[1])?;
                        Result::Ok(match instruction_token {
                            "INC" => Instruction::Inc { cell },
                            "DEC" => Instruction::Dec { cell },
                            "GOTO" => Instruction::Goto { cell },
                            _ => panic!("this should not happen")
                        })
                    } else {
                        Result::Err(format!("Not 2 tokens in: {}", value))
                    }
                }
                "GOTOZ" => {
                    if tokens.len() == 3 {
                        let condition_cell = parse_nr(tokens[1])?;
                        let goto_cell = parse_nr(tokens[2])?;
                        Result::Ok(Instruction::GotoZ { condition_cell, goto_cell })
                    } else {
                        Result::Err(format!("Not 3 tokens in: {}", value))
                    }
                }
                _ => Result::Err(format!("Unknown token: {}", tokens[0]))
            }
        } else {
            Result::Err(format!("No tokens in: {}", value))
        }
    }
}

#[test]
fn test_parse() {
    assert_eq!(Result::Ok(Instruction::Stop), Instruction::try_from("STOP".to_string()));
    assert_eq!(Result::Ok(Instruction::Inc { cell: 42 }), Instruction::try_from("INC 42".to_string()));
    assert_eq!(Result::Ok(Instruction::Dec { cell: 13 }), Instruction::try_from(" DEC 13 ".to_string()));
    assert_eq!(Result::Ok(Instruction::Goto { cell: 0 }), Instruction::try_from(" GOTO  0".to_string()));
    assert_eq!(Result::Ok(Instruction::GotoZ { condition_cell: 42, goto_cell: 0 }), Instruction::try_from("GOTOZ 42 0".to_string()));

    assert!(Instruction::try_from("".to_string()).is_err());
    assert!(Instruction::try_from("INC 1 2 3 ".to_string()).is_err());
    assert!(Instruction::try_from("what is this even".to_string()).is_err());
}

fn parse_commands(text: String) -> Result<Vec<Instruction>, String> {
    let mut result = vec![];
    for (line_nr, line) in text.lines().enumerate() {
        let instruction = Instruction::try_from(line.to_string())
            .map_err(|e| format!("error in line {}: {}", line_nr + 1, e))?;
        result.push(instruction)
    }
    Ok(result)
}

#[test]
fn test_parse_commands() {
    let input = "INC 1
    DEC 2
    GOTO 3
    STOP";
    let expected = vec![
        Instruction::Inc { cell: 1 },
        Instruction::Dec { cell: 2 },
        Instruction::Goto { cell: 3 },
        Instruction::Stop
    ];
    assert_eq!(Result::Ok(expected), parse_commands(input.to_string()));
}

#[derive(Debug)]
struct GotoProgram {
    instructions: Vec<Instruction>
}

#[derive(Debug)]
struct GotoProgramState<'a> {
    program: &'a GotoProgram,
    program_counter: RegisterIndex,
    memory: Vec<RegisterIndex>,
}

impl GotoProgramState<'_> {
    fn run(&mut self) {
        loop {
            match self.program.instructions[self.program_counter] {
                Instruction::Stop => {
                    break;
                }
                Instruction::Inc { cell } => {
                    self.memory[cell] += 1;
                }
                Instruction::Dec { cell } => {
                    self.memory[cell] -= 1;
                }
                Instruction::Goto { cell } => {
                    self.program_counter = cell;
                }
                Instruction::GotoZ { condition_cell, goto_cell } => {
                    if self.memory[condition_cell] == 0 {
                        self.program_counter = goto_cell;
                    }
                }
            }
            self.program_counter += 1;
        }
    }
}

fn main() {
    let args: Vec<_> = args().collect();
    let program_code = read_to_string(&args[1]).expect("Error while reading code");
    let instructions = parse_commands(program_code).expect("Error while parsing code");
    let program = GotoProgram { instructions };
    println!("program = {:?}", program);
    let mut state = GotoProgramState {
        program: &program,
        program_counter: 0,
        memory: vec![0; 10]
    };
    println!("input: {:?}", state.memory);
    state.run();
    println!("result: {:?}", state.memory);
}
