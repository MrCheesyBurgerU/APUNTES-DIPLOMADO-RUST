
pub fn example_string() {

        // String:
    // - Vive en el heap.
    // - Es dueña de sus datos.
    // - Se puede modificar.
    // - Útil para cadenas que cambian o se construyen dinámicamente.

    // Esta función ilustra el uso de `String`, que es una cadena dinámica
    // almacenada en el heap y mutable.

    // `String` es una cadena que puede crecer o modificarse
    // Vive en el heap y es dueña de sus datos

    // Al ser dueño de sus datos significa que la variable es responsable de
    // liberar la memoria asociada a esos datos cuando ya no se usa
    let mut greeting = String::from("Hola");

    // Se puede modificar porque es mutable y vive en el heap
    greeting.push_str(", mundo!");

    // Imprimimos el contenido
    println!("String: {}", greeting);
}


pub fn example_str() {

    // &str:
    // - Referencia a una cadena, vive en el stack o como literal.
    // - No se puede modificar.
    // - Más eficiente para acceso de solo lectura o uso estático.

    // `&str` es una referencia a una cadena, típicamente estática o en el stack.
    let greeting: &str = "Hello";

    // No se puede modificar, ya que es una referencia inmutable.
    // greeting.push_str(", world!"); // ❌ Esto daría error de compilación.

    println!("&str: {}", greeting);
}


pub fn conversions() {

    let my_string = String::from("Rust");

    // Se convierte de `String` a `&str` usando `.as_str()`.
    let str_ref: &str = my_string.as_str();
    println!("String as &str: {}", str_ref);

    // Se convierte de `&str` a `String` usando `.to_string()`.
    let static_str: &str = "Fast";
    let new_string = static_str.to_string();
    println!("&str as String: {}", new_string);
}