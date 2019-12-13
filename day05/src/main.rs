use std::fs::File;
use std::io;
use std::io::Read;
use std::str::FromStr;
use core::fmt;

struct Instruction {
    opcode: i32,
    operation: fn(x: i32, y: i32) -> i32,
    //operation to be executed, 1 for add, 2 for mult
    fist_operand_pos: usize,
    //position index of first operand
    second_operand_pos: usize,
    //position index of second operand
    result_pos: usize, //position index of result
    //number of positions this instruction ocuppies
    size: i32,
}
impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Instruction {{ opcode: {}, param1: {}, param2: {} , resultpos: {}, size:{} }}", self.opcode, self.fist_operand_pos, self.second_operand_pos, self.result_pos, self.size)
    }
}
fn main() -> io::Result<()> {
    let mut file = File::open("src/day05.txt")?;
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;
    let mut program_vec: Vec<i32> = file_content.split(",").map(|entry| i32::from_str(entry).unwrap_or(-9999)).collect();


    //part1
    //insert input
    run_program(program_vec.as_mut());
    println!("part1 done");
    println!("value at pos 0 is {}", program_vec[0]);
    //end part 1


    Ok(())
}


fn run_program(program: &mut Vec<i32>) -> &Vec<i32> {
    let program_size = program.len();
    let mut last_op_code: Instruction;

    let mut i = 0;
    program.insert(1,1);
    let mut current_instruction = read_instruction(&program, i);
    program.remove(1);
    while  current_instruction.opcode != 99 {
        println!("reading instruction at pos {}",i);
        current_instruction = read_instruction(program, i);
        if current_instruction.operation == error_operation || current_instruction.operation == halt_operation {
            println!("exiting, read instruction {:?}",current_instruction);
            break;
        }
        println!("executing {:?}",current_instruction);
        execute_instruction(&current_instruction, program);
        i = i+current_instruction.size as usize ;
    }

    return program;//return end state
}

fn read_instruction(program: &Vec<i32>, pos: usize) -> Instruction {

    let opcode_num = program[pos]%100;

    //parse opcode and determine operation
    let mut opcode_operation =
        match opcode_num {
            1 => add_operation,
            2 => multiply_operation,
            3 => copy_operation,
            99 => halt_operation,
            _ => error_operation,
        };
    //how many positions should be read
    let mut num_pos = match opcode_num {
        1 => 4,
        2 => 4,
        3 => 2,//1 opcode,0 param,1 output
        4 => 3,//1 opcode,1 param,1 output
        99 => 0,
        _ => 0,
    };

    //check if opcode can be fully read
    if (program.len() <= pos + num_pos) {
        //println!("not enough program to read opcode");
        if (program[pos] != 99) {
            opcode_operation = error_operation
        }
        return Instruction { opcode: opcode_num, operation: opcode_operation, fist_operand_pos: 0, second_operand_pos: 0, result_pos: 0 , size: num_pos as i32 };
    } else {
        // println!("constructing opcode with params - operation:{} first_operand:{} second_operand:{} position to be overridden:{}",program[pos],program[program[pos+1]as usize],program[program[pos+2]as usize],program[pos+3]as usize);
        return match opcode_num {
            2 => Instruction { opcode: opcode_num, operation: opcode_operation, fist_operand_pos: 1, second_operand_pos: 0 as usize, result_pos: program[pos + 1] as usize , size: num_pos as i32 },
            3 => Instruction { opcode: opcode_num, operation: opcode_operation, fist_operand_pos: program[pos + 1] as usize, second_operand_pos: 0 as usize, result_pos: program[pos + 2] as usize , size: num_pos as i32 },
            4 => Instruction { opcode: opcode_num, operation: opcode_operation, fist_operand_pos: program[pos + 1] as usize, second_operand_pos: program[pos + 2] as usize, result_pos: program[pos + 3] as usize , size: num_pos as i32 },
            _ => Instruction { opcode: opcode_num, operation: opcode_operation, fist_operand_pos: program[pos + 1] as usize, second_operand_pos: program[pos + 2] as usize, result_pos: program[pos + 3] as usize , size: num_pos as i32 },
        };
    }
}

fn execute_instruction(instruction: &Instruction, program: &mut Vec<i32>) {
    let operation = instruction.operation;
    if (operation == halt_operation || operation == error_operation) {
        return;
    }
    let mut first_param=program[instruction.fist_operand_pos as usize];
    let mut second_param=program[instruction.second_operand_pos as usize];
    //todo: parse opcode for mode detection
    let mut param_modes = vec!();
    for digit in instruction.opcode.to_string().chars(){
        param_modes.push(digit.to_digit(10));
    }

    println!("instruction opcode: {:?}",instruction.opcode);
    println!("param_modes: {:?}",param_modes);
    let pos_first_op_mode:i32 = param_modes.len() as i32 - 2;
    let pos_second_op_mode:i32 = param_modes.len() as i32 - 3;

//    println!("operand 1 mode: {}",param_modes[pos_first_op_mode].unwrap());
//    println!("operand 2 mode: {}",param_modes[pos_second_op_mode].unwrap());
    if(pos_first_op_mode >= 0 && param_modes[pos_first_op_mode as usize].unwrap()==1){first_param= instruction.fist_operand_pos as i32;}
    if(pos_second_op_mode >= 0 && param_modes[pos_second_op_mode as usize].unwrap()==1){second_param= instruction.second_operand_pos as i32;}


    program[instruction.result_pos as usize] = operation(first_param,second_param);
    println!(" to position {}",instruction.result_pos);
    println!("finished executing\n");
}


fn add_operation(x: i32, y: i32) -> i32 {
    print!("x + y = {}",x+y);
    return x + y;
}

fn multiply_operation(x: i32, y: i32) -> i32 {
    print!("x * y = {}",x*y);
    return x * y;
}

fn error_operation(x: i32, y: i32) -> i32 {
    return -1;
}

fn halt_operation(x: i32, y: i32) -> i32 {
    println!("halt");
    return 99;
}

fn copy_operation(x: i32, y: i32) -> i32 {
    print!("copy x ({}) ",x);
    return x;
}


#[cfg(test)]
mod tests {
    use crate::run_program;

    #[test]
    fn test_run_program() {
        let mut program: Vec<i32> = vec![1, 0, 0, 0, 99];
        let mut end_state: Vec<i32> = vec![2, 0, 0, 0, 99];
        assert_eq!(run_program(program.as_mut()), &end_state);
    }
}
