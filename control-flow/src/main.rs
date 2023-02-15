fn main() {
    let test = 7;

    // / # This code won't run
    // / '''
    // / fn main() {
    // /     let number = 3
    // /     if number {
    // /         println!("number was three");
    // /     }
    // / }
    // / '''
    // / >must chang number != 0 for it to run

    if test<5 {
        println!("The condition was true");
    }  else {
        println!("The condition was false");
    }
    divisible(6);
}

fn divisible(number:i32) {

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    ternary();
    breaking();
    // loopLabels();
    count_down();
    for_loop();
    testing();
}

// if condition like a ternary 

fn ternary() {
    let condition = true;
    let number = if condition {5} else {6};
    println!("The value of number is: {number}");
}

// fn misMatchTernary() {
//     let condition = true;
//     // panics! because type mismatch with the return values of 5 and 'six'
//     // won't work because variables are of a single type and 'number'
//     // can only be one type
//     // values are expressions in Rust and can return for a let
//     let number = if condition {5} else {'six'};
//     println!("The value of number is: {number}");
// }

fn breaking () {
    let mut counter = 0;
    let result = loop {
        counter += 1; 
        if counter == 10 {
            break counter * 2
        }
    };

    println!("result:  {result}");
} 

fn loopLabels() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn count_down() {
    let mut number = 3;
    
    while number != 0 {
        println!("{number}");

        number -= 1;
    }
    println!("LIFTOFF!!!");
}


fn for_loop () {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1; 
    }

    // error prone- if index value or test condition are incorrect
    // because index will get to 5 but exits before the body of the
    // while loop runs
    // Also, if you change the a array then you must change the condition

    // for loop is more concise in this situation

    for element in a {
        println!("The value in the for loop is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn testing () {
    
    let mut x = 0;
    'a: loop {
        x += 1;
        println!("{x}");
        'b: loop {
            if x > 10 {
                continue 'a;
            } else {
                break 'b;
            }      
        }
        break;       
    }
}