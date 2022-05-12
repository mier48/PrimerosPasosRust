use std::fmt::write;

use std::fmt;

#[derive(Debug)]
struct User {
    nombre: String,
    edad: i32
}


impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} ({})", self.nombre, self.edad)
    }
}
// impl std::fmt::Debug for User {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Usuario {}, tiene {} años.", self.nombre, self.edad)
//     }
// }

fn main() {
    let usuario = User {
        nombre: "Alberto Mier".to_string(),
        edad: 26,
    };

    println!("Hola {}", usuario);
    println!("Hola {:?}", usuario);
}