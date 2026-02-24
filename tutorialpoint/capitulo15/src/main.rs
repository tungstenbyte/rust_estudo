fn main() {
    exemplo_de_erro();
    exemplo_borrowing_corret();
    exemplo_mutable_integer_reference();
    examplo_muting_string_ref();
    exemplo5();
    exemplo6();
    exemplo7();
    exemplo8();
    exemplo9();
    exemplo10();
}

fn exemplo_de_erro() {
    println!("≡ 1 -----------------------------");
    println!("Aqui acontece o erro e o porque precisamos do borrowing");
    // a list of nos
    let v = vec![10, 20, 30];
    print_vector_first(v);
    // println!("{}",v[0]); // this line gives error
}

fn print_vector_first(x: Vec<i32>) {
    println!("Inside print_vector function {:?}", x);
}

fn exemplo_borrowing_corret() {
    println!("≡ 2 -----------------------------");
    println!("Borrowing Correto");
    let v = vec![10, 20, 30];
    print_vector_corret1(&v); // passing reference
    println!("Printing the value from main() v[0]={}", v[0]);
}
fn print_vector_corret1(x: &Vec<i32>) {
    println!("Inside print_vector function {:?}", x);
}
fn exemplo_mutable_integer_reference() {
    println!("≡ 3 -----------------------------");
    println!("Illustration: Mutating an integer reference");
    let mut i = 3;
    add_one(&mut i);
    println!("{}", i);
}
fn add_one(e: &mut i32) {
    *e += 1;
}

fn examplo_muting_string_ref() {
    println!("≡ 4 -----------------------------");
    println!("Mutating a string reference");

    let mut name: String = String::from("TutorialsPoint");
    display_mutable_string_ref(&mut name); //pass a mutable reference of name
    println!("The value of name after modification is:{}", name);
}
fn display_mutable_string_ref(param_name: &mut String) {
    println!("param_name value is :{}", param_name);
    param_name.push_str(" Rocks"); //Modify the actual string,name
}
fn exemplo5() {
    println!("≡ 5 -----------------------------");
    println!("Mutating a string reference");

    let mut name: String = "TutorialsPoint".to_string();
    display_mutable_string_ref(&mut name); //pass a mutable reference of name
    println!("The value of name after modification is:{}", name);    
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
