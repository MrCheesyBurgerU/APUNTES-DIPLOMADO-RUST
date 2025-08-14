use std::env;

fn manual_parse() {
    // Collect genera una colección con todos los argumentos 
    // que se reciben por medio del CLI
    let args: Vec<String> = env::args().collect();

    // Si la longitud es 1 no hay comandos, por ejemplo cargo run
    // Los comandos tambien se pueden pasar desde target/debug/unidad-10.exe
    if args.len() == 1 {
        println!("Please, provide an argument");
    }
    else {
        let arg_1: &String = &args[1];
        println!("The argument received is: {}", arg_1);
    }
}

use clap::{Parser};

// Se puede usar clap para simplificar la tarea de la línea de comandos
// La configuración de clap se presenta en el cargo
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args{
    name: String, 
    age: String
}

// Mucho mas facil y mantenible para codigo real
fn clap() {
    let args = Args::parse();
    println!("Name: {}", args.name);
    println!("Age: {}", args.age);
}

// Clap permite utilizar la anotación #[arg(short, long)] para
// saber como acepta el nombre de los argumentos, por ejemplo si
// se especifican las dos para un argumento input pues ponerse como
// input o como i