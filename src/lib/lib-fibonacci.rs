
//use memoize::memoize;
// extern crate memoization;
use std::{collections::HashMap};

// use memoization::my_custom_memoization;
// use lazy_static::lazy_static;
use memoize::memoize;


// lazy_static! {
//     static ref CACHE : Mutex<HashMap<usize, usize>> = Mutex::new(HashMap::new());
// }


pub fn fib_naive (n: usize) -> usize{
    match n  {
        0 => 0,
        1 => 1,
        _ => fib_naive(n-1) + fib_naive(n-2)
    }   
}

//TODO Error handling
pub fn fibonacci(n: usize) -> usize{

    // let mut cache: HashMap<usize,usize> = HashMap::new();

    // let fib_cache_2 = | n: usize| {
    //     match cache.get(&n).map(|entry| entry.clone()){
    //         Some(value) => value,
    //         None => {
    //            let value =  match n {
    //                 0 => 0,
    //                 1 => 1,
    //                 _ => self::(n-1) + fib_cache_2(n-2)
    //             };
    //             cache.insert(n, value);
    //             return value;
    //         }
    //     }
    // };

    fn fib_cache (cache: &mut HashMap<usize,usize>, n: usize) -> usize {
        match cache.get(&n).map(|entry| entry.clone()){
            Some(value) => value,
            None => {
               let value =  match n {
                    0 => 0,
                    1 => 1,
                    _ => fib_cache(cache, n-1) + fib_cache(cache, n-2)
                };
                cache.insert(n, value);
                return value;
            }
        }
    }

    let mut cache: HashMap<usize,usize> = HashMap::new();
    fib_cache(&mut cache, n)
}

#[memoize] 
pub fn fibonacci2(n: usize) -> usize {
    match n  {
        0 => 0,
        1 => 1,
        _ => fibonacci2(n-1) + fibonacci2(n-2)
    }     
}

#[cfg(test)]
mod test_fib{
    use crate::fib_naive;

    #[test]
    fn it_works(){
       assert_eq!(fib_naive(2), 1);
       assert_eq!(fib_naive(6), 8);
       assert_eq!(fib_naive(15), 610);
       assert_eq!(fib_naive(30), 832040);
    }

}