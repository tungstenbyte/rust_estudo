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
    println!("Illustration: Creating a Vector - new()");

    let mut v = Vec::new();

    v.push(20);
    v.push(30);
    v.push(40);
    println!("size of vector is :{}", v.len());
    println!("{:?}", v);
}
fn exemplo2() {
    println!("≡ 2 -----------------------------");
    println!("Illustration: Creating a Vector - vec! Macro");
    let v = vec![1, 2, 3];
    println!("{:?}", v);
}
fn exemplo3() {
    println!("≡ 3 -----------------------------");
    println!("Illustration: push()");

    let mut v = Vec::new();
    v.push(20);
    v.push(30);
    v.push(40);

    println!("{:?}", v);
}
fn exemplo4() {
    println!("≡ 4 -----------------------------");
    println!("Illustration: remove()");
    let mut v = vec![10, 20, 30];
    v.remove(1);
    println!("{:?}", v);
}
fn exemplo5() {
    println!("≡ 5 -----------------------------");
    println!("Illustration: contains()");
    let v = vec![10, 20, 30];
    if v.contains(&10) {
        println!("found 10");
    }
    println!("{:?}", v);
}
fn exemplo6() {
    println!("≡ 6 -----------------------------");
    println!("Illustration: len()");

    let v = vec![1, 2, 3];
    println!("size of vector is :{}", v.len());
}
fn exemplo7() {
    println!("≡ 7 -----------------------------");
    println!("Accessing values from a Vector");

    let mut v = Vec::new();
    v.push(20);
    v.push(30);

    println!("{:?}", v[0]);

    v.push(20);
    v.push(30);
    v.push(40);
    v.push(500);

    for i in &v {
        println!("{}", i);
    }

    println!("{:?}", v);
}
fn exemplo8() {
    println!("≡ 8 -----------------------------");
    println!("");
}
fn exemplo9() {
    println!("≡ 9 -----------------------------");
    println!("");
}
fn exemplo10() {
    println!("≡ 10 -----------------------------");
    println!("");
}

fn exemplo11() {
    println!("≡ 8 -----------------------------");
    println!("");
}
fn exemplo12() {
    println!("≡ 9 -----------------------------");
    println!("");
}
fn exemplo13() {
    println!("≡ 10 -----------------------------");
    println!("");
}

