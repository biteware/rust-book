fn main() {
    // Mutability (immutable by default).
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Constant declaration format.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("There are {} seconds in three hours", THREE_HOURS_IN_SECONDS);

    // Variable shadowing
    // -> By using let we can keep immutability after a few small change operations.
    let y = 2;
    let y = y + 5; // Overwrites the original y, but y is still set to 2 initially.
    {
        let y = y * 2; // shadows y so evaluates to 7 * 2
        println!("The shadowed variable y has a value of {y}");
    }
    // Back to 7 as outside the above scope.                            
    println!("The non-shadowed variable y has a value of {y}"); 

    // let can also be great for reusing variable names when converting types
    let spaces = "   ";
    let spaces = spaces.len();
    println!("There are {spaces} spaces");

    // scalar data types
    // There are 4 of these: integers, floating-point numbers, booleans and characters.
    let i: u32 = 5;
    let f: f64 = 2.0;
    let p: bool = true;
    let c: char = 'a';
    println!("int {} float {} bool {} char {}", i, f, p, c);

    // compound type - tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("there is a tuple type available {} {} {}", x, y, z);

    // compound type - array
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array access {}", a[0]);

    let a = [3; 10];
    println!("create array with 10 length, fill with value of 3 - {}", a[9]);
}
