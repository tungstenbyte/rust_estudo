fn main() {
    exemplo1();
    exemplo2();
    exemplo3();
    exemplo4();
    exemplo5();
    exemplo6();
    exemplo7();
    exemplo8();
    exemplo9();
    exemplo10();
}

fn exemplo1() {
    println!("≡ 1 -----------------------------");
    let minhatupla: (i32, f64, u8) = (-325, 4.9, 22);
    println!("{:?}", minhatupla);
    println!("Integer is: {:?}", minhatupla.0);
    println!("Floate 64 is: {:?}", minhatupla.1);
    println!("Unsigned 8 is: {:?}", minhatupla.2);
}
fn exemplo2() {
    println!("≡ 2 -----------------------------");
    let b: (i32, bool, f64) = (110, true, 10.9);
    print_tuple(b);
}
fn print_tuple(param_tuple: (i32, bool, f64)) {
    println!("Insider print method: {:?}", param_tuple);
}

fn exemplo3() {
    println!("≡ 3 -----------------------------");
    let b: (i32, bool, f64) = (30, true, 7.9);
    display(b);
}
fn display(x: (i32, bool, f64)) {
    println!("Inside print method");
    let (age, is_male, cgpa) = x; //assigns a tuple to distinct variables
    println!("Age is {} , isMale? {},cgpa is {}", age, is_male, cgpa);
}

fn exemplo4() {
    println!("≡ 4 -----------------------------");
}
fn exemplo5() {
    println!("≡ 5 -----------------------------");
}
fn exemplo6() {
    println!("≡ 6 -----------------------------");
}
fn exemplo7() {
    println!("≡ 7 -----------------------------");
}
fn exemplo8() {
    println!("≡ 8 -----------------------------");
}
fn exemplo9() {
    println!("≡ 9 -----------------------------");
}
fn exemplo10() {
    println!("≡ 10 -----------------------------");
}
