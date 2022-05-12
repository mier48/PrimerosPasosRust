

fn main() {
    let sum = |num: i32, num2: i32| -> i32 {
        num + num2 + 1
    };

    // let sum = sumar_uno;
    //println!("{}", sum(4, 32));

    let mut cont = 1;
    let mut add = move || {
        cont += 1;
        println!("{}", cont)
    };
   
    let var = &cont;
    add();
    println!("var: {}", var)
}

fn sumar_uno(num: i32) -> i32 {
    num + 1
}