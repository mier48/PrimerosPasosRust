//structs
struct Usuario {
    nombre: String,
    nacimiento: i32,
    email: String,
    activo: bool,
    user_role: UserRole,
    website: Website,
}

impl Usuario {
    fn edad(&self) -> i32 {
        let actual = 2022;
        actual - self.nacimiento
    }
}

//Enum = enumeration
enum UserRole {
    BASIC,
    ADMIN,
}

enum Website {
    URL(String),
    INSTAGRAM(String),
    LINKEDIN(String),
    FACEBOOK(String),
}
fn main() {
    let mut user = Usuario {
        nombre: "Alberto".to_string(),
        nacimiento: 1995,
        email: String::from("xxxxx@ccccc.edeee"),
        activo: true,
        user_role: UserRole::ADMIN,
        website: Website::URL(String::from("Prueba 1")),
    };

    println!("Usuario: {}, edad: {}", user.nombre, user.nacimiento);

    user.activo = false;

    //shoirthand init
    let user1 = nuevo_usuario(String::from("Prueba 1"), String::from("xxxxx@ccccc.edeee"));
    let user2 = Usuario {
        nombre: "Corsair".to_string(),
        email: "fehjfhijekg".to_string(),
        ..user1
    };

    //tuple structs
    struct Point(i32, i32, i32);

    let pointA = Point(12, 34, 67);

    let edad = user.edad();
    println!("Usuario: {}, edad: {}, rol:", user.nombre, user.edad());

    let access = hasAccess(user.user_role);
}

fn nuevo_usuario(nombre: String, email: String) -> Usuario {
    return Usuario {
        nombre: nombre,
        nacimiento: 1954,
        email: email,
        activo: false,
        user_role: UserRole::BASIC,
        website: Website::FACEBOOK(String::from("Prueba 34")),
    };
}

fn hasAccess(userRole: UserRole) -> bool {
    match userRole {
        UserRole::ADMIN => true,
        UserRole::BASIC => false,
    }
}

fn go_to_website(website: Website) {
    match website {
        Website::FACEBOOK => "",
        Website::INSTAGRAM => "",
        Website::LINKEDIN => "",
        Website::URL => "",
    }
}
