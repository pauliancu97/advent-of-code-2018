use crate::{day_sixteen::{AddImmediate, AddRegister, BitwiseAndImmediate, BitwiseAndRegister, BitwiseOrImmediate, BitwiseOrRegister, EqualImmediateRegister, EqualRegisterImmediate, EqualRegisterRegister, GreaterImmediateRegister, GreaterRegisterImmediate, GreaterRegisterRegister, Instruction, MultiplyImmediate, MultiplyRegister, SetImmediate, SetRegister}, utils::read_lines};
use lazy_static::lazy_static;
use regex::Regex;

const ADDR: &str = "addr";
const ADDI: &str = "addi";
const MULR: &str = "mulr";
const MULI: &str = "muli";
const BANR: &str = "banr";
const BANI: &str = "bani";
const BORR: &str = "borr";
const BORI: &str = "bori";
const SETR: &str = "setr";
const SETI: &str = "seti";
const GTIR: &str = "gtir";
const GTRI: &str = "gtri";
const GTRR: &str = "gtrr";
const EQIR: &str = "eqir";
const EQRI: &str = "eqri";
const EQRR: &str = "eqrr";

struct InstructionDescription {
    instruction: Box<dyn Instruction>,
    arguments: Vec<i64>
}

impl InstructionDescription {
    fn from_string(string: &str) -> Option<InstructionDescription> {
        lazy_static! {
            static ref INSTR_REGEX: Regex = Regex::new(r"([a-z]+) (\d+) (\d+) (\d+)").unwrap();
        }
        let captures = INSTR_REGEX.captures(string)?;
        let instruction_str = &captures[1];
        let first_arg = captures[2].parse::<i64>().ok()?;
        let second_arg = captures[3].parse::<i64>().ok()?;
        let third_arg = captures[4].parse::<i64>().ok()?;
        let arguments: Vec<i64> = vec![0, first_arg, second_arg, third_arg];
        let instruction = match instruction_str {
            ADDI => Option::<Box<dyn Instruction>>::Some(Box::new(AddImmediate)),
            ADDR => Option::<Box<dyn Instruction>>::Some(Box::new(AddRegister)),
            MULI => Option::<Box<dyn Instruction>>::Some(Box::new(MultiplyImmediate)),
            MULR => Option::<Box<dyn Instruction>>::Some(Box::new(MultiplyRegister)),
            BANI => Option::<Box<dyn Instruction>>::Some(Box::new(BitwiseAndImmediate)),
            BANR => Option::<Box<dyn Instruction>>::Some(Box::new(BitwiseAndRegister)),
            BORI => Option::<Box<dyn Instruction>>::Some(Box::new(BitwiseOrImmediate)),
            BORR => Option::<Box<dyn Instruction>>::Some(Box::new(BitwiseOrRegister)),
            SETI => Option::<Box<dyn Instruction>>::Some(Box::new(SetImmediate)),
            SETR => Option::<Box<dyn Instruction>>::Some(Box::new(SetRegister)),
            GTIR => Option::<Box<dyn Instruction>>::Some(Box::new(GreaterImmediateRegister)),
            GTRI => Option::<Box<dyn Instruction>>::Some(Box::new(GreaterRegisterImmediate)),
            GTRR => Option::<Box<dyn Instruction>>::Some(Box::new(GreaterRegisterRegister)),
            EQIR => Option::<Box<dyn Instruction>>::Some(Box::new(EqualImmediateRegister)),
            EQRI => Option::<Box<dyn Instruction>>::Some(Box::new(EqualRegisterImmediate)),
            EQRR => Option::<Box<dyn Instruction>>::Some(Box::new(EqualRegisterRegister)),
            _ => None,
        }?;
        Some(
            InstructionDescription {
                instruction,
                arguments
            }
        )
    }

    fn execute(&self, computer: &Computer) -> Vec<i64> {
        self.instruction.get_registers_values(
            &computer.registers,
            &self.arguments
        )
    }
}

struct Computer {
    instruction_pointer: usize,
    registers: Vec<i64>,
    instruction_register: usize,
    program: Vec<InstructionDescription>
}

impl Computer {
    fn from_strings(strings: &Vec<String>) -> Option<Computer> {
        let instruction_register = get_instruction_register(&strings[0])?;
        let program: Vec<_> = strings.iter().skip(1)
            .filter_map(|string| InstructionDescription::from_string(string))
            .collect();
        if program.len() == strings.len() - 1 {
            Some(
                Computer {
                    instruction_pointer: 0,
                    registers: vec![0; 6],
                    instruction_register,
                    program
                }
            )
        } else {
            None
        }
    }

    fn step(&mut self) {
        self.registers[self.instruction_register] = self.instruction_pointer as i64;
        let current_instruction = &self.program[self.instruction_pointer];
        let updated_registers = current_instruction.execute(self);
        self.registers = updated_registers;
        self.instruction_pointer = self.registers[self.instruction_register] as usize;
        self.instruction_pointer += 1;
    }

    fn is_halted(&self) -> bool {
        self.instruction_pointer >= self.program.len()
    }

    fn execute_until_halt(&mut self) {
        while !self.is_halted() {
            self.step();
        }
    }

    fn get_register_value(&self, index: usize) -> i64 {
        self.registers[index]
    }
}

fn get_instruction_register(string: &str) -> Option<usize> {
    lazy_static!{
        static ref INSTR_REG_REGEX: Regex = Regex::new(r"#ip (\d)").unwrap();
    }
    let captures = INSTR_REG_REGEX.captures(string)?;
    captures[1].parse::<usize>().ok()
}

pub fn solve_part_one() {
    let strings = read_lines("day_nineteen.txt");
    let mut computer = Computer::from_strings(&strings).expect("Error reading program for computer.");
    computer.execute_until_halt();
    let answer = computer.get_register_value(0);
    println!("{}", answer);
}

pub fn solve_part_two() {
    let strings = read_lines("day_nineteen.txt");
    let mut computer = Computer::from_strings(&strings).expect("Error reading program for computer.");
    computer.execute_until_halt();
    computer.instruction_pointer = 0;
    computer.registers = vec![0; 6];
    computer.registers[0] = 1;
    computer.execute_until_halt();
    let answer = computer.get_register_value(0);
    println!("{}", answer);
}