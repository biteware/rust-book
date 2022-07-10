fn main() {
    let num = 5;

    if num < 10 {
        println!("the number is lower than 10");
    } else {
        println!("who really uses elses");
    }

    if num != 0 {
        println!("num does not equal 0");
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("the result is {result}");

    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    let a = [1, 2, 3, 4, 5];
    for element in a {
        println!("the value is {element}");
    }
}
