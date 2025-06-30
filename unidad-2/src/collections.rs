use std::vec;

pub fn integer_vector() {

    // Para crear un vector se utiliza una macro, al igual que
    // para imprimir contenido por consola

    let vector: Vec<i32> = vec![1, 2, 3];

    // Los vectores se pueden acceder por indice

    let index: usize = 2;
    let _value: i32 = vector[index];

    println!("\n====== INTEGER VECTOR [COLLECTIONS]  ======");
    println!("Vector = {:?}", vector);
}