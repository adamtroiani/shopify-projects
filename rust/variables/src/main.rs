fn main() {
    const HI: &str = "HI";
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    println!("The value of HI is: {HI}");

    // Function with implicit return
    fn implicit_return(a: i32, b: i32) -> i32 {
        a + b  // No semicolon = return value
    }

    // Function with explicit return
    fn explicit_return(a: i32, b: i32) -> i32 {
        return a + b;  // With return keyword
    }

    // Function with early return
    fn early_return(a: i32) -> i32 {
        if a < 0 {
            return 0;  // Early return
        }
        a  // Implicit return
    }

    println!("Implicit return: {}", implicit_return(5, 3));
    println!("Explicit return: {}", explicit_return(5, 3));
    println!("Early return (positive): {}", early_return(5));
    println!("Early return (negative): {}", early_return(-5));

    // Tuple examples
    let tuple: (i32, f64, &str) = (1, 2.0, "hello");
    println!("First element: {}", tuple.0);
    println!("Second element: {}", tuple.1);
    println!("Third element: {}", tuple.2);

    // Destructuring
    let (a, b, c) = tuple;
    println!("Destructured: a={}, b={}, c={}", a, b, c);

    // Mutable tuple
    let mut mut_tuple = (1, 2);
    mut_tuple.0 = 3;
    println!("Mutable tuple: {:?}", mut_tuple);
}
