// Rewrite the factorial function using a `for` lo
pub fn factorial(n: u32) -> u32 {
    /*let mut total: u32 = 1;
    //let pre_total = total;
    let mut a = n;
    println!("a is: {}", a);
    //total *= a;
    println!("The total of this factorial is: {}",total);
    //let static mut itar = a;

    let mut itar = a;
    for ita in a..1{
        total *= a;
        a -= 1;
        itar -= 1;
    }*/

    let mut total: u32 = 1;
    let pre_total = total;
    let mut a = n;
    println!("a is: {}", a);
    //total *= a;
    println!("The total of this factorial is: {}",total);
    
    /*while a >= 1 {
        total *= a;
        a -= 1;
    }*/
    let mut pre = a;
    for mut i in 1..a{
        total *= pre;
        pre -= 1;
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
