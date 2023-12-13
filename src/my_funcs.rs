/// Function: add_five
///
/// # Arguments (nym: u32)
///
/// # Returns u32
///
/// # Example
/// ```
/// let x = 5;
/// let y = add_five(x);
/// ```
///
/**
 * This is a multiline block
 * This is for the function add_five
 */
pub fn add_five(num: u32) -> u32 {
    /*
       First Item
       Returns result
    */
    num + 5 // adds 5
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn adds_five_test() {
        let x: u32 = 100;
        let y: u32 = add_five(x);

        // prints x and y
        println!("x and y are from test: {} {}", x, y);
        assert_eq!(y, 105);
    }
}
