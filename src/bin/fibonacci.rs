// use fibonacci;
use birgitte_fibonacci::fib;
use birgitte_fibonacci::fib_memo;
use std::time::{Instant};


fn main(){
    let n = 90;

    let start = Instant::now();
    let fib =  fib(n);
    let duration = start.elapsed().as_millis();

    println!("Fibonacci of {n} is {fib} in {duration} ms");

    let start2 = Instant::now();
    let fib2 =  fib_memo(n);
    let duration2 = start2.elapsed().as_millis();

    println!("Fibonacci of {n} is {fib2} in {duration2} ms");
}