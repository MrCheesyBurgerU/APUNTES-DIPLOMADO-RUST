
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

    // Aca se muestra otro ejemplo haciendo match con una expresión disyunta
    match letter {
        'a' | 'e' | 'i' | 'o' | 'u' => println!("Vowel"),
        _ => println!("Consonant")
    }
}


pub fn classify_number(x: i32) {

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


pub fn classify_tuple() {

    // También se puede usar match para trabajar con otros tipos de datos
    let tuple: (i32, i32) = (4, -6);

    // Match tiene la capacidad de desempaquetar los tipos de datos
    match tuple {
        (x, y) if x >= 0 && y >= 0 => println!("Está en el cuadrante 1"),
        _ => println!("No está en el cuadrante 1"),
    }
}


pub fn classify_direction() {

    // Clasificacion exactamente sobre los ejes
    let point: (i32, i32) = (0, 5); 

    match point {
        (0, y) if y > 0 => println!("Está en el norte"),
        (0, y) if y < 0 => println!("Está en el sur"),
        (x, 0) if x > 0 => println!("Está en el este"),
        (x, 0) if x < 0 => println!("Está en el oeste"),
        (0, 0) => println!("Está en el origen"),
        _ => println!("Está fuera de los ejes cardinales"),
    }

    // Si donde estan los ceros ponemos _ entonces ese valor se ignora
    // en el match, osea que cualquier número hace el match
}


pub fn for_in_loop() {

    // Permite iterar sobre los elementos que se encuentran en 
    // un objeto o tipo iterable

    // Se pueden usar rangos exclusivos .. o rangos inclusivos ..=
    // La parte izquierda siempre es inclusiva

    for item in 0..10 {
        if item == 5 {
            println!("Salta = {}", item);
            continue;
        }
        println!("Número iterado = {}", item);
    }

    // Ahora tambien se puede hacer un for inclusivo

    for item in 0..=10 {
        if item == 5 {
            println!("Salta = {}", item);
            continue;
        }
        println!("Número iterado = {}", item);
    }
}


pub fn while_loop() {

    // Se declara mutable para poder modificarla
    let mut number = 10;

    // Imprime de 10 a 1, que es cuando se deja de cumplir la condición
    while number != 0 {
        println!("Número = {}", number);
        number -= 1;
    }
}


pub fn pure_loop() {

    // Para imprimir desde el cero hasta el nueve
    let mut number = 0;

    // Se pueden hacer loop puros, donde la condicion de parada la define
    // un break
    loop {
        println!("Número = {}", number);
        if number == 10 {
            break;
        }
        number += 1;
    }
}


pub fn pure_loop_with_return() -> i32 {

    // Se declara la variable mutable
    let mut number = 0;

    // El loop itera hasta el 10, pero en este caso lo retorna
    // El valor retornado se almacena en la variable result
    let result = loop {
        println!("Número = {}", number);
        if number == 10 {
            // Retorna `number` como valor del loop
            break number 
        }
        number += 1;
    };

    // Retorna el valor resultante del loop
    result 
}
