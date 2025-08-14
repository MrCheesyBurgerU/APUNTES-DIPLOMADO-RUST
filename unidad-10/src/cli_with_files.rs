use std::fs;

use clap::Parser;

// Programa que lee un archivo, convierte su contenido a mayusculas y
// lo guarda en un nuevo archivo
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct  File {
    #[arg(short, long)]
    input: String,
    #[arg(short, long)]
    output: String
}

fn execute() {
    // Se crea la estructura para el manejo y lectura de los argumentos
    let args: File = File::parse();
    
    // Esta parte se puede mejorar usando un result para manejo de errores
    // Al ser un programa sencillo solo es para demostracion
    let content: String = fs::read_to_string(&args.input).unwrap();

    // Se convierte el contenido a mayusculas
    let new_content: String = content.to_uppercase();

    // Se escribe el contenido en la ruta de salida, lo mismo, se puede
    // mejorar el manejo de errores
    fs::write(&args.output, new_content).unwrap();
}

