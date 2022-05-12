fn main() {
    let num = 6;
    if num == 5 {
        println!("es cinco");
    } else if num == 6 {
        println!("es seis");
    } else {
        println!("no es ni cinco ni seis");
    }

    let result = if num > 5 { 1000 } else { 0 };
    println!("res: {}", result);

    let mut cont = 0;
    let res = loop {
        if cont == 100 {
            break cont;
        }

        cont += 1;
    };

    println!("Contador: {}", res);

    while cont < 1000 {
        println!("Contador: {}", cont);
        cont += 1;
    }

    let mut arr = [1, 21, 3, 3, 44, 5];
    let var = arr.sort();
    for i in (0..198).rev() {
        println!("Contador: {}", i);
    }
}
