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

struct Employee {
    name: String,
    company: String,
    age: u32,
}

fn exemplo3() {
    println!("≡ 3 -----------------------------");
    println!("Passing a struct to a function");

    let emp1 = Employee {
        company: String::from("TutorialsPoint"),
        name: String::from("Mohtashim"),
        age: 50,
    };
    let emp2 = Employee {
        company: String::from("TutorialsPoint"),
        name: String::from("Kannan"),
        age: 32,
    };

    display1(emp1);
    display1(emp2);
}

fn display1(emp: Employee) {
    println!(
        "Name is :{} company is {} age is {}",
        emp.name, emp.company, emp.age
    );
}

fn who_is_elder(emp1: Employee, emp2: Employee) -> Employee {
    if emp1.age > emp2.age {
        return emp1;
    } else {
        return emp2;
    }
}

fn exemplo4() {
    println!("≡ 4 -----------------------------");
    println!("Returning struct from a function");
    let emp1 = Employee {
        company: String::from("TutorialsPoint"),
        name: String::from("Mohtashim"),
        age: 50,
    };
    let emp2 = Employee {
        company: String::from("TutorialsPoint"),
        name: String::from("Kannan"),
        age: 32,
    };

    let elder = who_is_elder(emp1, emp2);
    println!("elder is:");

    display1(elder);
}

//define dimensions of a rectangle
struct Rectangle {
    width: u32,
    height: u32,
}
//logic to calculate area of a rectangle
impl Rectangle {
    fn area(&self) -> u32 {
        //use the . operator to fetch the value of a field via the self keyword
        self.width * self.height
    }
}

fn exemplo5() {
    println!("≡ 5 -----------------------------");
    println!("Method in Structure");

    let small = Rectangle {
        width: 10,
        height: 20,
    };

    println!(
        "width is {} height is {} area of Rectangle is{}",
        small.width,
        small.height,
        small.area()
    );
}

//declare a structure
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    //static method that creates objects of the Point structure
    fn getInstance(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
    //display values of the structure's field
    fn display(&self) {
        println!("x ={} y={}", self.x, self.y);
    }
}

fn exemplo6() {
    println!("≡ 6 -----------------------------");
    println!("Static Method in Structure");
    // Invoke the static method
    let p1 = Point::getInstance(10, 20);
    p1.display();
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
