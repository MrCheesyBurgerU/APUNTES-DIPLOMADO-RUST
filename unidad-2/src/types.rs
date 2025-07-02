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


pub fn arrays() {

    // Los arrays tienen tamaño fijo conocido en tiempo de compilación.
    // Todos los elementos deben ser del mismo tipo.

    // Se puede especificar el tipo y la longitud
    let a: [i32; 4] = [10, 20, 30, 40];

    // También pueden ser inferidos automáticamente
    let _b = [1, 2, 3];

    // Inicialización repetida: valor, número de repeticiones
    let _c = [0; 5]; // [0, 0, 0, 0, 0]

    // Los elementos pueden accederse por índice
    let first = a[0];
    let last = a[a.len() - 1];

    // Al imprimir un array, también se puede usar el marcador {:?}
    println!("\n======  ARRAY [TYPES] ======");
    println!("Array a = {:?}", a);
    println!("Primer valor de a = {}", first);
    println!("Último valor de a = {}", last);
}


pub fn structs() {
    
    // Una struct permite definir tipos personalizados con campos nombrados.
    // Se puede declarar dentro de la función para ejemplos simples
    
    struct Persona {
        nombre: String,
        edad: u8,
    }

    // Instanciación de la estructura
    let persona: Persona = Persona {
        nombre: String::from("Alice"),
        edad: 30,
    };

    // Acceso a los campos con notación de punto
    println!("\n======  STRUCT [TYPES] ======");
    println!("Nombre: {}", persona.nombre);
    println!("Edad: {}", persona.edad);

    // Las structs también pueden imprimirse con {:?} si derivan Debug
    // Para imprimir directamente, deberíamos derivar #[derive(Debug)]
}


pub fn enums() {

    // Los enums permiten definir un tipo que puede ser una de varias variantes.
    enum Estado {
        Activo,
        Inactivo,
        Pendiente(u8), // También pueden tener datos asociados
    }

    // Ejemplo de uso
    let estado: Estado = Estado::Pendiente(3);

    println!("\n======  ENUM [TYPES] ======");
    match estado {
        Estado::Activo => println!("Estado: Activo"),
        Estado::Inactivo => println!("Estado: Inactivo"),
        Estado::Pendiente(dias) => println!("Estado: Pendiente desde hace {} días", dias),
    }
}