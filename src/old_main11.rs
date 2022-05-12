fn main() {
    let v: Vec<i32> = Vec::new();

    algo();
}

fn algo() {
    let mut v = vec![1, 2, 3];

    v.push(9);
    v.push(19);

    // match v.get(1) {
    //     Some(valor) => println!("valor: {}", valor),
    //     _ => (),
    // }

    // for i in v {
    //     println!("valor: {}", i);
    // }

    for i in &v {
        println!("valor: {}", i);
    }

    for i in &mut v {
        *i += 10;
    }

    for i in &v {
        println!("valor: {}", i);
    }

    enum Mensaje {
        TEXTO(String),
        ERROR(i32),
    }

    let mensajes = vec![Mensaje::TEXTO("Holaaaa".to_string()), Mensaje::ERROR(128)];

    for m in &mensajes {
        match m {
            Mensaje::TEXTO(texto) => println!("valor:  {}", texto),
            Mensaje::ERROR(num) => println!("valor:  {}", num),
        }
    }
}
