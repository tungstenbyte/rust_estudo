fn main() {
    tipos_numericos();
    uso_de_strings();
    usando_condicionais();
    looping();
    funcoes();
    tuplas();
    destructions();
    arrays_main();
    ownership();
}

fn tipos_numericos() {
    println!("S. No.  Size     Signed   Unsigned");
    println!("1      8 bit    i8       u8");
    println!("2      16 bit   i16      u16");
    println!("3      32 bit   i32      u32");
    println!("4      64 bit   i64      u64");
    println!("5      128 bit  i128     u128");
    println!("6      Arch     isize    usize");
}

fn uso_de_strings() {
    println!("Literal: &str --> Conhecido em tempo de compilação.");
    println!("Object: String::new --> Mutable UTF8 Runtime");

    let comapy: &str = "Holdfy aqui";
    let other: &'static str = "conteudo estatico aqui";

    let mut z: String = String::new();
    z.push_str("Hello");

    let name: String = "Silvio esta por aqui".to_string();
    let example_string = String::from("mais alguns textos");

    let numero: i32 = 2026;
    let texto: String = numero.to_string();

    println!("company : {}", comapy);
    println!("other : {}", other);
    println!("z : {}", z);
    println!("name : {}", name);
    println!("example_string : {}", example_string);
    println!("numero : {}", numero);
    println!("texto : {}", texto);
}

fn usando_condicionais() {
    // Exemplo utilizando if
    let num: i32 = 12;

    if num % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }

    // Exemplo utilizando match (semelhante ao switch)

    let uf: &str = "PR";

    let estado: &str = match uf {
        "SC" => {
            println!("Encontrado estado para SC");
            "Santa Catarina"
        }
        "PR" => "Paraná",
        "SP" => "São Paulo",
        "RJ" => "Rio de Janeiro",
        _ => "Desconhecido",
    };

    println!("Estado: {}", estado);
}

fn looping() {
    for x in 1..11 {
        // loop de 1 até 10
        println!("loop for x: {}", x);
    }

    let mut x: i32 = 0; // precisa ser mut porque x vai ser a incrementada

    while x < 10 {
        x += 1;
        println!("loop while x: {}", x);
    }

    let mut y: i32 = 0;

    loop {
        y += 2;

        if y > 10 {
            break;
        }
        println!("loop loop y: {}", y);
    }
}

fn funcoes() {
    let mut x: i32 = 6;
    funcao_by_value(x);
    println!(
        "O valor inicial de x era 6 apos chamar a funcao o valor de x é: {}",
        x
    );

    let mut y: i32 = 3;

    funcao_by_referencia(&mut y);

    println!(
        "O valor inicial de y era 3 apos chamar a funcao o valor de y é: {}",
        y
    );
}

fn funcao_by_value(mut parametro: i32) {
    parametro = parametro * 10;
    println!("Valor de parametro dentro da funcao x10 : {}", parametro);
}

fn funcao_by_referencia(parametro: &mut i32) {
    *parametro = *parametro - 2;
}

fn tuplas() {
    // Tuples sao com () arrays com []
    // o index das tuplas é:     0, 1  , 2
    let tuple: (i32, f64, u8) = (1, 2.5, 3);

    println!("f64 é: {}", tuple.1); // faz referencia ao index 1 que é o segundo elemento (0,1,2) neste caso;
    println!("todos eles {:?}", tuple);
}

fn destructions() {
    let tuple: (i32, f64, u8) = (1, 2.1, 3);
    let (a, b, c) = tuple;
    println!("Destruction da tuple: {:?}", tuple);
    println!(
        "Variaveis criadas a partir do desctruction: a: {}, b: {} e c: {}",
        a, b, c
    );
}

fn arrays_main() {
    // arrays com []  e Tuples sao com ()
    // Arrays sao sequencias

    let x: [i32; 4] = [10, 20, 30, 40];
    for i in 0..4 {
        println!("index: {} value: {}", i, x[i])
    }
    // exemplo de array chamando funcao por passagem de valor
    let arr1 = [10, 20, 30];
    println!("aqui por value: original: {:?}", arr1);
    array_value_update(arr1);
    println!(
        "aqui por value: apos chamar funcao por value, nao deve mudar nada: {:?}",
        arr1
    );

    // exemplo de array chamando funcao por passagem por referencia
    let mut arr2 = [10, 20, 30];
    println!("aqui por referencia: original: {:?}", arr2);
    array_referencia_update(&mut arr2);
    println!(
        "aqui por referencia: apos chamar funcao por value, deve mudar: {:?}",
        arr2
    );
}

fn array_value_update(mut arr: [i32; 3]) {
    for i in 0..3 {
        arr[i] = 0;
    }
    println!("Dentro da funcao por value, com mut: {:?}", arr)
}

fn array_referencia_update(arr: &mut [i32; 3]) {
    for i in 0..3 {
        arr[i] = 1000;
    }
    println!("Dentro da funçao por referencia com :&mut : {:?}", arr)
}

fn ownership() {
    ownership_transfering_of_variable_to_another_variavel();
    owership_passaing_value_to_a_function();
    ownership_retorning_value_from_a_function();
    ownership_tipos_primitivos();
}

fn ownership_transfering_of_variable_to_another_variavel() {
    let v1 = vec![1, 2, 3];
    let v2 = v1;

    // na linha  acima: let v2 = v1; tranferiu-se o ownership, donos do valor,  de v1 para v2
    // assim ao tentar acessar da forma como esta abaixo a variavel que já nao tem mais o ownership
    // do valor, ira dar erro.
    // println!("{:?}", v1);
    println!("{:?}", v2);
}

fn owership_passaing_value_to_a_function() {
    let v1 = vec![1, 2, 3];
    let v2 = v1; // ownership movido para v2

    display_ownership1(v2); // ownership movido para a função

    // println!("{:?}", v2); // erro: v2 não é mais válido aqui
}

fn display_ownership1(v: Vec<i32>) {
    println!("caso ownsership1 {:?}", v);
}

fn ownership_retorning_value_from_a_function() {
    let v = vec![1, 2, 3];
    let v2 = v; // moveu aqui o ownership
    let x = display_ownership2(v2);
    println!("valor do vector retornado da funcao e reatribuido ao x parametro {:?}",x);
}

fn display_ownership2(v: Vec<i32>) -> Vec<i32> {
    println!("caso ownership2 aqui esta o vetor dentro da funcao que ira retornar o mesmo vetor por conta do ownership: {:?}",v);
    v
}
fn ownership_tipos_primitivos(){
    println!("Tipos primitivos copia o valor nao perde o ownership");
    let a = 10;
    let b = a;
    println!("tipos primitivos a: {} e b: {}", a,b);
}