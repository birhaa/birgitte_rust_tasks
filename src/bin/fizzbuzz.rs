fn fizz_buzz(number: usize) -> String {

    return match (number % 3 == 0, number % 5 == 0){
        (true, true) => String::from("FizzBuzz"),
        (true, false)  => String::from("Fizz"),
        (false, true) => String::from("Buzz"),
        _ => number.to_string()
    };
}

fn main(){
    let itr: usize = 1000;
    for i in 1..itr{
        let restult =fizz_buzz(i); 
        println!("{restult}");
    }
}

#[cfg(test)]
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