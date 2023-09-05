use regex::Regex;
use utils::file_reader::FileReader;

#[derive(PartialEq, Eq)]
enum InstructionType {
    HLF,
    TPL,
    INC,
    JMP,
    JIE,
    JIO,
}

enum Register {
    A,
    B,
}

struct Instruction {
    instruction_type: InstructionType,
    register: Option<Register>,
    offset: Option<i32>,
}

fn main() {
    let re = Regex::new(
        r"^(hlf) (a|b)|(tpl) (a|b)|(inc) (a|b)|(jmp) (\+|-)(\d+)|(jie) (a|b), (\+|-)(\d+)|(jio) (a|b), (\+|-)(\d+)$",
    )
    .unwrap();

    let mut instructions = vec![];
    // process input to put all distances in the map "distances"
    FileReader::process_lines("./input/problem_23.txt", &mut |line| {
        if !re.is_match(line) {
            panic!("Invalid instruction {}", line);
        }

        let instruction_type = match &line[0..3] {
            "hlf" => InstructionType::HLF,
            "tpl" => InstructionType::TPL,
            "inc" => InstructionType::INC,
            "jmp" => InstructionType::JMP,
            "jie" => InstructionType::JIE,
            "jio" => InstructionType::JIO,
            _ => unreachable!("Unknown instruction type: {}", &line[0..3]), // SHOULDNT HAPPEN BECAUSE LINE MATCHES REGEX
        };

        let register = if instruction_type == InstructionType::JMP {
            None
        } else {
            Some(match &line[4..5] {
                "a" => Register::A,
                "b" => Register::B,
                _ => unreachable!("Unknown register: {}", &line[4..5]), // SHOULDNT HAPPEN BECAUSE LINE MATCHES REGEX
            })
        };

        let offset = match instruction_type {
            InstructionType::JMP => Some(offset_str_to_offset(&line[4..])),
            InstructionType::JIO | InstructionType::JIE => Some(offset_str_to_offset(&line[7..])),
            _ => None,
        };

        let instruction = Instruction {
            instruction_type,
            offset,
            register,
        };

        instructions.push(instruction);
    });

    let mut instruction_ptr = 0;

    let mut registers = [0, 0];

    while instruction_ptr < instructions.len() as i32 {
        let instruction = &instructions[instruction_ptr as usize];

        match instruction.instruction_type {
            InstructionType::HLF => {
                let register = instruction.register.as_ref().unwrap();
                apply_to_register(&mut registers, register, |x| x / 2);

                instruction_ptr += 1;
            }
            InstructionType::TPL => {
                let register = instruction.register.as_ref().unwrap();
                apply_to_register(&mut registers, register, |x| x * 3);

                instruction_ptr += 1;
            }
            InstructionType::INC => {
                let register = instruction.register.as_ref().unwrap();
                apply_to_register(&mut registers, register, |x| x + 1);

                instruction_ptr += 1;
            }
            InstructionType::JMP => {
                let offset = instruction.offset.unwrap();
                instruction_ptr += offset;
            }
            InstructionType::JIE => {
                let register = instruction.register.as_ref().unwrap();
                let register_value = registers[register_to_index(register) as usize];

                if register_value % 2 == 0 {
                    let offset = instruction.offset.unwrap();
                    instruction_ptr += offset;
                } else {
                    instruction_ptr += 1;
                }
            }
            InstructionType::JIO => {
                let register = instruction.register.as_ref().unwrap();
                let register_value = registers[register_to_index(register) as usize];

                if register_value == 1 {
                    let offset = instruction.offset.unwrap();
                    instruction_ptr += offset;
                } else {
                    instruction_ptr += 1;
                }
            }
        }
    }

    println!("Value of b in the end: {}", registers[1]); // 307
}

fn offset_str_to_offset(offset_str: &str) -> i32 {
    let size: i32 = offset_str[1..].parse().unwrap();

    let direction_str = &offset_str[0..1];
    match direction_str {
        "+" => size,
        "-" => -size,
        _ => unreachable!("Unknown offset direction: {}", direction_str), // SHOULDNT HAPPEN BECAUSE LINE MATCHES REGEX
    }
}

fn register_to_index(register: &Register) -> i32 {
    match register {
        Register::A => 0,
        Register::B => 1,
    }
}

fn apply_to_register(registers: &mut [i32; 2], register: &Register, cb: impl FnOnce(i32) -> i32) {
    let index = register_to_index(register);
    registers[index as usize] = cb(registers[index as usize]);
}
