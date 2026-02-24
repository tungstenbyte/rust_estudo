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
    println!("Characters from the string");
    let n1 = "Tutorials".to_string();

    println!("length of string is {}", n1.len());
    let c1 = &n1[4..9]; // fetches characters at 4,5,6,7, and 8 indexes
    println!("{}", c1);
}
fn exemplo2() {
    println!("≡ 2 -----------------------------");
    println!("Slicing an integer array");
    let data = [10, 20, 30, 40, 50];
    use_slice(&data[1..4]); //this is effectively borrowing elements for a while
}

fn use_slice(slice: &[i32]) {
    // is taking a slice or borrowing a part of an array of i32s
    println!("length of slice is {:?}", slice.len());
    println!("{:?}", slice);
}

fn exemplo3() {
    println!("≡ 3 -----------------------------");
    println!("Mutable Slices");
    let mut data = [10, 20, 30, 40, 50];
    use_slice_mutable(&mut data[1..4]); // passes references of 20, 30 and 40
    println!("{:?}", data);
}

fn use_slice_mutable(slice: &mut [i32]) {
    println!("length of slice is {:?}", slice.len());
    println!("{:?}", slice);
    slice[0] = 1010; // replaces 20 with 1010
}

fn exemplo4() {
    println!("≡ 4 -----------------------------");
    println!("");
}
fn exemplo5() {
    println!("≡ 5 -----------------------------");
    println!("");
}
fn exemplo6() {
    println!("≡ 6 -----------------------------");
    println!("");
}
fn exemplo7() {
    println!("≡ 7 -----------------------------");
    println!("");
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
