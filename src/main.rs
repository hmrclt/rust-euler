fn main() {

   // If we list all the natural numbers below 10 that are multiples of 3 or 5,
   // we get 3, 5, 6 and 9. The sum of these multiples is 23.

   // Find the sum of all the multiples of 3 or 5 below 1000.    
    let range = std::ops::Range { start: 1, end: 1000 };
    let rangef = range.filter(|x| x % 3 == 0 || x % 5 == 0);
    let total = rangef.fold(0, |sum, x| sum + x);
    // rangef.for_each(move |x| 
    //     println!("{}", x)
    // );
    println!("{}", total);
}
