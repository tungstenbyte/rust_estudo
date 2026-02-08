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
    let arr: [i32; 4] = [10, 20, 30, 40];
    println!("array is {:?}", arr);
    println!("array size is :{}", arr.len());
}
fn exemplo2() {
    println!("≡ 2 -----------------------------");
    let arr = [10, 20, 30, 40];
    println!("array is {:?}", arr);
    println!("array size is :{}", arr.len());
}
fn exemplo3() {
    println!("≡ 3 -----------------------------");
    let arr: [i32; 4] = [-1; 4];
    println!("array is {:?}", arr);
    println!("array size is :{}", arr.len());
}
fn exemplo4() {
    println!("≡ 4 -----------------------------");
    let arr: [i32; 4] = [10, 20, 30, 40];
    println!("array is {:?}", arr);
    println!("array size is :{}", arr.len());
    for index in 0..4 {
        println!("index is: {} & value is : {}", index, arr[index]);
    }
}
fn exemplo5() {
    println!("≡ 5 -----------------------------");
    let arr: [i32; 4] = [10, 20, 30, 40];
    println!("array is {:?}", arr);
    println!("array size is :{}", arr.len());
    for val in arr.iter() {
        println!("value is :{}", val);
    }
}
fn exemplo6() {
    println!("≡ 6 -----------------------------");
    let mut arr: [i32; 4] = [10, 20, 30, 40];
    arr[1] = 0;
    println!("{:?}", arr);
}
fn exemplo7() {
    println!("≡ 7 -----------------------------");
    let arr = [10, 20, 30];
    update_array(arr);

    print!("Inside main {:?}", arr);
}

fn update_array(mut arr: [i32; 3]) {
    for i in 0..3 {
        arr[i] = 0;
    }
    println!("Inside update {:?}", arr);
}

fn exemplo8() {
    println!("≡ 8 -----------------------------");
    let arr = [10, 20, 30];
    update_by_value(arr);

    print!("Inside main {:?}", arr);
}

fn update_by_value(mut arr: [i32; 3]) {
    for i in 0..3 {
        arr[i] = 0;
    }
    println!("Inside update {:?}", arr);
}

fn exemplo9() {
    println!("≡ 9 -----------------------------");
    let mut arr = [10, 20, 30];
    update_by_ref(&mut arr);

    print!("Inside main {:?}", arr);
}
fn update_by_ref(arr: &mut [i32; 3]) {
    for i in 0..3 {
        arr[i] = 0;
    }
    println!("Inside update {:?}", arr);
    fn exemplo10() {
        println!("≡ 10 -----------------------------");
    }
}

fn exemplo10() {
    println!("≡ 10 -----------------------------");
}