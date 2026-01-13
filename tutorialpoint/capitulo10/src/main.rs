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
    println!("1 -----------------------------");
    for x in 1..11 {
        // 11 is not inclusive
        if x == 5 {
            continue;
        }
        println!(":: x is {}", x);
    }
}
fn exemplo2() {
    println!("2 -----------------------------");
    let mut x = 0;
    while x < 10 {
        x += 1;
        println!("inside loop x value is {}", x);
    }
    println!("outside loop x value is {}", x);
}
fn exemplo3() {
    println!("3 -----------------------------");
    let mut x = 0;
    loop {
        x += 1;
        println!("x={}", x);
        if x == 15 {
            break;
        }
    }
}
fn exemplo4() {
    println!("4 -----------------------------");
    let mut count = 0;
    for num in 0..21 {
        if num % 2 == 0 {
            continue;
        }
        count += 1;
    }
    println!(" The count of odd values between 0 and 20 is: {} ", count);
    //outputs 10
}
fn exemplo5() {
    println!("5 -----------------------------");
}
fn exemplo6() {
    println!("6 -----------------------------");
}
fn exemplo7() {
    println!("7 -----------------------------");
}
fn exemplo8() {
    println!("8 -----------------------------");
}
fn exemplo9() {
    println!("9 -----------------------------");
}
fn exemplo10() {
    println!("10 -----------------------------");
}
