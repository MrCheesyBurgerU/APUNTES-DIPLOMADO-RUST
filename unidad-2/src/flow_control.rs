
pub fn simple_if_else(x: i32) {

    // if sencillo con if else, si una condición es verdadera imprime algo y si no imprime otra cosa

    if x == 0 || x == 1 {
        println!("x is either 0 or 1");
    } 
    else {
        println!("x is something else");
    }
}


pub fn if_else_expression(x: i32) {

    // Asignación haciendo uso de if else, esto se llama expresión en brazo  
    // Los tipos de la expresión de brazo deben ser exactamente los mismos

    let result = if x > 10 {100} else {200};

    println!("The value of result is: {}", result);
}


pub fn match_example(x: i32) {

    // Uso de match para evaluar múltiples valores posibles
    // Es similiar a un switch en otros lenguajes
    // Permite operaciones sobre rangos, disyunciones, etc

    match x {
        
        0 => println!("x is zero"),
        1 => println!("x is one"),
        2 | 3 => println!("x is two or three"), // varios valores posibles
        4..=10 => println!("x is between 4 and 10"), // rango inclusivo
        _ => println!("x is something else"), // caso por defecto

        // Se pueden hacer rangos exclusivos quitando el igual al final del declarador
        // de rango. El lado izquierdo siempre es inclusivo

        // El o solo se puede hacer sobre patrones estaticos, osea sobre condiciones que
        // son directas, nada de menores o cosas de ese estilo
    }
}


pub fn is_vowel() {

    let letter: char = 'e';

    // Aca se muestra otor ejemplo haciendo match con una expresión disyunta
    match letter {
        'a' | 'e' | 'i' | 'o' | 'u' => println!("Vowel"),
        _ => println!("Consonant")
    }
}


fn classify_number(x: i32) {

    // Se pueden usar condiciones compuestas jugando con if
    // Se debe dejar la variable evaluada como retorno del if

    // Clasifica si el número es cero, par positivo, par negativo
    // o si es impar o no aplica

    match x {
        n if n == 0 => println!("Es cero"),
        n if n < 0 && n % 2 == 0 => println!("Es par negativo"),
        n if n > 0 && n % 2 == 0 => println!("Es par positivo"),
        _ => println!("No es par (es impar o no aplica)"),
    }
}





