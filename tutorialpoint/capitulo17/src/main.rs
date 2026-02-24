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
    println!("Initializing a structure");

    struct Employee {
        name: String,
        company: String,
        age: u32,
    }

    let emp1 = Employee {
        company: String::from("TutorialsPoint"),
        name: String::from("Mohtashim"),
        age: 50,
    };

    println!(
        "Name is :{} company is {} age is{}",
        emp1.name, emp1.company, emp1.age
    );
}
fn exemplo2() {
    println!("≡ 2 -----------------------------");
    println!("Modifying a struct instance");
    struct Employee {
        name: String,
        company: String,
        age: u32,
    }

    let mut emp1 = Employee {
        company: String::from("TutorialsPoint"),
        name: String::from("Mohtashim"),
        age: 50,
    };
    emp1.age = 40;
    println!(
        "Name is :{} company is {} age is{}",
        emp1.name, emp1.company, emp1.age
    );
}
fn exemplo3() {
    println!("≡ 3 -----------------------------");
    println!("Passing a struct to a function");
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
