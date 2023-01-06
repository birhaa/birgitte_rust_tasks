mod rot13;
mod ts;

use std::usize;

fn add_two_numbers(n1: i32, n2: i32) -> i32 {
    return n1 + n2;
}

fn reverse(str: String) -> String {
    return str.chars().rev().collect()
}

fn two_sum(arr : &[i32])  -> String {
    let (answer, rest) = arr.split_at(1);
    let mut _result=Vec::new();
    for i in 0..rest.len() {
        let start : usize = i+1;
        for j in start..rest.len(){
            if rest[i] + rest[j] == answer[0] {
                _result.push(String::from("{rest[i]},{rest[j]}"));
                break;
              }
        }
        
    }
    _result.concat()
}

// //TODO Make better
// fn fizzbuzz(number: usize) -> String {
//     let check_number = |number: usize, divider: usize| number % divider == 0;

//     return  match number {
//         i if check_number(i, 3) && check_number(i, 5) => String::from("FizzBuzz"),
//         i if check_number(i, 5) => String::from("Buzz"),
//         i if check_number(i, 3) => String::from("Fizz"),
//         _ => number.to_string()
//     };
// }

fn fizz_buzz(number: usize) -> String {
    let check_number = |fizz: usize, buzz: usize| (number % fizz == 0, number % buzz == 0);

    return match check_number(3, 5) {
        (true, true) => String::from("FizzBuzz"),
        (true, false)  => String::from("Fizz"),
        (false, true) => String::from("Buzz"),
        _ => number.to_string()
    };
}

mod test{
    use crate::fizz_buzz;

    #[test]
    fn it_works(){
       assert_eq!(fizz_buzz(15), "FizzBuzz");
       assert_eq!(fizz_buzz(3), "Fizz");
       assert_eq!(fizz_buzz(5), "Buzz");
       assert_eq!(fizz_buzz(1), "1")
    }

}

fn fib(n: usize) -> usize{
    return  match n  {
        0 => 0,
        1 => 1,
        _ => fib(n-1) + fib(n-2)
    }
}

mod test_fib{
    use crate::fib;

    #[test]
    fn it_works(){
       assert_eq!(fib(2), 1);
       assert_eq!(fib(6), 8);
       assert_eq!(fib(15), 610);
       assert_eq!(fib(30), 832040);
    }

}




fn main() {
    let num = add_two_numbers(15, 10);
    let num = num + 1;

    {
        let num = num * 2;
        println!("The value of x in the inner scope is: {num}");
    }

    println!("The value of x is: {num}");

    let str = reverse(String::from("hello"));
    print!("{str}");
    let value = two_sum(&[7, 3, 5, 2, -4, 8, 11]);
    print!("{value}")
}


#[test]
fn should_fail(){
    unimplemented!("Fail")
}