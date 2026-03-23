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
    println!("Illustration: Using an Enumeration");

    #[derive(Debug)]
    enum GenderCategory {
        Male,
        Female,
    }

    let male = GenderCategory::Male;
    let female = GenderCategory::Female;
    println!("{:?}", male);
    println!("{:?}", female);
}
fn exemplo2() {
    println!("≡ 2 -----------------------------");
    println!("Struct and Enum");

    // The `derive` attribute automatically creates the implementation
    // required to make this `enum` printable with `fmt::Debug`.
    #[derive(Debug)]
    enum GenderCategory {
        Male,
        Female,
    }
    // The `derive` attribute automatically creates the implementation
    // required to make this `struct` printable with `fmt::Debug`.
    #[derive(Debug)]
    struct Person {
        name: String,
        gender: GenderCategory,
    }

    let p1 = Person {
        name: String::from("Mohtashim"),
        gender: GenderCategory::Male,
    };
    let p2 = Person {
        name: String::from("Amy"),
        gender: GenderCategory::Female,
    };

    println!("{:?}", p1);
    println!("{:?}", p2);
}
fn exemplo3() {
    println!("≡ 3 -----------------------------");
    println!("Option Enum");
    let result = is_even(3);
    println!("{:?}", result);
    println!("{:?}", is_even(30));
}

fn is_even(no: i32) -> Option<bool> {
    if no % 2 == 0 { Some(true) } else { None }
}

fn exemplo4() {
    println!("≡ 4 -----------------------------");
    println!("Match Statement and Enum");
    print_size(CarType::SUV);
    print_size(CarType::Hatch);
    print_size(CarType::Sedan);
}

enum CarType {
    Hatch,
    Sedan,
    SUV,
}
fn print_size(car: CarType) {
    match car {
        CarType::Hatch => {
            println!("Small sized car");
        }
        CarType::Sedan => {
            println!("medium sized car");
        }
        CarType::SUV => {
            println!("Large sized Sports Utility car");
        }
    }
}

fn exemplo5() {
    println!("≡ 5 -----------------------------");
    println!("Match with Option");
    match is_evennn(4) {
        Some(data) => {
            if data == true {
                println!("Even no {:?}", data);
            }
        }
        None => {
            println!("not even");
        }
    }
}
fn is_evennn(no: i32) -> Option<bool> {
    if no % 2 == 0 { Some(true) } else { None }
}

fn exemplo6() {
    println!("≡ 6 -----------------------------");
    println!("Match & Enum with Data Type");

    // The `derive` attribute automatically creates the implementation
    // required to make this `enum` printable with `fmt::Debug`.
    #[derive(Debug)]
    enum GenderCategory {
        Name(String),
        Usr_ID(i32),
    }

    let p1 = GenderCategory::Name(String::from("Mohtashim"));
    let p2 = GenderCategory::Usr_ID(100);
    println!("{:?}", p1);
    println!("{:?}", p2);
    match p1 {
        GenderCategory::Name(val) => {
            println!("{}", val);
        }
        GenderCategory::Usr_ID(val) => {
            println!("{}", val);
        }
    }
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
