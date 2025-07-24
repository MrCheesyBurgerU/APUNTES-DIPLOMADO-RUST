use std::io::{self, Read};
use std::fs::File;

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

