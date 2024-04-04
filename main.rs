fn main() {
    // print text and a new line
    let x = 12;
    println!("hi hello ! {}", x);
    print!("hi hello !");

    printer2();
}

fn printer2() {
    let character: char = 'A';

    // convert char type to u8 integer type
    let integer = character as u8;

    println!("character = {}", character);
    println!("integer = {}", integer);

    //

    // only u8 integer data type can be converted into char
    let integer: u8 = 65;
    // convert integer to char using the as keyword
    let character = integer as char;

    println!("integer = {}", integer);
    println!("character = {}", character);
}

fn printer() {
    let age = 31;
    let name = "Jack";

    let ch = 'q';
    let sign = true;

    // print the variables using println!
    println!("\nName = {}, Age = {}", name, age);
    //  define serial
    println!("\nName = {1}, Age = {0}", name, age);

    let mut x = 1;
    println!("Value of x = {}", x);

    // change the value of variable x
    x = 2;
    println!("Updated value of x = {}", x);

    const PI: f32 = 3.14;
    println!("Initial Value of PI: {}", PI);
}
