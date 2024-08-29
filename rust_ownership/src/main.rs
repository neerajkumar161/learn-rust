/*
 * Stack and Heaps - https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html#the-stack-and-the-heap
 * Stack and Heaps are type of memory allocation in the memory storage.
 * - Data Stored in one after another
 * - Last in First out (LIFO)
 * - Faster than Heap
 * - Push (add values) and Pop (remove values)
 * - Ideal for fixed sizes where size in already known.
 * Heaps
 * - Less organised than Stack
 * - Slower than Stack
 * - Find space for value and store it and returns pointer for value
 * - Ideal for unknown size or where size can grow for values
 * - Eg. Group of people trying to sit in restaurant, where host finds the enough space for people and then allocate. Later when new late members comes in, host finds the table number where group is seated
 *
 * What OWNERSHIP Addresses in Rust?
 * - Keeping records what part of data stored in heap.
 * - Minimize the duplicating data.
 * - Clean up unused data, so heap don't run out of memory
 * Overall main purpose of OWNERSHIP is to manage heap data.
 *
 * ONWNERSHIP Rules
 * - Each value in Rust has an owner.
 * - There can only be one owner at a time.
 * - When the owner goes out of scope, the value will be dropped.
 *   */

fn main() {
    /* String are example for unknown size */
    // https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html#memory-and-allocation
    let mut str = String::from("Hello");
    str.push(',');
    str.push_str(" World!");

    println!("{str}");

    /* https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move
     * Variables and Data Interaction with Move
     *  */

    let x = 5;
    let y = x;
    println!("x and y: {x} {y}");
    /* y will copy value from x, and both stores in Stack. Since integers are simple values which means their size is already defined, for example in this case x and y having size of 32bit integer.
     * Other Data types like: Boolean, Floats, Chars,Tuples eg (i32, i32) but not (i32, String)
     * Now let's take a look at the String version
     */

    let s1 = String::from("Hello There!");
    let s2 = s1; // move in Rust, Shallow Copy in JS, but s1 is removed here.
    println!("s2: {s2}");
    /* Let's understand what is different here.
     *
     * s1, what is stored in Stack
     * - pointer (to content of s1)
     * - len (12, how much memory in bytes)
     * - Capacity (12, total amount of memory in bytes)
     *
     * s1, what is stored in Heap
     * index, value
     * 0    , H
     * 1    , e
     * 2    , l
     * 3    , H
     * 4    , o
     * ... so on
     *
     * When we assign s1 to s2, the String data is copied, meaning we copy the pointer, the length, and the capacity that are on the stack. We do not copy the data on the heap that the pointer refers to.
     *
     * double free error, toi illustrate we try to access s1, it will give us compile error
     *
     * println!("s1: {s1}");  // because Rust remove the s1 from memory, only s2 is available
     *
     * In JS this is shallow copy, in Rust it called move.
     *
     * Deep Copy the value using clone();
     * https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-clone
     *
     *  */

    let s3 = String::from("Hello");
    let s4 = s3.clone();

    println!("s3: {s3} and s4: {s4}"); // No compiler error

    /* Ownership and Functions - https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html#ownership-and-functions
     *
     * The mechanics of passing a value to a function are similar to those when assigning a value to a variable. Passing a variable to a function will move or copy, just as assignment does.
     *
     * */

    {
        let s5 = String::from("Neeraj Kumar"); // s5 comes into scope

        takes_ownership(s5); // s5 moves to function
                             // s5 no longer values, Rust removed it from memory
                             // println!("After takes_ownership s5: {s5}");  // Compile error
        fn takes_ownership(str: String) {
            // str comes to scope
            println!("{str}"); //
        } // Here, str goes out of scope and 'drop' is called. The backing memory is freed.

        let num1 = 5; // num1 comes into scope

        makes_copy(num1); // num1 would move into function, but i32 is copied, so it's okay to still
                          // use num1 afterward

        println!("After makes_copy num1: {num1}");
        fn makes_copy(num: i32) {
            // num comes into scope
            println!("{num}");
        } // Here, num goes out of scope. Nothing special happens

        /* Returning values can also tranfer ownership, Let's take a example */
        let s6 = give_ownership();
        let s7 = String::from("Neeraj");
        let s8 = takes_and_gives_back(s7);

        println!("s6: {s6}"); // Works
                              // println!("{s7}");  // Compile error, because ownershipt is transferred to another var
        println!("s8: {s8}"); // s7 tranferred to s8}

        fn give_ownership() -> String {
            let some_string = String::from("Neeraj");
            some_string
        }

        fn takes_and_gives_back(a_string: String) -> String {
            a_string
        }
    }

    /* We can return multiple values from a function using tuples, but this is too much work for a concept that should be common. So Rust provide us a feature that using a value without transferring the ownership, called REFERENCES. */
    {
        let s9 = String::from("Hello there!");
        let (s10, s10_len) = calculate_len(s9);

        println!("s10 and length: {s10} {s10_len}");

        fn calculate_len(s: String) -> (String, usize) {
            let len = s.len();

            (s, len)
        }
    }
    /* References and Borrowing
     * We can pass the value as a reference so we don't need to return the value from a function
     * A reference like a pointer in that it's an address we can follow to access the data stored at that address; that data is owned by some other variable.
     * Unlike a pointer, a reference is a guaranteed to point to a valid value of a particular type for the life of a reference. */
    {
        let s11 = String::from("Neeraj Kumar!");
        let s11_str_len = calculate_len_ref(&s11);
        println!("The length of s11: {s11_str_len}");

        fn calculate_len_ref(s: &String) -> usize {
            // Here s > &String, is reference which does not ownereship the variables

            // s.push_str("I am adding here!");  // NonMutable reference
            s.len()
        } // Since no ownership, s will not be dropped
    }
    /* We call action of creating reference BORROWING. As in real life, if a person owns something, you can borrow it from them. When you're done, you have to give it back. You don't own it
    *
    * Mutable References, we can modify the reference using &mut keyword example
    * we can create only one mutable reference at a time
    *
    *  This will gives us error.
    *  let mut s = String::from("hello");
       let r1 = &mut s;
       let r2 = &mut s;
       println!("{}, {}", r1, r2);
    * */

    {
        let s12 = String::from("Hey");
        change_str(&mut str, ", Neeraj Kumar!");
        println!("Updated String s12: {s12}");

        fn change_str(str: &mut String, add_data: &str) {
            str.push_str(add_data);
            // str.to_string()
        }
    }

    // Using curly braces, we can create multiple mutable references, due to their scope
    {
        let s13 = String::from("Hey!!");
        {
            let _r1 = &s13;
        }
        let _r2 = &13;
        // Rust enforces a similar rule for combining mutable and immutable references. This code results in an error
        let s14 = String::from("Hello");

        let r3 = &s14; // No problem
        let r4 = &s14; // No problem
                       // let r5 = &mut s14; // BIG PROBLEM
        println!("{}, {}", r3, r4);
    }

    // But this will work, because r1 and r2 is used in println, so they will removed from memory
    {
        let mut s = String::from("hello");

        let r1 = &s; // no problem
        let r2 = &s; // no problem
        println!("{r1} and {r2}");
        // variables r1 and r2 will not be used after this point

        let r3 = &mut s; // no problem
        println!("This will work: {r3}");
    }

    /*
     * Dangling References - A pointer that references a location in memory that may have been given to someone else
     *  */
    {
        let _ref_to_nothing = dangle();

        fn dangle() {
            let _s = String::from("Dangle Fn");
            // &s  // Cannot return a ref to local variable, because s ends by the eofunction
        }
    }

    /* Slice Type
     * https://doc.rust-lang.org/stable/book/ch04-03-slices.html#the-slice-type
     * Slices lets you reference the contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not a owenership.
     */
    {
        fn _find_first_word(s: &String) -> usize {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    // b' ' <- Bytes representation of the space character
                    return i;
                }
            }

            s.len()
        }

        /* String Slices - A string slice is a reference to part of a string */
        let s = String::from("Hello World");

        let hello = &s[0..5];
        let world = &s[6..11];
        println!("String Slices: {}, second{}", hello, world);

        // THese two are equal, same for end_index
        let _slice = &s[0..2];
        let _slice = &s[..2];
        // Take the entire string, both are same
        let _entire = &s[0..s.len()];
        let _entire = &s[..];

        // Let's rewrite the first_word_fn
        fn first_word(s: &str) -> &str {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i];
                }
            }

            &s[..]
        }
        // let sentence = String::from("Neeraj Kumar");  // Both will work, because first_word accepts &str
        // Feature calls deref coercians
        let sentence = "Neeraj Kumar";
        let word = first_word(&sentence);

        println!("first_world word:{word}");
        // sentence.clear();
        // println!("first_world word:{word}"); // Will give error in sentence.clear();

        // We can create other type of slices too
        let arr = [1, 2, 3, 4, 5, 6, 7];
        let arr_slice = &arr[..4];
        assert_eq!(arr_slice, &[1, 2, 3, 4])
    }
}
