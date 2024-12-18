// Define a function named `factorial` that, given a non-negative integer `n`,
// returns `n!`, the factorial of `n`.
//
// The factorial of `n` is defined as the product of all positive integers up to `n`.
// For example, `5!` (read "five factorial") is `5 * 4 * 3 * 2 * 1`, which is `120`.
// `0!` is defined to be `1`.
//
// We expect `factorial(0)` to return `1`, `factorial(1)` to return `1`,
// `factorial(2)` to return `2`, and so on.
//
// Use only what you learned! No loops yet, so you'll have to use recursion!

pub fn factorial(n: u128) -> u128{
    let mut total: u128 = 1;
    let pre_total = total;
    let mut a = n;
    println!("a is: {}", a);
    total *= a;
    println!("The total of this factorial is: {}",total);
    
    if a > 1 {
        a -= 1;
        println!("This is the: {}, iteration",a);
        
        total *= factorial(a);
        
    }
    else{
        total = pre_total;
    }
    

    total
    
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
