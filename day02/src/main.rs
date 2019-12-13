use std::fs::File;
use std::io;
use std::io::Read;
use std::str::FromStr;

struct Opcode {
    operation: fn(x: i32, y: i32) -> i32,
    //operation to be executed, 1 for add, 2 for mult
    fist_operand_pos: usize,
    //position index of first operand
    second_operand_pos: usize,
    //position index of second operand
    result_pos: usize, //position index of result
}

fn main() -> io::Result<()> {
    let mut file = File::open("src/day02.txt")?;
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;
    let mut program_vec: Vec<i32> = file_content.split(",").map(|entry| i32::from_str(entry).unwrap_or(-9999)).collect();


    //part2
   for i in 0..99{
       for j in 0..99{
          let mut program_vec_part2 = program_vec.clone();
           let noun = i;
           let verb = j;
           program_vec_part2[1] = i;
           program_vec_part2[2] = j;
           run_program(program_vec_part2.as_mut());
           if(program_vec_part2[0]==19690720){
               println!("part2 done");
               println!("noun: {} verb:{}",i,j);
               println!("100 * noun + verb = {}",100 * noun + verb);
               break;
           }
       }
   }
    //end part2

    //part1
    program_vec[1] = 12;
    program_vec[2] = 2;
    run_program(program_vec.as_mut());
    println!("part1 done");
    println!("value at pos 0 is {}",program_vec[0]);
    //end part 1




    Ok(())
}


fn run_program(program: &mut Vec<i32>) -> &Vec<i32> {
    let program_size = program.len();
    let mut last_op_code: Opcode;
    let opcode_size:i32 = 4;

    for i in (0..program_size).step_by(4) {
        let current_opcode = read_opcode(program, i );
        if current_opcode.operation == error_operation {
            break;
        }
        execute_opcode(current_opcode, program);
    }

    return program;//return end state
}

fn read_opcode(program: &Vec<i32>, pos: usize) -> Opcode {
    //determine operation
    let mut opcode_operation =
    match program[pos] {
        1  =>  add_operation,
        2  =>  multiply_operation,
        99 =>  halt_operation,
        _  =>  error_operation,
    };

    //check if opcode can be fully read
    if(program.len()<=pos+4){
        //println!("not enough program to read opcode");
        if(program[pos]!=99) {
            opcode_operation=error_operation
        }
        return Opcode{operation:opcode_operation,fist_operand_pos:0,second_operand_pos:0,result_pos:0};
    }
    else{
       // println!("constructing opcode with params - operation:{} first_operand:{} second_operand:{} position to be overridden:{}",program[pos],program[program[pos+1]as usize],program[program[pos+2]as usize],program[pos+3]as usize);
        return Opcode { operation: opcode_operation, fist_operand_pos: program[pos + 1] as usize, second_operand_pos: program[pos + 2] as usize, result_pos: program[pos + 3] as usize };
    }
}

fn execute_opcode(opcode: Opcode, program: &mut Vec<i32>) {
    let operation = opcode.operation;
    if(operation==halt_operation || operation==error_operation){
        return;
    }
    program[opcode.result_pos as usize] = operation(program[opcode.fist_operand_pos as usize],program[opcode.second_operand_pos as usize]);

}


fn add_operation(x: i32, y: i32) -> i32 {
    return x + y;
}

fn multiply_operation(x: i32, y: i32) -> i32 {
    return x * y;
}

fn error_operation(x: i32, y: i32) -> i32 {
    return -1;
}

fn halt_operation(x: i32, y: i32) -> i32 {
    println!("halt");
    return 99;
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