fn main() {
    let mut nombre: Option<String> = None;

    match nombre {
        None => println!("Nombre vacio"),
        Some(nombre) => println!("{}", nombre),
    }

    let newUser = User {
        name: "Alberto 2".to_string(),
        age: None,
    };

    let age = newUser.getAge();
    if age > 0 {
        println!("La edad es: {}", age)
    }
}

struct User {
    name: String,
    age: Option<i32>,
}

impl User {
    fn getAge(&self) -> i32 {
        if self.age.is_none() {
            -1
        } else {
            self.age.unwrap()
        }
    }
}
