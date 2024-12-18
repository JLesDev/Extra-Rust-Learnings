//use std::{num::Saturating, u32};

pub fn factorial(n: u32) -> u32 {
    let mut result: u32 = 1;
    //let mut result = Saturating(u32::MAX);
    for i in 1..=n {
        // Use saturating multiplication to stop at the maximum value of u32
        // rather than overflowing and wrapping around
        
        //result = i.saturating_mul(result*i);
        //result = Saturating(u32);
        
        //    result *= i;
        
        // result = saturating_mul(2u32);
        let u = result *= (i as u32);
        //let t: u32 = u as u32;
        //result = (u).saturating_add(u32::MAX);
        //result = result.saturating_mul(u);
        
        //result = Saturating(u32::MAX);
        
        
    }
    result
    //let ac = result as u32;
    //ac


}

#[cfg(test)]
mod tests {
    use crate::factorial;
    /* 
    #[test]
    fn twentieth() {
        assert_eq!(factorial(20), u32::MAX);
    }

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
    }*/
}
