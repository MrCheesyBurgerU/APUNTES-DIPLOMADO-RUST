// Un String es una colección dinámica de caracteres Vec<u8>. Utila para
// trabajar texto que crece o que se desea modificar

pub fn create_string() {
    // Creamos un String mutable vacío
    let mut empty: String = String::new();

    // Creamos un String mutable con contenido usando String::from
    let mut greeting: String = String::from("Hello");

    // Creamos un &str (slice de cadena)
    let slice: &str = "Welcome";

    // Convertimos el slice a String usando .to_string()
    let mut converted: String = slice.to_string();

    // Mostramos los valores originales
    println!("Empty string: '{}'", empty);
    println!("Greeting: '{}'", greeting);
    println!("Converted from slice: '{}'", converted);

    // Modificamos los strings
    empty.push_str("Now I have text!");
    greeting.push_str(", world!");
    converted.push_str(" to Rust!");

    // Mostramos los resultados modificados
    println!("Updated empty: '{}'", empty);
    println!("Updated greeting: '{}'", greeting);
    println!("Updated converted: '{}'", converted);
}


