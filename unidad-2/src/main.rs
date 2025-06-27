fn main() {
    integer_types();
    infinite_and_nan();
    vector_index();
    the_numbers_are_not_auto_converted();
    as_is_insecure();
    try_into_and_unwrap_usage();
    immutable_variables_by_default();
    shadowing();
    boolean_types();
    characters();
    tuples();
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

// iX Enteros con signo, de 8 a 128 bits
// uX Enteros sin signo, de 8 a 128 bits
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
    
    // === DESCOMENTAR PARA VER LA EJECUCION ===

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


fn try_into_and_unwrap_usage() {

    // Uso de try into para asegurarse si se puede convertir

    // === DESCOMENTAR PARA VER LA EJECUCION ===

    //let value: i32 = 1500;
    //let result: i8 = value.try_into().unwrap();

    //println!("\nDemonstration that as is unsafe:");
    //println!("Value = {}", value);
    //println!("Result = {}", result);

    // Se genera un error con retorno de exito debido a que 
    // la conversion no es segura. Unwrap obtiene el resultado si 
    // la conversion es exitosa o genera panico si la conversion falla.

    // Panico es la forma controlada en la cual rust falla en tiempo de 
    // ejecucion

    // Al generarses panico se interrumpe la ejecucion normal del programa
    // Es casi como una excepcion, solo que rust no maneja exepciones de 
    // manera explicita como otros lenguajes
}


fn immutable_variables_by_default() {
    
    // Al usar let la variable se declara como inmutable
    // Al intentar mutar hay error,

    // === DESCOMENTAR PARA VER LA EJECUCION ===

    // let immutable: i32 = 1500; 
    // immutable = 200;

    // Uso _ para silenciar el warning generado por el no cambio
    // de valor de la variable
    let mut _mutable: i32 = 1500;
    _mutable = 200;

    println!("\nDemonstration let mut:");
    println!("Value = {}", _mutable);
    println!("Change = {}", _mutable);

    // Las constantes definidas con const siempre son inmutables,
    // no se puede cambiar la propiedad porque son constantes
    // Por convencion se usan mayusculas const VALUE
}


fn shadowing() {

    // Variable declarada sobre el scope exterior
    let mut cont = 0;
    cont = cont + 2;

    println!("\nDemonstration shadowing:");

    // Se pueden generar scopes con corchetes
    {
        // Dentro del nuevo scope, la variable eclipsa a la variable original
        // Esto es, se apropia de su uso hasta que se cierre el alcance del scope
        let mut cont: i32 = cont * 5;
        cont = cont + 1;
        println!("Cont value (inner scope) = {}", cont);
    }

    // El scope exterior no se ve afectado por las operaciones internas
    println!("Cont value (outer scope) = {}", cont);
}


fn boolean_types() {
    let is_correct = true;
    let is_incorrect = false;

    println!("\nBool datatype:");
    println!("True = {}", is_correct);
    println!("False = {}", is_incorrect);
}


fn characters() {

    // Tambien ocurre inferencia de tipos, como con otros primitivos
    let c = 'z';
    let z: char = 'z';

    // Soporta emojis y otros tipos de datos desde que se respete el
    // limite de tamaño 4 bytes / 32 bits

    println!("\nChar datatype:");
    println!("Inferred = {}", c);
    println!("Explicit = {}", z);
}


fn tuples() {

    // Definicion de una tupla usando los tipos explicitos
    // La tuplas son inmutables, entonces no se puede cambiar contenido
    // Aun asi, si la tupla se define con let mut se vuelve mutable
    let tuple: (i32, f64, u8) = (1500, 7.1, 1);

    // Tambien se puede hacer inferencia de tipos para las tuplas
    // A diferencia de arriba, el 1 se infirio como un entero basico 
    let tuple2 = (1500, 7.1, 1);

    // Se pueden desempaquetar los valores de la tupla en variables
    let (x, y, z) = tuple2;

    // Se puede desempaquetar infiriendo tipos, no es necesario definirlos
    // explicitamente

    println!("\nTuple (Composite Type):");

    // Si se usa println con {} obtenemos el erros, porque el macro no sabe como
    // imprimir la tupla, no tiene el metodo implementado

    // Se usa el marcado {:?} el cual permite hacer una impresion en modo
    // depuracion, lo muestra tal cual viene
    println!("Tuple = {:?}", tuple);

    // Se puedne imprimir los valores desempaquetados
    println!("x = {}, y = {}, z = {}", x, y, z);

    // Puedo acceder a las tuplas mediante indices, de forma clasica
    // Se usa . para acceder a los indices en tuplas
    println!("x (by index) = {}", tuple2.0);
}






