fn main() {
    let mut x = 5;   //add mut to get rid of error
    println!("The value of x is {x}");
    // immutable variables by default
    // throws an error 'cannot assign twice to immutable variable'
    x =6; 
    println!("The value of x is {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 60 * 3;
    // type must listed for const variables
    // const variables are always immutable

    /////// SHADOWING ////////
    /// 
    
    let y = 5;

    // the previous value is added to 1-- shawdowing
    let y = y + 1;

    {
        // the previous value is multiplied by two
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    // with shadowing you can change variable types on the flt
    
    let spaces = "   ";
    let spaces = spaces.len();

    println!("Spaces: {spaces}")
    // // contrast this with:
    // let mut spaces = "   ";
    // spaces = spaces.len()  /// type error
}
