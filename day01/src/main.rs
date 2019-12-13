use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("src/day01.txt")?;
    let mut reader = BufReader::new(file);
    let mut iter = reader.by_ref().lines();


    //let sum:i32 = iter.by_ref().map(|x| x.unwrap()).map(|line|mass_for_module(&line)).sum();
    let sum2:i32 = iter.by_ref().map(|x| x.unwrap()).map(|line|mass_for_module_recursive(&line,0)).sum();


//    println!("part1: {}",sum);
    println!("part2: {}",sum2);
    Ok(())
}
fn mass_for_module(module_string: &str) -> i32{
    let module_int:i32 = module_string.parse().unwrap();
    return (math::round::floor(module_int as f64/ 3 as f64, 0) as i32) - 2;
}
fn mass_for_module_recursive(module_string: &str, mut total_fuel:i32) -> i32{
    let fuel_for_module = mass_for_module(module_string);
    println!("fuel for module {} is {}",module_string,fuel_for_module);
    if fuel_for_module>0 {
        total_fuel +=fuel_for_module;
      return  mass_for_module_recursive(&fuel_for_module.to_string(), total_fuel);

    }else{
        println!("fuel below 0,end of recursion");
        println!("total fuel {}\n",total_fuel);
        return total_fuel;
    }
}


#[cfg(test)]
mod tests{
    use crate::{mass_for_module, mass_for_module_recursive};

    #[test]
    fn test_mass_for_module(){
        assert_eq!(mass_for_module("12"),2);
        assert_eq!(mass_for_module("14"),2);
        assert_eq!(mass_for_module("1969"),654);
        assert_eq!(mass_for_module("100756"),33583);
    }
    #[test]
    fn test_mass_for_module_recursive(){
        assert_eq!(mass_for_module_recursive("14",0),2);
        assert_eq!(mass_for_module_recursive("1969",0),966);
        assert_eq!(mass_for_module_recursive("100756",0),50346);
    }
}
