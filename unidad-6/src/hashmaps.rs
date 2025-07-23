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
