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

    let part_1_result = apply_all_instructions([0, 0], &instructions);
    let part_2_result = apply_all_instructions([1, 0], &instructions);

    println!("Value of b in the end: {}", part_1_result); // 307
    println!("Value of b in the end: {}", part_2_result); // 160
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

fn apply_all_instructions(mut registers: [i32; 2], instructions: &Vec<Instruction>) -> i32 {
    let mut instruction_ptr = 0;

    while instruction_ptr < instructions.len() as i32 {
        let instruction = &instructions[instruction_ptr as usize];

        match instruction.instruction_type {
            InstructionType::HLF => {
                apply_to_register(&mut registers, instruction, &mut instruction_ptr, |x| x / 2);
            }
            InstructionType::TPL => {
                apply_to_register(&mut registers, instruction, &mut instruction_ptr, |x| x * 3);
            }
            InstructionType::INC => {
                apply_to_register(&mut registers, instruction, &mut instruction_ptr, |x| x + 1);
            }
            InstructionType::JMP => {
                let offset = instruction.offset.unwrap();
                instruction_ptr += offset;
            }
            InstructionType::JIE => {
                jump_conditionally(&mut registers, instruction, &mut instruction_ptr, |x| {
                    x % 2 == 0
                });
            }
            InstructionType::JIO => {
                jump_conditionally(&mut registers, instruction, &mut instruction_ptr, |x| {
                    x == 1
                });
            }
        }
    }

    registers[1]
}

fn register_to_index(instruction: &Instruction) -> usize {
    let register = instruction.register.as_ref().unwrap();

    match register {
        Register::A => 0,
        Register::B => 1,
    }
}

fn apply_to_register(
    registers: &mut [i32; 2],
    instruction: &Instruction,
    instruction_ptr: &mut i32,
    cb: impl FnOnce(i32) -> i32,
) {
    let index = register_to_index(instruction);
    registers[index] = cb(registers[index]);

    (*instruction_ptr) += 1;
}

fn jump_conditionally(
    registers: &mut [i32; 2],
    instruction: &Instruction,
    instruction_ptr: &mut i32,
    cb: impl FnOnce(i32) -> bool,
) {
    let register_value = registers[register_to_index(instruction)];

    let offset = if cb(register_value) {
        instruction.offset.unwrap()
    } else {
        1
    };

    (*instruction_ptr) += offset;
}
