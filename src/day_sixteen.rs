use std::iter::once;
use lazy_static::lazy_static;
use regex::Regex;

use crate::utils::read_lines;

struct SampleInput {
    before_registers_string: String,
    instruction_string: String,
    after_registers_string: String
}

struct Sample {
    before_registers: Vec<i64>,
    instruction: Vec<i64>,
    after_registers: Vec<i64>
}

impl Sample {
    fn from_input(sample_input: &SampleInput) -> Sample {
        let before_registers = get_before_registers_values(&sample_input.before_registers_string);
        let instruction = get_space_delimited_nums(&sample_input.instruction_string);
        let after_registers = get_after_registers_values(&sample_input.after_registers_string);
        Sample {
            before_registers,
            instruction,
            after_registers
        }
    }
}

trait Instruction {
    fn get_registers_values(&self, original_registers: &[i64], instruction: &[i64]) -> Vec<i64>;
}

struct AddRegister;

impl Instruction for AddRegister {
    fn get_registers_values(&self, original_registers: &[i64], instruction: &[i64]) -> Vec<i64> {
        let mut registers: Vec<i64> = original_registers.to_vec();
        registers[instruction[3] as usize] = registers[instruction[1] as usize] + registers[instruction[2] as usize];
        registers
    }
}

struct AddImmediate;

impl Instruction for AddImmediate {
    fn get_registers_values(&self, original_registers: &[i64], instruction: &[i64]) -> Vec<i64> {
        let mut registers: Vec<i64> = original_registers.to_vec();
        registers[instruction[3] as usize] = registers[instruction[1] as usize] + instruction[2];
        registers
    }
}

struct MultiplyRegister;

impl Instruction for MultiplyRegister {
    fn get_registers_values(&self, original_registers: &[i64], instruction: &[i64]) -> Vec<i64> {
        let mut registers: Vec<i64> = original_registers.to_vec();
        registers[instruction[3] as usize] = registers[instruction[1] as usize] * registers[instruction[2] as usize];
        registers
    }
}

struct MultiplyImmediate;

impl Instruction for MultiplyImmediate {
    fn get_registers_values(&self, original_registers: &[i64], instruction: &[i64]) -> Vec<i64> {
        let mut registers: Vec<i64> = original_registers.to_vec();
        registers[instruction[3] as usize] = registers[instruction[1] as usize] * instruction[2];
        registers
    }
}

struct BitwiseAndRegister;

impl Instruction for BitwiseAndRegister {
    fn get_registers_values(&self, original_registers: &[i64], instruction: &[i64]) -> Vec<i64> {
        let mut registers: Vec<i64> = original_registers.to_vec();
        registers[instruction[3] as usize] = registers[instruction[1] as usize] & registers[instruction[2] as usize];
        registers
    }
}

struct BitwiseAndImmediate;

impl Instruction for BitwiseAndImmediate {
    fn get_registers_values(&self, original_registers: &[i64], instruction: &[i64]) -> Vec<i64> {
        let mut registers: Vec<i64> = original_registers.to_vec();
        registers[instruction[3] as usize] = registers[instruction[1] as usize] & instruction[2];
        registers
    }
}

struct BitwiseOrRegister;

impl Instruction for BitwiseOrRegister {
    fn get_registers_values(&self, original_registers: &[i64], instruction: &[i64]) -> Vec<i64> {
        let mut registers: Vec<i64> = original_registers.to_vec();
        registers[instruction[3] as usize] = registers[instruction[1] as usize] | registers[instruction[2] as usize];
        registers
    }
}

struct BitwiseOrImmediate;

impl Instruction for BitwiseOrImmediate {
    fn get_registers_values(&self, original_registers: &[i64], instruction: &[i64]) -> Vec<i64> {
        let mut registers: Vec<i64> = original_registers.to_vec();
        registers[instruction[3] as usize] = registers[instruction[1] as usize] | instruction[2];
        registers
    }
}

struct SetRegister;

impl Instruction for SetRegister {
    fn get_registers_values(&self, original_registers: &[i64], instruction: &[i64]) -> Vec<i64> {
        let mut registers: Vec<i64> = original_registers.to_vec();
        registers[instruction[3] as usize] = registers[instruction[1] as usize];
        registers
    }
}

struct SetImmediate;

impl Instruction for SetImmediate {
    fn get_registers_values(&self, original_registers: &[i64], instruction: &[i64]) -> Vec<i64> {
        let mut registers: Vec<i64> = original_registers.to_vec();
        registers[instruction[3] as usize] = instruction[1];
        registers
    }
}

struct GreaterImmediateRegister;

impl Instruction for GreaterImmediateRegister {
    fn get_registers_values(&self, original_registers: &[i64], instruction: &[i64]) -> Vec<i64> {
        let mut registers: Vec<i64> = original_registers.to_vec();
        let first = instruction[1];
        let second = registers[instruction[2] as usize];
        registers[instruction[3] as usize] = if first > second {
            1
        } else {
            0
        };
        registers
    }
}


struct GreaterRegisterImmediate;

impl Instruction for GreaterRegisterImmediate {
    fn get_registers_values(&self, original_registers: &[i64], instruction: &[i64]) -> Vec<i64> {
        let mut registers: Vec<i64> = original_registers.to_vec();
        let first = registers[instruction[1] as usize];
        let second = instruction[2];
        registers[instruction[3] as usize] = if first > second {
            1
        } else {
            0
        };
        registers
    }
}

struct GreaterRegisterRegister;

