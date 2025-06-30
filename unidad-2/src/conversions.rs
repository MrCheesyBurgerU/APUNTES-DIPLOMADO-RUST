
pub fn as_number_conversion() {

    // En rust no hay conversión de tipos de forma automática
    // Siempre se debe específicar la conversión o se genera error

    // Como buena práctica los números se deben convertir al de mayor
    // tamaño, para evitar desbordamientos

    let number_1: i64 = 28;
    let number_2: i32 = 32;

    // Si aca no se realizara la conversion, se presenta un error
    // La conversión llevada a cabo con as se llama promoción

    // as siempre realiza la conversión desde que los tipos de datos sean
    // compatibles. No es seguro ya que se pueden generar desbordamientos

    println!("\n====== AS CONVERSION [CONVERSIONS] ======");
    if number_1 < (number_2 as i64) {
        print!("number_1 was converted from i32 to i64")
    }

    // Para demostrar que as es inseguro se puede convertir de un tipo 
    // entero de mayor bits a uno de menor y ver la perdida de información

    let value: i32 = 1500;
    let result: i8 = value as i8;

    println!("\n====== AS IS UNSAFE [CONVERSIONS] ======");
    println!("Value (i32) = {}", value);
    println!("Result (i8) = {}", result);
}


pub fn into_number_conversion() {

    // Para generar conversiones seguras se puede usar el trait into
    // Esto se puede hacer siempre y cuando el compilador pueda determinar
    // el tipo de dato objetivo o de destino

    let number_1: i64 = 28;
    let number_2: i32 = 32;

    println!("\n====== INTO CONVERSION [CONVERSIONS] ======");
    if number_1 < (number_2.into()) {
        print!("number_1 was converted from i32 to i64")
    }

    // En caso de que la conversion con into falle porque no es segura
    // la ejecucion del programa falla

    // Para esto se utiliza try_into().unwarp(), si la conversión es
    // exitosa unwrap devuelve el valor convertido, pero si falla genera
    // un error con retorno de éxito. Esto lo hace generando pánico

    // El pánico es la forma controlada que tiene rust para falla en tiempo
    // de ejecución. El pánico interrumpe la ejecución normal del programa
}