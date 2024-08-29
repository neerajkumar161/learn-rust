fn main() {

    /* Variablas & Mutability and Constants */
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of updated x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three Hours in Seconds: {THREE_HOURS_IN_SECONDS}");
    // THREE_HOURS_IN_SECONDS = 890; // cannot assign to constant

    /* Shadowing */
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y: {y}");

    let spaces = "     ";
    let spaces = spaces.len();

    println!("The spaces: {spaces}");


    // To remove warning, we added _ to mutable variable
    let mut _another_spaces = "   ";
    let _another_spaces = _another_spaces.len();
    println!("Another spaces: {_another_spaces}");

}
