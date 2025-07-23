// Una colección es una estructura de datos permite almacenar 
// múltiples valores en una sola variable. Puede cambiar de 
// tamaño en tiempo de ejecución, a diferencia por ejemplo de los
// arreglos

// Los vectores solo admiten un tipo de dato homogéneo, y los HashMap
// funcionan por llave valor

// Los vectores son una lista dinámica de elementos del mismo tipo

pub fn declare_and_add() {
    // Declara un vector de número
    let mut numbers: Vec<i32> = vec![1 ,2, 3];
    // Función para añadir a la última posicion
    numbers.push(4);
}


pub fn declare_empty() {
    // Creamos un vector vacío de cadenas de texto
    let mut fruits: Vec<&'static str> = Vec::new();

    // Agregamos elementos al vector
    fruits.push("Apple");
    fruits.push("Pear");

    // Iteramos sobre las referencias a los elementos del vector
    for fruit in &fruits {
        println!("Fruit {}", fruit);
    }

    // Imprimimos el total de frutas usando el método .len()
    println!("Total fruits = {}", fruits.len());
}


pub fn use_pop() {
    // Declaramos un vector mutable de cadenas
    let mut fruits: Vec<&'static str> = vec!["Apple", "Banana", "Pear"];

    // Mostramos el contenido original
    println!("Original fruits: {:?}", fruits);

    // Eliminamos el último elemento usando pop()
    let last: Option<&'static str> = fruits.pop();

    // Verificamos si se extrajo algo
    match last {
        Some(fruit) => println!("Popped fruit: {}", fruit),
        None => println!("No fruit to pop"),
    }

    // Mostramos el vector después del pop
    println!("Remaining fruits: {:?}", fruits);
}


pub fn ownership_and_borrowing() {
    // Creamos un vector que posee cadenas dinámicas (String)
    let mut fruits: Vec<String> = Vec::new();

    // Agregamos elementos usando String::from para que el vector tenga la propiedad
    fruits.push(String::from("Apple"));
    fruits.push(String::from("Banana"));

    // --- Préstamo inmutable ---
    // Obtenemos una referencia inmutable al primer elemento
    let first: &String = &fruits[0];
    // Aca no puedo usar el vector, porque por el momento no posee la propiedad de la 
    // primera fruta, hay que esperar que se libere
    println!("First fruit (borrowed immutably): {}", first);

    // --- Préstamo mutable ---
    // Obtenemos una referencia mutable al segundo elemento y lo modificamos
    let second: &mut String = &mut fruits[1];
    second.push_str(" 🍌");
    println!("Modified second fruit (borrowed mutably): {}", second);

    // --- Transferencia de propiedad ---
    // Quitamos un elemento del vector y lo movemos a otra variable
    let last: String = fruits.pop().expect("No hay fruta para extraer");
    println!("Popped fruit (ownership moved): {}", last);

    // Mostramos el estado final del vector
    println!("Final fruits: {:?}", fruits);
}