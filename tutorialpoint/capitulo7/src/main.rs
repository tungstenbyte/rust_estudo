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
    let mut company: &str = "TutorialsPoint";
    let mut location: &str = "Hyderabad";
    println!("company is : {} location :{}", company, location);

    company = "outra company";
    location = "outra localizacao";
    println!("company is : {} location :{}", company, location);


}
fn exemplo2() {
    println!("Exemplo 2: --------------------------------------");
    let mut company: &'static str = "TutorialsPoint";
    let mut location: &'static str = "Hyderabad";
    println!("company is : {} location :{}", company, location);


    company = "outra company";
    location = "outra localizacao";
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
    let mut company = "Tutorial".to_string();
    company.push('s');
    println!("{}", company);
}

fn exemplo9() {
    println!("Exemplo 9: --------------------------------------");
    let mut company = "Tutorials".to_string();
    company.push_str(" Point");
    println!("{}", company);
}
fn exemplo10() {
    println!("Exemplo 10: --------------------------------------");
    let fullname = " Tutorials Point";
    println!("length is {}", fullname.len());
}

fn exemplo11() {
    println!("Exemplo 11: --------------------------------------");
    let fullname = " Tutorials Point \r\n";
    println!("Before trim ");
    println!("length is {}", fullname.len());
    println!();
    println!("After trim ");
    println!("length is {}", fullname.trim().len());
}
fn exemplo12() {
    println!("Exemplo 12: --------------------------------------");
    let msg = "Tutorials Point has good tutorials".to_string();
    let mut i = 1;
    for token in msg.split_whitespace() {
        println!("token {} {}", i, token);
        i += 1;
    }
}

fn exemplo13() {
    println!("Exemplo 13: --------------------------------------");

    let fullname = "Kannan,Sudhakaran,Tutorialspoint";

    for token in fullname.split(",") {
        println!("token is {}", token);
    }

    //store in a Vector
    println!("\n");
    let tokens: Vec<&str> = fullname.split(",").collect();
    println!("firstName is {}", tokens[0]);
    println!("lastname is {}", tokens[1]);
    println!("company is {}", tokens[2]);
}
fn exemplo14() {
    println!("Exemplo 14: --------------------------------------");
    let number = 2020;
    let number_as_string = number.to_string(); // convert number to string
    println!("{}", number_as_string);
    println!("{}", number_as_string == "2020");
}

fn exemplo15() {
    println!("Exemplo 15: --------------------------------------");
    let n1 = "Tutorials".to_string();
    let n2 = "Point".to_string();
    let n3 = format!("{} {}", n1, n2);
    println!("{}", n3);
}
