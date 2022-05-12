fn main() {
    let edad: Option<i32> = Some(20);

    match edad {
        Some(value) => println!("Edad: {}", value),
        _ => (),
    }

    if let Some(value) = edad {
        println!("Edad: {}", value);
    }

    let mut unread_message = Some(20);

    loop {
        match unread_message {
            Some(value) => {
                if value > 0 {
                    println!("Tienes {} mensajes no leidos", value);
                    unread_message = Some(value - 1);
                } else {
                    println!("no tienes mensajes sin leer");
                    unread_message = None;
                }
            }

            _ => (break),
        }
    }

    while let Some(value) = unread_message {
        if value > 0 {
            println!("Tienes {} mensajes no leidos", value);
            unread_message = Some(value - 1);
        } else {
            println!("no tienes mensajes sin leer");
            unread_message = None;
        }
    }
}
