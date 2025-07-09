// ------------------------------------------------------------
// Diferencia entre `struct` y `enum` en Rust
// ------------------------------------------------------------
// Tanto las estructuras (struct) como los enumeradores (enum)
// permiten agrupar datos y definir métodos con `impl`. Sin embargo,
// tienen propósitos distintos y se usan en situaciones diferentes.
//
// ------------------------------------------------------------
// struct:
// Se usa para agrupar datos que siempre aparecen juntos.
// Todos los campos están presentes al mismo tiempo.
// Ejemplo típico: representar una entidad con sus atributos.
//
// Ejemplo:
// struct Tool {
//     name: String,
//     status: String,
// }
//
// Esta estructura siempre tendrá un nombre y un estado al mismo tiempo.
// Se accede a los campos con el operador punto: tool.name, tool.status.
//
// ------------------------------------------------------------
// enum:
// Se usa cuando un valor puede tomar diferentes formas o variantes.
// Solo una variante está activa en un momento dado.
// Cada variante puede tener datos distintos asociados o ninguno.
//
// Ejemplo:
// enum Message {
//     Text(String),
//     Move(i32, i32),
//     Greet { name: String },
// }
//
// Aquí, un `Message` puede ser texto, o una coordenada, o un saludo.
// Se usa `match` para ejecutar lógica según la variante.
//
// ------------------------------------------------------------
// En resumen:
// - Usa `struct` cuando necesitas representar algo con múltiples
//   propiedades activas al mismo tiempo.
// - Usa `enum` cuando una cosa puede ser de varios tipos, pero solo uno
//   a la vez.
//
// struct → "tiene esto y esto"
// enum   → "es esto o esto"
// ------------------------------------------------------------


#[derive(Debug)] // Permite imprimir cualquier variante del enum
enum Message {
    // Los enums permiten definir un tipo que puede tener distintos valores
    // etiquetados (variantes), cada uno con su propio tipo de dato asociado
    // Útile para representar mútliples tipos de mensajes, eventos o estados 
    // Variante que representa un mensaje de texto
    Text(String),

    // Variante que representa un movimiento con coordenadas x, y
    Move(i32, i32),

    // Variante que representa un saludo con nombre incluido (struct inline)
    Greet { name: String },
}


impl Message {
    // Método para procesar el mensaje según su variante
    pub fn process(&self) {
        match self {
            Message::Text(content) => {
                println!("Processing text message [MESSAGE::TEXT] = {}", content);
            }
            Message::Move(x, y) => {
                println!("Processing move message [MESSAGE::MOVE] = x: {}, y: {}", x, y);
            }
            Message::Greet { name } => {
                println!("Processing greeting [MESSAGE::GREET] = Hello, {}!", name);
            }
        }
    }
}


pub fn enum_usage_example() {
    // Creamos un mensaje de texto
    let msg1 = Message::Text(String::from("Hello, world!"));

    // Creamos un mensaje de movimiento
    let msg2 = Message::Move(10, 25);

    // Creamos un mensaje de saludo con un nombre
    let msg3 = Message::Greet {
        name: String::from("Alice"),
    };

    // Procesamos cada mensaje usando su método
    msg1.process();
    msg2.process();
    msg3.process();

    // También podemos imprimir el enum completo con {:?}
    println!("Enum variant [ENUM USAGE] = {:?}", msg1);
    println!("Enum variant [ENUM USAGE] = {:?}", msg2);
    println!("Enum variant [ENUM USAGE] = {:?}", msg3);
}


// Se puede usar un ejemplo con una estructura mas compleja
// Se pueden anidar estructuras y luego usarlas en los enums
#[derive(Debug)]
struct ComplexContactInfo {
    email: String,
    phone: String,
}


#[derive(Debug)]
struct ComplexUser {
    username: String,
    age: u32,
    is_active: bool,
    contact: ComplexContactInfo,
    nickname: Option<String>,
}


impl ComplexUser {
    pub fn new(username: &str, age: u32, email: &str, phone: &str) -> Self {
        ComplexUser {
            username: username.to_string(),
            age,
            is_active: true,
            contact: ComplexContactInfo {
                email: email.to_string(),
                phone: phone.to_string(),
            },
            nickname: None,
        }
    }

    pub fn set_nickname(&mut self, nickname: &str) {
        self.nickname = Some(nickname.to_string());
    }
}


#[derive(Debug)]
enum ComplexMessage {
    Text(String),
    Move(i32, i32),
    Greet(ComplexUser),
}


impl ComplexMessage {
    // Método para procesar el mensaje según su variante
    pub fn process(&self) {
        match self {
            ComplexMessage::Text(content) => {
                println!("Processing text message [COMPLEX_MESSAGE::TEXT] = {}", content);
            }
            ComplexMessage::Move(x, y) => {
                println!("Processing move message [COMPLEX_MESSAGE::MOVE] = x: {}, y: {}", x, y);
            }
            ComplexMessage::Greet(user) => {
                println!(
                    "Processing greeting [COMPLEX_MESSAGE::GREET] = Hello, {} ({:?})!",
                    user.username, user.nickname
                );
            }
        }
    }
}


pub fn complex_enum_usage_example() {
    // Creamos un mensaje de texto
    let msg1 = ComplexMessage::Text(String::from("Hello, world!"));

    // Creamos un mensaje de movimiento
    let msg2 = ComplexMessage::Move(10, 25);

    // Creamos un usuario para usar en el saludo
    let mut user = ComplexUser::new("rozo", 25, "rozo@example.com", "123-456");
    user.set_nickname("Rozo");

    // Creamos un mensaje de saludo con un usuario completo
    let msg3 = ComplexMessage::Greet(user);

    // Procesamos cada mensaje usando su método
    msg1.process();
    msg2.process();
    msg3.process();

    // También podemos imprimir el enum completo con {:?}
    println!("Enum variant [COMPLEX ENUM USAGE] = {:?}", msg1);
    println!("Enum variant [COMPLEX ENUM USAGE] = {:?}", msg2);
    println!("Enum variant [COMPLEX ENUM USAGE] = {:?}", msg3);
}


// Tambien se pueden poner enums como campos de un struct
// Usandolos como tipos de datos
#[derive(Debug)]
enum Status {
    Active,
    Inactive,
    Banned(String), // Puede incluir una razón
}


#[derive(Debug)]
struct Account {
    username: String,
    status: Status,
}


impl Account {
    pub fn new(username: &str, status: Status) -> Self {
        Account {
            username: username.to_string(),
            status,
        }
    }

    pub fn show(&self) {
        match &self.status {
            Status::Active => println!("User '{}' is active.", self.username),
            Status::Inactive => println!("User '{}' is inactive.", self.username),
            Status::Banned(reason) => {
                println!("User '{}' is banned: {}", self.username, reason)
            }
        }
    }
}


pub fn struct_with_enum_field_example() {
    let account1 = Account::new("alice", Status::Active);
    let account2 = Account::new("bob", Status::Banned("violation of rules".to_string()));

    account1.show();
    account2.show();

    println!("Account full info [STRUCT WITH ENUM FIELD] = {:?}", account1);
    println!("Account full info [STRUCT WITH ENUM FIELD] = {:?}", account2);
}