fn another_function(x: i32, y: &str) -> &str {
    println!("Another function: {x}");
    y
}

fn main() {
    let y = {
        let x = 3;
        x+1
    };

    println!("Hello, world!: {y}");

    another_function(5, "hey");
}
