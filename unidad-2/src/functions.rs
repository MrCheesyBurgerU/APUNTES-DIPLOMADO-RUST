
fn greet() {

    // Esta función no retorna ningún valor
    // Su tipo de retorno es `unit`, representado por `()`.
    
    println!("Hola desde una función sin retorno"); // retorna `()`
}


fn add_with_return(x: i32, y: i32) -> i32 {

    // Esta función usa `return` para devolver la suma de dos números enteros.
    // El tipo de retorno se especifica con `-> i32`.
    
    return x + y;
}


fn add_without_return(x: i32, y: i32) -> i32 {
    
    // Esta función retorna el resultado sin usar `return`, simplemente dejando la expresión sin `;`.
    // En Rust, la última expresión sin punto y coma se devuelve.
    
    x + y // expresión retornada implícitamente
}


fn divide_and_residue(x: i32, y: i32) -> (i32, i32) {
    
    // Esta función retorna una tupla `(cociente, residuo)` de una división entera.
    
    (x / y, x % y) // retorno implícito
}
