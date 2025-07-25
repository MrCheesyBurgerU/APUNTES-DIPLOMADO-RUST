use std::io::{self, ErrorKind, Read};
use std::fs::{File, Metadata};

// Intenta abrir un archivo y leer su contenido sin usar el operador ?
pub fn read_file_manual(path: &str) -> Result<String, io::Error> {
    // Intentar abrir el archivo
    let mut file = match File::open(path) {
        Ok(f) => f,
        // Error al abrir: se retorna
        Err(e) => return Err(e), 
    };

    let mut contents = String::new();

    // Intentar leer el contenido
    match file.read_to_string(&mut contents) {
        // Lectura exitosa
        Ok(_) => Ok(contents),   
        // Error al leer: se retorna
        Err(e) => Err(e),        
        // Tambien se puede hacer que cuando haya un error se genere panico
        // Pero al generar panico se corta la ejecución del programa
    }
}


// Intenta abrir un archivo y devolver su contenido como String
// Esta función devuelve un Result, lo que permite manejar errores de forma controlada.
pub fn read_file_contents(path: &str) -> Result<String, io::Error> {
    // Si falla, el error se propaga automáticamente
    let mut file: File = File::open(path)?; 
    let mut contents = String::new();
    // También se puede propagar con ?
    file.read_to_string(&mut contents)?; 
    Ok(contents)
}


// Uso explícito del match para manejar el resultado de forma detallada
// Los results retornados se pueden imprimir usando modo debug
// Para eso se debe retornar el match, osea dejarlo sin ; 
pub fn print_file(path: &str) {
    match read_file_contents(path) {
        Ok(text) => {
            println!("Contenido del archivo:\n{}", text);
        }
        Err(e) => {
            // Error recuperable: el archivo podría no existir o no tener permisos
            println!("No se pudo leer el archivo: {}", e);
        }
    }
}


// Otra función sencilla que retorna Result manualmente
// En este ejemplo se retorna el match, y entonces se puede imprimir con .?
pub fn divide(a: f64, b: f64) -> Result<f64, &'static str> {
    if b == 0.0 {
        // Error recuperable: división por cero
        Err("No se puede dividir por cero")
    } else {
        Ok(a / b)
    }
}


// Intenta abrir el archivo "saludo.txt". Si no existe, lo crea.
// Si ocurre otro tipo de error, se produce un panic.
// Finalmente, devuelve los metadatos del archivo.
pub fn open_or_create_and_get_metadata() -> Metadata {
    let file_result = File::open("saludo.txt");

    let file = match file_result {
        // Archivo abierto correctamente
        Ok(f) => f, 
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("saludo.txt") {
                // Archivo no existía, se crea exitosamente
                Ok(f) => f, 
                // Error creando el archivo
                Err(e) => panic!("Error creating file: {}", e), 
            },
            // Otro tipo de error, se puede agregar mejor manejo
            _ => panic!("Error opening file: {}", e), 
        },
    };

    // Intenta obtener los metadatos del archivo
    // unwrap puede causar panic si falla
    file.metadata().unwrap() 
}


// Busca el primer número par en una lista y lo devuelve, o None si no hay
pub fn find_first_even(numbers: &[i32]) -> Option<i32> {
    // Itera sobre la lista y encuentra el primer número divisible por 2
    for &num in numbers {
        if num % 2 == 0 {
            // Valor válido encontrado
            return Some(num); 
        }
    }
    // No se encontró ningún número par
    None 
}


// Define una enumeración personalizada para representar distintos tipos de errores
#[derive(Debug)]
enum MyError {
    FileNotFound(String),
    FileCorrupted(String),
}


// Esta función intenta abrir un archivo y obtener su tamaño.
// Si ocurre un error, devuelve un `MyError` específico.
fn handle_file() -> Result<u64, MyError> {
    // Intenta abrir el archivo "Noexisto.txt"
    let file = File::open("Noexisto.txt");

    // Maneja el resultado del intento de abrir el archivo
    let mut file = match file {
        Ok(file) => file,
        Err(_) => return Err(MyError::FileNotFound("The file does not exist".to_string())),
    };

    // Retorna el tamaño del archivo si todo salió bien
    Ok(file.metadata().unwrap().len())
}

