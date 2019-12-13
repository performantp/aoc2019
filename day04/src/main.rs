use std::io;

fn main() -> io::Result<()> {
    let mut input: Vec<i32> = (402328..864247).collect();
    let mut num_correct_pw = 0;
    for password in input {
        if (is_six_digits(password.to_string()) &&
            has_two_same_adjacent(password.to_string()) &&
            has_only_increasing_digits(password.to_string()) &&
            has_series_of_max_two(password.to_string())
        ) {
            num_correct_pw += 1;
        }
    }
    println!("{} of passwords matched the criteria", num_correct_pw);
    Ok(())
}

fn is_six_digits(password: String) -> bool {
    return password.len() == 6 && password.parse::<f64>().is_ok();
}

fn has_two_same_adjacent(password: String) -> bool {
    let mut last: char = 'n'; //no digit
    for digit in password.chars() {
        if digit == last { return true; }
        last = digit;
    }
    return false;
}

fn has_only_increasing_digits(password: String) -> bool {
    let mut last: char = 'n'; //no digit
    for digit in password.chars() {
        if digit.to_digit(10) < last.to_digit(10) { return false; }
        last = digit;
    }
    return true;
}

fn has_series_of_max_two(password: String) -> bool {
//    println!("checking max2 for {}",pawssword);
    let mut groups = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut last: char = 'n'; //no digit
    let mut seq_length = 1;
//println!("password.chars: {:?}", password.chars());
    for digit in password.chars() {
//        println!("digit:{} last:{}",digit,last);
        if digit == last {
            seq_length += 1;
            groups[(digit.to_digit(10)).unwrap() as usize] = seq_length;
        }else{
            seq_length=1;
        }
        last = digit;
    }
    println!("groups {:?}", groups);
    let mut group_of2: bool = false;
    for digit in groups.iter() {
        if *digit == 2 { group_of2 = true; }
    }
    return group_of2 ;
}

#[cfg(test)]
mod tests {
    use crate::has_series_of_max_two;

    #[test]
    fn test_subseq() {
        assert_eq!(has_series_of_max_two(String::from("112233")), true);
        assert_eq!(has_series_of_max_two(String::from("123444")), false);
        assert_eq!(has_series_of_max_two(String::from("111122")), true);
    }
}
