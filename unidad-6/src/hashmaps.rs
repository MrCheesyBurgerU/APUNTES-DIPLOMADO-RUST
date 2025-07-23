use std::collections::HashMap;

// Utiles para almacenar parejas llave valor y buscar valores
// por una clave o llave, no por un indice

pub fn create_hashmap() {
    // Creamos un HashMap mutable que asocia String (nombre) con u32 (edad)
    let mut people: HashMap<String, u32> = HashMap::new();

    // Insertamos algunos valores
    people.insert("Alice".to_string(), 30);
    people.insert("Bob".to_string(), 25);
    people.insert("Charlie".to_string(), 40);

    // Recorremos el mapa e imprimimos las parejas clave-valor
    for (name, age) in &people {
        println!("{} is {} years old", name, age);
    }
}


pub fn query_hashmap() {
    // Creamos e inicializamos el HashMap
    let mut people: HashMap<String, u32> = HashMap::new();
    people.insert("Alice".to_string(), 30);
    people.insert("Bob".to_string(), 25);

    // Nombre a consultar
    let name_to_query = "Alice";

    // Realizamos la consulta usando .get()
    match people.get(name_to_query) {
        Some(age) => println!("{} is {} years old", name_to_query, age),
        None => println!("{} is not in the list", name_to_query),
    }

    // Otra consulta con un nombre que no está
    let name_to_query = "Charlie";
    match people.get(name_to_query) {
        Some(age) => println!("{} is {} years old", name_to_query, age),
        None => println!("{} is not in the list", name_to_query),
    }
}


pub fn query_hashmap_with_unwrap() {
    // Creamos e inicializamos el HashMap
    let mut people: HashMap<String, u32> = HashMap::new();
    people.insert("Alice".to_string(), 30);
    people.insert("Bob".to_string(), 25);

    // Consulta segura con unwrap: sabemos que "Bob" está en el mapa
    let age_bob = people.get("Bob").unwrap();
    println!("Bob is {} years old", age_bob);

    // Consulta insegura: Charlie no está en el mapa, unwrap causará panic si se descomenta
    // let age_charlie = people.get("Charlie").unwrap(); // esto causaría un panic

    // En cambio, para claves inciertas usamos match para evitar errores
    let name_to_query = "Charlie";
    match people.get(name_to_query) {
        Some(age) => println!("{} is {} years old", name_to_query, age),
        None => println!("{} is not in the list", name_to_query),
    }
}


pub fn entry_or_insert_example() {
    // Creamos e inicializamos un HashMap mutable
    let mut scores: HashMap<String, u32> = HashMap::new();

    // Insertamos un valor por primera vez usando entry + or_insert
    scores.entry("Alice".to_string()).or_insert(50);

    // Intentamos insertar un valor para Alice, pero como ya existe, no se sobrescribe
    scores.entry("Alice".to_string()).or_insert(80);

    // Insertamos un nuevo valor para una clave que no estaba
    scores.entry("Bob".to_string()).or_insert(70);

    // Mostramos el resultado
    for (name, score) in &scores {
        println!("{} has score {}", name, score);
    }
}


pub fn hashmap_ownership_and_borrowing() {
    // Creamos dos Strings que vamos a insertar en el HashMap
    let key = String::from("Alice");
    let value = String::from("Engineer");

    // Creamos un HashMap mutable
    let mut people: HashMap<String, String> = HashMap::new();

    // --- Transferencia de propiedad ---
    // Insertamos key y value en el mapa: la propiedad se mueve al HashMap
    people.insert(key, value);

    // key y value ya no se pueden usar después de este punto
    // println!("{}", key);     
    // println!("{}", value);  

    // --- Préstamo ---
    // Consultamos por referencia sin mover la propiedad
    if let Some(profession) = people.get("Alice") {
        println!("Alice is a {}", profession);
    }

    // --- Préstamo mutable ---
    // Modificamos el valor asociado a una clave usando entry + or_insert_mutable
    if let Some(job) = people.get_mut("Alice") {
        job.push_str(" (Rust developer)");
    }

    // Mostramos el contenido final del HashMap
    for (name, profession) in &people {
        println!("{}: {}", name, profession);
    }
}
