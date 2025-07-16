fn main() {
    let mut num1;
    let mut num2;
    let temp;

    num1 = 10;
    num2 = 20;

    println!("Numbers before swapping");
    println!("num1: {num1}, num2: {num2}");

    temp = num1;
    num1 = num2;
    num2 = temp;

    println!("Numbers after swapping");
    println!("num1: {num1}, num2: {num2}");
}