impl Instruction for GreaterRegisterRegister {
    fn get_registers_values(&self, original_registers: &[i64], instruction: &[i64]) -> Vec<i64> {
        let mut registers: Vec<i64> = original_registers.to_vec();
        let first = registers[instruction[1] as usize];
        let second = registers[instruction[2] as usize];
        registers[instruction[3] as usize] = if first > second {
            1
        } else {
            0
        };
        registers
    }
}

struct EqualImmediateRegister;

impl Instruction for EqualImmediateRegister {
    fn get_registers_values(&self, original_registers: &[i64], instruction: &[i64]) -> Vec<i64> {
        let mut registers: Vec<i64> = original_registers.to_vec();
        let first = instruction[1];
        let second = registers[instruction[2] as usize];
        registers[instruction[3] as usize] = if first == second {
            1
        } else {
            0
        };
        registers
    }
}


struct EqualRegisterImmediate;

impl Instruction for EqualRegisterImmediate {
    fn get_registers_values(&self, original_registers: &[i64], instruction: &[i64]) -> Vec<i64> {
        let mut registers: Vec<i64> = original_registers.to_vec();
        let first = registers[instruction[1] as usize];
        let second = instruction[2];
        registers[instruction[3] as usize] = if first == second {
            1
        } else {
            0
        };
        registers
    }
}

struct EqualRegisterRegister;

impl Instruction for EqualRegisterRegister {
    fn get_registers_values(&self, original_registers: &[i64], instruction: &[i64]) -> Vec<i64> {
        let mut registers: Vec<i64> = original_registers.to_vec();
        let first = registers[instruction[1] as usize];
        let second = registers[instruction[2] as usize];
        registers[instruction[3] as usize] = if first == second {
            1
        } else {
            0
        };
        registers
    }
}

fn get_sample_inputs(lines: &Vec<String>) -> Vec<SampleInput> {
    let positions = lines.iter()
        .map(|string| string.as_str())
        .chain(once(""))
        .enumerate()
        .filter(|&(_, string)| string.is_empty())
        .map(|(index, _)| index);
    positions
        .map(|index| {
            SampleInput {
                before_registers_string: lines[index - 3].clone(),
                instruction_string: lines[index - 2].clone(),
                after_registers_string: lines[index - 1].clone()
            }
        })
        .collect()
}

fn get_comma_delimited_nums(string: &str) -> Vec<i64> {
    lazy_static! {
        static ref COMMA_NUM: Regex = Regex::new(r"(\d+)(?:, )?").unwrap();
    }
    COMMA_NUM.captures_iter(string)
        .map(|capture| capture[1].parse::<i64>().unwrap())
        .collect()
}

fn get_space_delimited_nums(string: &str) -> Vec<i64> {
    lazy_static! {
        static ref SPACE_NUM: Regex = Regex::new(r"(\d+) ?").unwrap();
    }
    SPACE_NUM.captures_iter(string)
        .map(|capture| capture[1].parse::<i64>().unwrap())
        .collect()
}

fn get_before_registers_values(string: &str) -> Vec<i64> {
    lazy_static! {
        static ref BEFORE_REGEX: Regex = Regex::new(r"^Before: \[(.+)\]$").unwrap();
    }
    let comma_delimited_nums_str = &BEFORE_REGEX.captures(string).unwrap()[1];
    get_comma_delimited_nums(comma_delimited_nums_str)
}

fn get_after_registers_values(string: &str) -> Vec<i64> {
    lazy_static! {
        static ref BEFORE_REGEX: Regex = Regex::new(r"^After:  \[(.+)\]$").unwrap();
    }
    let comma_delimited_nums_str = &BEFORE_REGEX.captures(string).unwrap()[1];
    get_comma_delimited_nums(comma_delimited_nums_str)
}

fn get_samples(lines: &Vec<String>) -> Vec<Sample> {
    let samples_inputs = get_sample_inputs(lines);
    samples_inputs.iter()
        .map(|sample_input| Sample::from_input(sample_input))
        .collect()
}

fn is_instruction_matching_sample(sample: &Sample, instruction: &Box<dyn Instruction>) -> bool {
    let instruction_result = instruction.get_registers_values(&sample.before_registers, &sample.instruction);
    instruction_result == sample.after_registers
}

fn get_num_of_instructions_matching_samples(sample: &Sample, instructions: &Vec<Box<dyn Instruction>>) -> usize {
    instructions.iter()
        .filter(|&instruction| is_instruction_matching_sample(sample, instruction))
        .count()
}

pub fn solve_part_one() {
    let instructions: Vec<Box<dyn Instruction>> = vec![
        Box::new(AddRegister),
        Box::new(AddImmediate),
        Box::new(MultiplyRegister),
        Box::new(MultiplyImmediate),
        Box::new(BitwiseAndRegister),
        Box::new(BitwiseAndImmediate),
        Box::new(BitwiseOrRegister),
        Box::new(BitwiseOrImmediate),
        Box::new(SetRegister),
        Box::new(SetImmediate),
        Box::new(GreaterImmediateRegister),
        Box::new(GreaterRegisterImmediate),
        Box::new(GreaterRegisterRegister),
        Box::new(EqualImmediateRegister),
        Box::new(EqualRegisterImmediate),
        Box::new(EqualRegisterRegister)
    ];
    let lines = read_lines("day_sixteen.txt");
    let samples = get_samples(&lines);
    let answer = samples.iter()
        .map(|sample| get_num_of_instructions_matching_samples(sample, &instructions))
        .filter(|&count| count >= 3)
        .count();
    println!("{}", answer);
}