fn main() {
    integer_types();
    infinite_and_nan();
    vector_index();
    the_numbers_are_not_auto_converted();
    as_is_insecure();
}


fn add(x: i32, y: i32) -> i32 {
    return x + y;
}


fn integer_types() {
    
    // Las variables declaradas, por defecto, son inmutables. Para 
    // hacerlas mutables se agrega la palabra reservada mut
    
    // El compilador es capaz de definir tipos
    let a = 20; 
    let b: i32 = 25;

    // Se puede incluir el tipo de dato en la definicion, de forma
    // literal
    let c = 15i32; 
    let d = 40_i32;

    // Aca se esta invocando una función que suma
    let e = add(add(a, b), add(c, d));

    // Macro que se encarga de encontrar el método exacto para 
    // correr sobre el tipo de dato que se está utilizando
    println!("\nInteger types:");
    println!("(a + b) + (c + d) = {}", e)
}


// Tipos numéricos

// iX Enteros con signo, de 8 a 64 bits
// uX Enteros sin signo, de 8 a 64 bits
// fX Flotantes, 32 o 64 bits - Se tiene f64::INFINITY f64::NEG_INFINITY y f64::NAN
// isize - usize, Enteros que asumen el ancho nativo de la CPU


fn infinite_and_nan() {

    // Cualquier operación con NAN retorna un NAN
    // Esto devuelve infinito positivo
    println!("\nEvaluation of infinities and nan:");
    let infinity:f64 = 1.0 / 0.0;
    println!("1.0 / 0.0 = {}", infinity);

    // Esto devuelve infinito negativo
    let neg_infinity: f64 = -1.0 /0.0;
    println!("-1.0 / 0.0 = {}", neg_infinity);

    // Esto devuelve NAN
    let nan: f64 = 0.0 / 0.0;
    println!("0.0 / 0.0 = {}", nan);
}


fn vector_index() {
    
    // Se utiliza la macro de creacion de vector
    // Por defecto los literales enteros son i32
    let vector = vec![1, 2, 3];

    // Se utiliza el tamaño de la arquitectura para evitar
    // desbordamiento
    let index: usize = 2;

    // Se conculta el numero en el indice 2 (0 based)
    let value: i32 = vector[index];

    println!("\nAccess to vector with architecture size:");
    println!("Value in index [2] = {}", value);
}


fn the_numbers_are_not_auto_converted() {

    let edad_a: i64 = 28;
    let edad_b: i32 = 30;

    // No se realiza conversion de tipos de forma automatica
    // Muestra error debido a que no lo puede convertir a i64
    
    // if edad_a < edad_b {
    //    println!("ageA is less than ageB");
    // }

    // Se puede solucionar convirtiendo el i32 a i64
    // Es mas seguro convertir los de menor a los de mayor tamaño
    // De lo contrario se podria estar perdiendo informacion 

    // Arreglando el error
    // As genera un casteo directo siempre que los tipos de datos sean
    // compatibles
    // Al castear con as se le llama promocion, promocion de i32 a i64
    println!("\nInteger conversion with as (unsafe):");
    if edad_a < (edad_b as i64) {
        println!("ageA is less than ageB");
    }
    
    // As no revisa que el casteo sea seguro

    // Si el compilador puede determinar el tipo de destino, se puede
    // usar .into(). Esto genera una conversion segura

    // .into() es un trait, funciona correctamente desde que exista una
    // implementacion para el tipo original, osea el tipo inferido al que
    // se quiere convertir

    println!("\nInteger conversion with into (safe):");
    if edad_a < (edad_b.into()) {
        println!("ageA is less than ageB");
    }
}


fn as_is_insecure() {

    // Conversion de mayor a menor cantidad de bits es insegura
    let value: i32 = 1500;
    let result: i8 = value as i8;

    // El resultado no corresponde con el valor original, la conversion
    // es insegura y se desborda
    println!("\nDemonstration that as is unsafe:");
    println!("Value = {}", value);
    println!("Result = {}", result);
}

