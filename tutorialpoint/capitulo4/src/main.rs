fn main() {
    exemplo1();
    exemplo2();
    exemplo3();
    exemplo4();
    exemplo5();
    exemplo6();
    exemplo7();
}

fn exemplo1() {
    println!("Exemplo1 -------------");
    let company_string: &str = "TutorialsPoint"; // string type
    let rating_float: f64 = 4.5; // float type
    let is_growing_boolean: bool = true; // boolean type
    let icon_char: char = '♥'; //unicode character type

    println!("company name is:{}", company_string);
    println!("company rating on 5 is:{}", rating_float);
    println!("company is growing :{}", is_growing_boolean);
    println!("company icon is:{}", icon_char);
}

fn exemplo2() {
    println!("Exemplo2 -------------");
    let result = 10; // i32 by default
    let age: u32 = 20;
    let sum: i32 = 5 - 15;
    let mark: isize = 10;
    let count: usize = 30;
    println!("result value is {}", result);
    println!("sum is {} and age is {}", sum, age);
    println!("mark is {} and count is {}", mark, count);
}

fn exemplo3() {
    println!("Exemplo3 -------------");
    let age: u8 = 255;

    // 0 to 255 only allowed for u8
    let weight: u8 = 255; //overflow value is 256 is 0 
    let height: u8 = 255; //overflow value is 257 is 1 
    let score: u8 = 255; //overflow value is 258 is 2

    println!("age is {} ", age);
    println!("weight is {}", weight);
    println!("height is {}", height);
    println!("score is {}", score);
}

fn exemplo4() {
    println!("Exemplo4 -------------");
    let result = 10.00; //f64 by default
    let interest: f32 = 8.35;
    let cost: f64 = 15000.600; //double precision
    println!("result value is {}", result);
    println!("interest is {}", interest);
    println!("cost is {}", cost);
}

fn exemplo5() {
    println!("Exemplo5 -------------");
    let float_with_separator = 11_000.555_001;
    println!("float value {}", float_with_separator);
    let int_with_separator = 50_000;
    println!("int value {}", int_with_separator);
}

fn exemplo6() {
    let isfun: bool = true;
    println!("Is Rust Programming Fun ? {}", isfun);
}

fn exemplo7(){
    let special_character = '@'; //default
let alphabet:char = 'A';
let emoji:char = '😁';
println!("special character is {}",special_character);
println!("alphabet is {}",alphabet);
println!("emoji is {}",emoji);
}