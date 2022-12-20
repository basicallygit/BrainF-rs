use std::{env, process::exit};

struct BrainFck {
    memory: [u8; 30000],
    pointer: usize,
    current_instruction: char,
    instructions: Vec<u8>
}

impl BrainFck {
    fn new(instructions: Vec<u8>) -> BrainFck {
        BrainFck {
            memory: [0; 30000], pointer: 0,
            current_instruction: ' ',
            instructions,
        }
    }

    fn run(&mut self) {
        let mut i = 0;
        while i < self.instructions.len() {
            self.current_instruction = self.instructions[i] as char;
            match self.current_instruction {
                '>' => self.pointer += 1,
                '<' => self.pointer -= 1,
                '+' => {
                    if self.memory[self.pointer] == 255 {
                        self.memory[self.pointer] = 0;
                    } else {
                        self.memory[self.pointer] += 1;
                    }
                },
                '-' => {
                    if self.memory[self.pointer] == 0 {
                        self.memory[self.pointer] = 255;
                    } else {
                        self.memory[self.pointer] -= 1;
                    }
                },
                '.' => print!("{}", self.memory[self.pointer] as char),
                ',' => {
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    self.memory[self.pointer] = input.as_bytes()[0];
                },
                '[' => {
                    if self.memory[self.pointer] == 0 {
                        let mut count = 1;
                        while count > 0 {
                            i += 1;
                            if self.instructions[i] as char == '[' {
                                count += 1;
                            } else if self.instructions[i] as char == ']' {
                                count -= 1;
                            }
                        }
                    }
                },
                ']' => {
                    if self.memory[self.pointer] != 0 {
                        let mut count = 1;
                        while count > 0 {
                            i -= 1;
                            if self.instructions[i] as char == '[' {
                                count -= 1;
                            } else if self.instructions[i] as char == ']' {
                                count += 1;
                            }
                        }
                    }
                },
                _ => unreachable!()
            }
            i += 1;
        }
    }
}


fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments, please provide a file to run.");
        exit(1);
    }
    let file = std::fs::read_to_string(&args[1])?;

    let mut instructions = Vec::new();
    for c in file.chars() {
        if c == '>' || c == '<' || c == '+' || c == '-' || c == '.' || c == ',' || c == '[' || c == ']' {
            instructions.push(c as u8);
        }
        //all other characters are interpreted as comments
    }

    let mut bf = BrainFck::new(instructions);
    bf.run();

    Ok(())
}
