
pub fn greet() {

    // Esta función no retorna ningún valor
    // Su tipo de retorno es `unit`, representado por `()`.
    
    println!("Hola desde una función sin retorno"); // retorna `()`
}


pub fn add_with_return(x: i32, y: i32) -> i32 {

    // Esta función usa `return` para devolver la suma de dos números enteros.
    // El tipo de retorno se especifica con `-> i32`.
    
    return x + y;
}


pub fn add_without_return(x: i32, y: i32) -> i32 {
    
    // Esta función retorna el resultado sin usar `return`, simplemente dejando la expresión sin `;`.
    // En Rust, la última expresión sin punto y coma se devuelve.
    
    x + y // expresión retornada implícitamente
}


pub fn divide_and_residue(x: i32, y: i32) -> (i32, i32) {
    
    // Esta función retorna una tupla `(cociente, residuo)` de una división entera.
    
    (x / y, x % y) // retorno implícito
}


pub fn fibonacci(x: i32) -> i32 {

    // Esta función calcula fibonacci de forma recursiva
    // La función se llama a si misma 

    // Asumiendo que fibonacci(0) = fibonacci(1) = 1

    if x == 0 || x == 1 {
        1
    }
    else {
        fibonacci(x - 1) + fibonacci(x - 2)
    }
}
