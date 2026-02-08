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
    print!("Assigning value of one variable to another variable");
    let v = vec![1, 2, 3]; // vector v owns the object in heap

    //only a single variable owns the heap memory at any given time
    let v2 = v; // here two variables owns heap value,
    //two pointers to the same content is not allowed in rust
    //Rust is very smart in terms of memory access ,so it detects a race condition
    //as two variables point to same heap
    // println!("{:?}", v); // erro aqui
}

fn display1(v: Vec<i32>) {
    println!("inside display {:?}", v);
}
fn exemplo2() {
    println!("≡ 2 -----------------------------");
    println!("Passing value to a function");
    let v = vec![1, 2, 3]; // vector v owns the object in heap
    let v2 = v; // moves ownership to v2
    display1(v2); // v2 is moved to display and v2 is invalidated
    // println!("In main {:?}", v2); //v2 is No longer usable here
}
fn display2(v: Vec<i32>) -> Vec<i32> {
    // returning same vector
    println!("inside display {:?}", v);
    v
}
fn exemplo3() {
    println!("≡ 3 -----------------------------");
    println!("Returning value from a function");
    let v = vec![1, 2, 3]; // vector v owns the object in heap
    let v2 = v; // moves ownership to v2
    let v2_return = display2(v2);
    println!("In main {:?}", v2_return);
}
fn exemplo4() {
    println!("≡ 4 -----------------------------");
    println!("Ownership and Primitive Types");
    let u1 = 10;
    let u2 = u1; // u1 value copied(not moved) to u2

    println!("u1 = {}", u1);
    println!("u2 = {}", u2);

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
