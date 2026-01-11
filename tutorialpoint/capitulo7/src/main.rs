fn main() {
    println!("String Literal (&str)");
    println!("String Object (String)");

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
    exemplo11();
    exemplo12();
    exemplo13();
    exemplo14();
    exemplo15();
}
fn exemplo1() {
    println!("Exemplo 1: --------------------------------------");
    let company: &str = "TutorialsPoint";
    let location: &str = "Hyderabad";
    println!("company is : {} location :{}", company, location);
}
fn exemplo2() {
    println!("Exemplo 2: --------------------------------------");
    let company: &'static str = "TutorialsPoint";
    let location: &'static str = "Hyderabad";
    println!("company is : {} location :{}", company, location);
}

fn exemplo3() {
    println!("Exemplo 3: --------------------------------------");
    let empty_string = String::new();
    println!("length is {}", empty_string.len());
    let content_string = String::from("TutorialsPoint");
    println!("length is {}", content_string.len());
}

fn exemplo4() {
    println!("Exemplo 4: --------------------------------------");
    let mut z = String::new();
    z.push_str("hello ");
    z.push_str("push str");
    println!("{}", z);
}

fn exemplo5() {
    println!("Exemplo 5: --------------------------------------");
    let name1 = "Hello TutorialsPoint , Hello!".to_string();
    println!("{}", name1);
}
fn exemplo6() {
    println!("Exemplo 6: --------------------------------------");

    let name1 = "Hello TutorialsPoint , Hello!".to_string(); //String object
    let name2 = name1.replace("Hello", "Howdy"); //find and replace
    println!("{}", name2);
}

fn exemplo7() {
    println!("Exemplo 7: --------------------------------------");
    let example_string = String::from("example_string");
    print_literal(example_string.as_str());
}
fn print_literal(data: &str) {
    println!("displaying string literal {}", data);
}

fn exemplo8() {
    println!("Exemplo 8: --------------------------------------");
}

fn exemplo9() {
    println!("Exemplo 9: --------------------------------------");
}
fn exemplo10() {
    println!("Exemplo 10: --------------------------------------");
}

fn exemplo11() {
    println!("Exemplo 11: --------------------------------------");
}
fn exemplo12() {
    println!("Exemplo 12: --------------------------------------");
}

fn exemplo13() {
    println!("Exemplo 13: --------------------------------------");
}
fn exemplo14() {
    println!("Exemplo 14: --------------------------------------");
}

fn exemplo15() {
    println!("Exemplo 15: --------------------------------------");
}
