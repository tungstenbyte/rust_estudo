fn main() {
    exemplo1();
    exemplo2();

}
fn exemplo1() {
    println!("Exemplo1 -------------------------------------");
    let fees = 25_000;
    let salary: f64 = 35_000.00;
    println!("fees is {} and salary is {}", fees, salary);
}

fn exemplo2() {
    println!("Exemplo2 -------------------------------------");
    let mut fees: i32 = 25_000;
    println!("fees is {} ", fees);
    fees = 35_000;
    println!("fees changed is {}", fees);
}

