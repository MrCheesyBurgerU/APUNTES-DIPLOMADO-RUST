
pub fn usage_of_mut() {

    // Al declarar una variable con let esta es inmutable por
    // defecto, si se intenta mutar se genera error

    // Para poder mutar una variable se utiliza la palabra 
    // reservada mut

    // Si se trata de constantes no hay forma de hacerlas
    // mutables

    println!("\n====== USAGE OF MUT [VARIABLES] ======");
    let mut mutable: i32 = 1500;
    println!("Original Value = {}", mutable);
    mutable = 200;
    println!("Changed Value = {}", mutable);
}


pub fn shadowing() {

    // El shadowing se aprovecha de los alcances de definicion
    // para aislar las operaciones sobre variables

    // En rust se pueden generar scopes usando corchetes

    let mut cont: i32 = 0;
    cont = cont + 2;

    // Dentro del nuevo alcance, la variable eclipsa a la definición 
    // original, es decir se apropia del uso hasta que se cierre el 
    // alcance

    println!("\n====== SHADOWING [VARIABLES] ======");

    {
        let mut cont: i32 = cont * 5;
        cont = cont + 1;

        println!("Cont Inner Scope = {}", cont);
    }

    // El alcance externo no se ve afectado por las otras operaciones
    println!("Cont Outter Scope = {}", cont);
}