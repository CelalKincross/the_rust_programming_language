fn main() {
    another_function(5);
    print_labeled_measurement(5, 'h'); //char literal is in single quotes

    //Statements and expressions
    // let is a statement-- it does not return the value of the 
    // assignment which is unline other languages like C and Ruby
    // the right side of the statment let x = must be a value

    let y = {
        let x = 3;
        x+1     
        // expressions don't require semicolons. If you put a semicolon
        // it will not return a value
    };
    // let a = (let b = 6); doesn't work because let b is not an 
    // expression and doesn't return a value. 
    println!("The value of y is {y}");
    
    let z = five();
    println!("The value of z is {z}");

    let v = plus_one(7);

    println!("The value of v is {v}");

}

fn another_function(x:i32) {
    println!("The value of x is {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is : {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x+1
}