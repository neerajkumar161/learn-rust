fn main() {
    println!("Hello, world!");
    another_function(56, 'h');
    statement_expressions();
    let returned_value = i_am_returning_a_value();
    println!("Returned value from a function: {returned_value}");
}

fn another_function(value: i32, unit_label: char) {
    println!("Hello from another_function: {value}{unit_label}");
}

fn statement_expressions() {
    // Will work with other langs, but not in Rust, y = 6 should return a value to be assigned in x
    // let x = (y = 6);
    let y = {
        let x = 3;
        /* Putting not semicolon means it is returning a value aka EXPRESSION */
        /* If we put semicolon then it will considered as a STATEMENT */
        x + 1
    };

    println!("The value of y is: {y}");
}

/* 
 * Functions with return values;
 * https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html#functions-with-return-values
 *
 */
fn i_am_returning_a_value() -> i32 { 
    19
}