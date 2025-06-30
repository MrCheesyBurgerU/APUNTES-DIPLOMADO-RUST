use crate::utils::add;

pub fn integers() {

    // Todas las variables declaradas son imnutables por defecto
    
    // Existen tanto enteros con signo (iX) como enteros sin signo (uX)
    // Ambos varían en potencias de 2, de 8 a 128 bits

    // Se puede utilizar el tamaño de la arquitectura para los enteros
    // isize y usize

    let a = 20;          // Tipo inferido como i32
    let b: i32 = 25;          // Tipo declarado explicitamente
    let c = 15i32;       // Tipo declarado en notación literal
    let d = 40_i32;      // Tipo declarado en notación con subrayado

    let e: i32 = add(add(a, b), add(c, d));

    // println es el macro encargado de encontrar el método exacto para
    // imprimir dependiendo del tipo de dato que se pasa como argumento

    println!("\n====== INTEGERS [TYPES] ======");
    println!("(a + b) + (c + d) = {}", e);
}


pub fn infinity_and_nan() {
    
    // Cualquier operación que involucre nan retorna nan como resultado

    // Los flotantes se definen con fX, solo definidos para 32 y 64
    // Los infinitos se pueden generar mediante operaciones
    // Tambien se pueden usar mediante fx::INFINITY y su negativo

    let infinity: f64 = 1.0 / 0.0; 
    let neg_infinity: f64 = 1.0 / 0.0;
    let nan: f64 = 0.0 / 0.0;

    println!("\n====== INFINITY AND NAN [TYPES] ======");
    println!("1.0 / 0.0 = {}", infinity);
    println!("-1.0 / 0.0 = {}", neg_infinity);
    println!("0.0 / 0.0 = {}", nan);
}


pub fn booleans() {

    // Los tipos booleanos se definen en minúscula

    let is_correct: bool = true;
    let is_incorrect: bool = false;

    println!("\n====== BOOLEANS [TYPES] ======");
    println!("True = {}", is_correct);
    println!("False = {}", is_incorrect);
}


pub fn characters() {
    
    // Como con los otros tipos de datos primitivos, también hay
    // inferencia de tipo

    let character: char = 'z';

    // Tiene soporte nativo para emojis y otros tipos de carácteres
    // Ocupa 4 bytes

    let emoji: char = '💀';

    println!("\n======  CHARACTERS [TYPES] =======");
    println!("Classic character = {}", character);
    println!("Emoji = {}", emoji);
}


pub fn tuples() {

    // Por defecto las tuplas son inmutables, pero se puede modificar 
    // con la palabra reservada mut

    // La tupla también es capaz de inferir tipos de datos

    let tuple: (i32, f64, u8) = (1500, 7.1, 1);

    // Se pueden desempaquetar los valores de una tupla en variables
    // individuales

    let (x, y, z) = tuple;

    // Dado que la tupla es un tipo de dato compuesto, al tratar de
    // imprimirla usando println!({}) se obtiene un error, ya que no se
    // ha implementado el método de impresión

    // Para evitar esto se puede usar el marcador {:?}, el cual permite
    // hacer una impresión en modo de depuración

    // Los valores de la tupla pueden ser accedidos mediante indices
    // usando la notacion tupla.indice

    println!("\n======  TUPLE [TYPES] ======");
    println!("Tuple = {:?}", tuple);
}

