fn main() {
    println!("Hello, world!");

    another_function();
    parameter_function(5);
    labeled_measurement(500, 'c');

    let x = five();
    println!("{}", x);
    println!("5+1 is {}", plus_one(five()));
}

fn another_function() {
    println!("Another function.");
}

fn parameter_function(x: i32) {
    println!("The value of x is {}", x);
}

fn labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

/* 
 * Long form
 * comment
 * over mutliple lines
 */
fn plus_one(x: i32) -> i32 {
    x + 1 // by not having a semi-colon this is na expression 
          // not a statement.
}
