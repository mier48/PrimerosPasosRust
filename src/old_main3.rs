fn main() {
    let alberto = Humano;
    println!("{}", alberto.di_hola());
    println!("{}", Humano::idioma());
     
    let guantes = Gato;
    println!("{}", guantes.di_hola());
    println!("{}", Gato::idioma());
}

struct Humano;
struct Gato;

trait Hablar {
    fn di_hola(&self) -> String;
    fn idioma() -> String {
        "No tengo idioma".to_string()
    }
}

impl Hablar for Humano {
    fn di_hola(&self)-> String {
            "Hola Amigos".to_string()
    }

    fn idioma() -> String {
        "Hablo español".to_string()
    }
}

impl Hablar for Gato {
    fn di_hola(&self)-> String {
            "Miau".to_string()
    }

    fn idioma() -> String {
        "no hablo".to_string()
    }
}