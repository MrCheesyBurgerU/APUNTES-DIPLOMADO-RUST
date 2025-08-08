// Los tipos genéricos permiten reutilizar funciones con distintos
// tipos de datos con carácteristicas compartidas, evitando errores
// y duplicación de código cuya lógica es la misma y solo cambia el 
// tipo de dato. Por ejemplo, una función que genérica que calcula 
// el mayor número dentro de un vector

// Se utilizan referencias porque no se conoce el tamaño del tipo de
// forma exacta, de igual forma se utilizan referencias para no mover
// la propiedad, solo acceder a ella


// std::cmp::PartialOrd restringe el tipo de dato T a aquellos que
// implementen el trait de orden parcial
pub fn greatest_number<T: std::cmp::PartialOrd>(list: &[T]) -> &T {

    // Siempre que se use el tipo de dato generico se hace por  medio
    // de referencia, el valor se asigna por referencia 
    let mut greatest: &T = &list[0];

    for number in list {
        // La comparación depende del tipo de dato, entonces toca
        // especificar el trait que debe implementar el tipo de dato
        // para que el código tenga sentido y consistencia
        if number > greatest {
            greatest = number;
        }
    }
    greatest
}


// Los parámetros de tipo genérico se declaran dentro de <>, usualmente
// se utilizan letras mayúsculas. Se pude, por ejemplo declarar una 
// estructura usando tipos genericos, esto permite declarar un punto donde
// ambas coordenadas son del mismo tipo


pub struct Point<T> {
    x: T,
    y: T
}

// Se pueden realizar implementaciones sobre estructuras genéricas
// Se deben utilizar referencias en el self, por el mismo motivo presentado
// antes, en este caso la funcion recupera la coordenada x

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}


// También se puede crear una enumeración utilizando tipos genéricos.
// Esta enum se parece a Option<T> y Result<T, E>: puede contener un valor válido (Ok)
// o representar la ausencia de valor (None).


pub enum GenericEnum<T> {
    Ok(T),
    None
}

pub fn try_generic_enum() {

    // Valor válido: un número entero
    let value1: GenericEnum<i32> = GenericEnum::Ok(42);

    // Valor válido: texto
    let value2: GenericEnum<&'static str> = GenericEnum::Ok("hello");

    // Valor no válido: None
    let value3: GenericEnum<i32> = GenericEnum::None;

    // Imprime el contenido de cada valor usando match
    match value1 {
        GenericEnum::Ok(v) => println!("Value 1: {}", v),
        GenericEnum::None => println!("Value 1: None"),
    }

    match value2 {
        GenericEnum::Ok(v) => println!("Value 2: {}", v),
        GenericEnum::None => println!("Value 2: None"),
    }

    match value3 {
        GenericEnum::Ok(v) => println!("Value 3: {}", v),
        GenericEnum::None => println!("Value 3: None"),
    }
}


// Tambien se puede utilizar mas de un tipo generico, por ejemplo en una
// estructura coordenada donde cada punto de la coordenada es de un tipo
// generico


pub struct Coordinate<T, U> {
    pub x: T,
    pub y: U,
}

pub fn try_coordinate() {

    // Coordenada con dos números enteros
    let point1 = Coordinate { x: 10, y: 20 };

    // Coordenada con un entero y un número decimal
    let point2 = Coordinate { x: 5, y: 3.5 };

    // Coordenada con texto (ejemplo no numérico)
    let point3 = Coordinate { x: "latitude", y: "longitude" };

    println!("Point 1: ({}, {})", point1.x, point1.y);
    println!("Point 2: ({}, {})", point2.x, point2.y);
    println!("Point 3: ({}, {})", point3.x, point3.y);
}




