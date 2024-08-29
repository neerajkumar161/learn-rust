/*
 * A struct, or structure, is a custom data type that lets you package together and name multiple related values that make up a meaningful group.
 *
 * It's like objects in JavaScript, which holds multiple values of different data types, so we can make complex or new data types
*/

fn main() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    /* Creating user using Structs */
    let mut user_one = User {
        active: true,
        username: String::from("user_one"),
        email: String::from("user_one@test.com"),
        sign_in_count: 10,
    };

    user_one.email = String::from("another_user_one@test.com");

    // println!("User One: {:?}", user_one);

    fn build_user(email: String, username: String) -> User {
        // We can use shorthand features like ES6 aka "field init shorthand syntax"
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }

    let user_two = build_user(
        String::from("neerajkumar@test.com"),
        String::from("neerajkumar123"),
    );

    // Creating struct instances using from another struct instance
    let user_three = User {
        username: user_two.username,
        email: user_two.email,
        active: false,
        sign_in_count: 1,
    };

    /* let user_four = User {
        active: user_two.active,
        // cannot user email and username becauase, this is already used in user_three
        email: user_two.email,
        username: user_two.username,
        sign_in_count: 2
    } */

    /*
    * Using Tuple Structs Without Named Fields to Create Different Types
    Rust also supports structs that look similar to tuples, called tuple structs. Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields; rather, they just have the types of the fields. Tuple structs are useful when you want to give the whole tuple a name and make the tuple a different type from other tuples, and when naming each field as in a regular struct would be verbose or redundant
    */

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let _black_color = Color(0, 0, 0);
    let _origin_point = Point(0, 0, 0);

    /*
     * Unit-Like Structs without any fields
     * You can also define structs without any fields, aka unit-like structs because they behave similarly to ()
     * Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself.
     */

    struct AlwaysEqual;

    let _subject = AlwaysEqual;

    /*
    * Ownership of Struct Data
    * https://doc.rust-lang.org/stable/book/ch05-01-defining-structs.html#ownership-of-struct-data
    *
    * Gives compile error, that it needs lifetime specifiers
       struct UserTwo {
           active: bool,
           username: &str,
           email: &str,
           sign_in_count: u64,
       }

    * In Chapter 10, we’ll discuss how to fix these errors so you can store references in structs, but for now, we’ll fix errors like these using owned types like String instead of references like &str.
    */

    /* An Example of using structs */

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    };

    let scale = 2;
    let rect1 = Rectangle {
        // Using debug in console, this will include the code line along with runtime value
        height: dbg!(30 * scale),
        width: 50,
    };

    println!("rect is {rect1:?}"); // fixed Debug issues in struct Rectangle
    println!("rect is for longer structs {rect1:#?}"); // fixed Debug issues in struct Rectangle
    println!(
        "The area of rectangle is {} square pixels using Structs",
        rect_area(&rect1)
    );

    fn rect_area(rectangle: &Rectangle) -> u32 {
        rectangle.height * rectangle.width
    }

    //  If don't pass &, we will taking the ownership, which lead to compiler error here
    // println!("{:?}", rect1.height)

    /* Our area function is now defined with one parameter, which we’ve named rectangle, whose type is an immutable borrow of a struct Rectangle instance. As mentioned in Chapter 4, we want to borrow the struct rather than take ownership of it. This way, main retains its ownership and can continue using rect1, which is the reason we use the & in the function signature and where we call the function.
    */

    /* Method Syntax, define methods for a struct,
     * it's like implementing Struct and add methods within
     * their first parameter is always self, which represents the instance of the struct the method is being called on.
     */

    struct Square {
        width: i32,
        height: i32,
    }

    impl Square {
        /* Same as bewlo fn area(self: &Self) , self is Rectangle,
        We chose &self here for the same reason we used &Rectangle in the function version: we don’t want to take ownership, and we just want to read the data in the struct, not write to it. If we wanted to change the instance that we’ve called the method on as part of what the method does, we’d use &mut self as the first parameter. 
        */
        fn area(&self) -> i32 {
            self.width * self.height
        }

        fn can_hold(&self, other_square: Square) -> bool {
            self.width > other_square.width && self.height > other_square.height
        }
    }

    let scale = 2;
    let square_1 = Square {
        width: 30 * scale,
        height: 60
    };

    let square_2 = Square {
        width: 70,
        height: 70
    };

    let square_3 = Square {
        width: 35,
        height: 35
    };


    println!("The area of a square is {}", square_1.area());
    println!("The square2 can hold square1 {}", square_1.can_hold(square_2));
    println!("The square3 can hold square1 {}", square_1.can_hold(square_3));

    // -> Operator,  https://doc.rust-lang.org/stable/book/ch05-03-method-syntax.html#wheres-the---operator


    /* Proper Square using Associated Functions that don't need self,
     * https://doc.rust-lang.org/stable/book/ch05-03-method-syntax.html#associated-functions
     */
    // Similar to String::from()
    // struct  PureSquare {
    //     width: u64
    //     height: u64
    // }
    impl Square {
        fn pure_square(size: i32) -> Self {
            Self {
                width: size,
                height: size
            }
        }
    }

    impl  Square {
        fn multi_impl_method(&self) {
            println!("This is multi_impl method");
        }
    }

    // Similar Like String::from , that create new instance, because, new keyword in not present in Rust
    let p_square = Square::pure_square(30);
    println!("Pure Square area {}", p_square.area());
    p_square.multi_impl_method();

    // Structs aren't the only way to create Custom Types, we have Rust's enum feature.
}
