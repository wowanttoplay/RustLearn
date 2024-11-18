fn main() {
    println!("Hello, world!");

    another_function();

    another_function2(5);

    print_labeled_measurement(1, 'b');

    let y = {
        let x = 1; // This is a statement, not an expression
        println!("The value of x is: {}", x);
        x + 1
    }; // The curly brackets is an expression, so x + 1 is not end with semicolon. let y = {} is a statement

    println!("y is {}", y);

    println!("five() = {}", five());
    println!("six() = {}", six());
}

fn another_function() {
    println!("Another function");
}

fn another_function2(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
    println!("The measurement is {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn six() -> i32 {
    return 6;
}
