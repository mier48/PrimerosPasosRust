fn main() {
    let edad: Option<i32> = Some(15);

    if edad.es_mayor_de_edad() {
        println!("Es mayor de edad");
    }else {
        println!("Es menor de edad");
    }
}

trait LicenciaConducir {
    fn es_mayor_de_edad(&self) -> bool;
}

impl LicenciaConducir for Option<i32> {
    fn es_mayor_de_edad(&self) -> bool {
        match self {
            Some(edad) => edad>&18,
            None => false
        }
    }
}