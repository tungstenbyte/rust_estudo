fn main() {
    exemplo1();
    exemplo2();
    exemplo3();
    exemplo4();
    exemplo5();
}
fn exemplo1() {
    println!("Exemplo1 -------------------------------------");
    const USER_LIMIT: i32 = 100; // Declare a integer constant
    const PI: f32 = 3.14; //Declare a float constant
    println!("user limit is {}", USER_LIMIT); //Display value of the constant
    println!("pi value is {}", PI); //Display value of the constant    
}

fn exemplo2() {
    println!("Exemplo2 -------------------------------------");
    let salary = 100.00;
    let salary = 1.50; // reads first salary
    println!("The value of salary is :{}", salary);
}

fn exemplo3() {
    println!("Exemplo3 -------------------------------------");
    let uname = "Mohtashim";
    let uname = uname.len();
    println!("name changed to integer : {}", uname);
}

fn exemplo4() {
    println!("Exemplo4 -------------------------------------");
    let uname: &str = "Mohtashim";
    let uname: usize = uname.len();
    println!("name changed to integer : {}", uname);
}
fn exemplo5() {
    println!("Exemplo5 -------------------------------------");
    // const NAME: &str = "Mohtashim";
    // const NAME: usize = NAME.len(); //Error : `NAME` already defined
    // println!("name changed to integer : {}", NAME);
}
