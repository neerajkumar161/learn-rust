fn main() {
    /* If are sometimes called arms, just like the arms in match expressions */
    let number = 5;

    if number < 5 {
        println!("Condition was true!")
    } else {
        println!("Condition was false!");
    }
    /* Unlike JavaScript, Rust will not convert to boolean value, it should be bool explicitely */
    let is_true = true;
    if is_true {
        println!("Yes it is true!");
    }

    // Valid statement
    if number != 10 {
        println!("Number is not equal to 10");
    }

    /* Multiple if else conditions */
    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3 and 2");
    }

    /* Using if in let statement */
    let is_valid = if true { 5 } else { 6 };
    /* Error, because first condition expecting number, not string for else condition */
    // let is_valid = if true { "True" } else { "Six" };

    println!("Value is: {is_valid}");

    /* Repetition with Loops
     * Rust has three kinds of loops:
     * loop, while and for
     */

    let mut i = 1;
    loop {
        if i == 6 {
            break;
        }
        println!("Looping until 5: {i}");
        i += 1;
    }

    // Returning values from loops
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Returned value from the loop is: {result}");

    /* In rust we can also label the loops,
     * Loop Labels to Disabmiguate Between multiple loops */

    let mut count = 0;
    'counting_up: loop {
        println!("count is {count}");
        let mut remaining = 10;

        loop {
            println!("remaining: {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1
        }

        count += 1;
    }

    println!("End count: {count}");

    /* Conditional loops with while */
    let mut number = 3;

    while number != 0 {
        println!("While loop number: {number}");
        number -= 1;
    }

    /* For loop - Looping through a collection with while and for loop */
    let coll_a = [10, 20, 30, 40, 50];
    let mut coll_index = 0;

    while coll_index < 5 {
        println!("The value of coll_a: {}", { coll_a[coll_index] });

        coll_index += 1;
    }

    // More consice alternative using for loops
    for element in coll_a {
        println!("The element in collection: {element}");
    }

    /* Countdown in for loop with rev method */
    // ..= 4 in inclusive
    for number in (1..=4).rev() {
        println!("number is :{number}");
    }

    temp_conversion();
    // 0 1 1 2 3 5 8 13 21
    let nth_fib = nth_fibonacci_number(8);
    println!("nth fib number: {nth_fib}");

    twevle_days_of_christmas();
}

fn temp_conversion() {
    // (1°C × 9/5) + 32 = 33.8°F
    // (1°F − 32) × 5/9 = -17.22°C
    let temp_in_deg_celcius = 1;
    let temp_in_fahrenheit = 33;
    let deg_celcius_to_far = (temp_in_deg_celcius * 9 / 5) + 32;
    let deg_far_to_celcius = (temp_in_fahrenheit - 32) * 5 / 9;
    println!("The temp is F is: {deg_celcius_to_far}F");
    println!("The temp in C is: {deg_far_to_celcius}");
}

fn nth_fibonacci_number(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }

    return nth_fibonacci_number(n - 1) + nth_fibonacci_number(n - 2);
}

fn twevle_days_of_christmas() {
    let gifts = [
        "A partridge in a pear tree.",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for day in 0..12 {
        println!(
            "On the {} day of christmas, my true love gave to me",
            match day {
                0 => "first",
                1 => "second",
                2 => "third",
                3 => "fourth",
                4 => "fifth",
                5 => "sixth",
                6 => "seventh",
                7 => "eighth",
                8 => "ninth",
                9 => "tenth",
                10 => "eleventh",
                _ => "twelveth",
            }
        );

        for i in (0..=day).rev() {
            if day > 0 && i == 0 {
                println!("And")
            };
            println!("{}", gifts[i]);
        }

        println!();
    }
}