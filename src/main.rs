fn main() {

    { 
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
        
    {
        // problem 2
        // ========================================
        // Each new term in the Fibonacci sequence is generated by adding the
        // previous two terms. By starting with 1 and 2, the first 10 terms will be:

        //                   1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...

        // By considering the terms in the Fibonacci sequence whose values do not
        // exceed four million, find the sum of the even-valued terms.
        let mut a = 1;
        let mut b = 2;
        let mut acc = 0;
        while b <= 4000000 {
            if b % 2 == 0 { acc = acc + b; }
            let oldb = b;
            b = a + b;
            a = oldb;
        }
        println!("{}", acc);
    }

    {
        // problem 3
        // ========================================
        // The prime factors of 13195 are 5, 7, 13 and 29.
        // What is the largest prime factor of the number 600851475143 ?
        
        fn prime_factors(n: u64) -> Vec<u64> {
            if n == 1 { return vec![]; } 

            match (
                std::ops::Range { start: 2, end: n-1 }.
                    filter(|x| n % x == 0).next()
            ) {
                Some(i) => {
                    let mut r = prime_factors(n / i);
                    r.push(i);
                    r
                },
                _ =>  vec![n]
            }
        }

        let q: u64 = 600851475143;
        let a = prime_factors(q);
        println!("The largest prime factor of {} is {}", q, &a[0]);        
    }
    
}
