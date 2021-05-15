fn main() {

    // using  mut to alter the value of a var
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // using shadowing
    let y = 2;
    let y = y + 5;
    let y = y * 3;
    //let y += 2; //This won't work

    let spaces = "    ";
    let spaces = spaces.len();

    println!("{}",spaces);
}
