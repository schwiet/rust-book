fn main() {
    println!("Hello, world!");

    another_function(5);
    print_labeled_measurement(5, 'm');

    let y = {
        let x = 3;
        // Expressions do not include ending semicolons. If you add a semicolon
        // to the end of an expression, you turn it into a statement, and it
        // will then not return a value.
        x + 1
    };

    println!("The value of y is: {y}");

    let z = five();
    println!("The value of z is: {z}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}