// Ayudan a garantizar que las referencias a los datos sean validas
// mientras se necesitan, la mayoria de las veces son implicitos o 
// inferidos

// Su principal objetivo es evitar la creación de referencias colgantes, 
// ya que esto genera errores en el programa

// Manejamos referencias para evitar la toma de propiedad
// Rust no sabe que referencia va a ser retornada, por lo cual no sabe
// que referencia mantener viva para retornar, por eso debemos explicar
// cuando deben durar las variables para que se puedan retornar

// &i32 es una referencia sencilla
// &'a i32 es una referencia con lifetime explicito
// &'a mut i32 es una referencia mutable con lifetimme explicito

// Con la notación queremos que la referencia que se retorna sea valida
// mientras que ambos parametros de entrada lo sean, en otras palabras que 
// x y y vivan lo suficiente para que puedan ser retornados


pub fn longest_string<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }
    else {
        y
    }
}

// Esto hace que rust no permita errores de memoria, como usar un valor que ya 
// se borro, si la referencia tiene un valor, ese valor debe seguir existiendo