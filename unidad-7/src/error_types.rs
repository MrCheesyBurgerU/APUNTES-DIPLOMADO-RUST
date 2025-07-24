use std::io::{self, ErrorKind};
use std::fs::File;

// Este ejemplo representa un error recuperable
// Se intenta abrir un archivo que puede no existir
// Si no se encuentra, el programa puede continuar de forma controlada
pub fn open_config_file() -> Result<File, io::Error> {
    let file: Result<File, io::Error> = File::open("config.txt");

    match file {
        Ok(f) => Ok(f),
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            // El archivo no existe: situación esperada que puede manejarse
            // Aquí podríamos crear el archivo o usar valores por defecto
            Err(io::Error::new(ErrorKind::NotFound, "Archivo no encontrado"))
        }
        Err(e) => {
            // Otro tipo de error al abrir el archivo, como permisos insuficientes
            Err(e)
        }
    }
}

// Este ejemplo ilustra un error irrecuperable
// Se fuerza un panic al acceder fuera de los límites del vector
// Este tipo de errores debe evitarse mediante diseño, no se manejan en tiempo de ejecución

pub fn trigger_unrecoverable_error() {
    let data: Vec<i32> = vec![10, 20, 30];
    // Esto provoca panic: acceso fuera de los límites
    let value: i32 = data[99]; 

    // Este código nunca se ejecutará
    println!("Valor: {}", value); 
}

// Otra forma de provocar un error irrecuperable es usando unwrap sin verificar
// Si el valor es None, unwrap terminará el programa abruptamente
pub fn unwrap_none_value() {
    let maybe_value: Option<i32> = None;
    // Provoca panic: se intentó acceder a un valor inexistente
    let value: i32 = maybe_value.unwrap(); 
    // No se ejecuta este fragmento de código 
    println!("Valor: {}", value); 
}