// En este caso se define el modulo como un archivo externo

// Igual se puede usar la función add en otros archivos y evita
// la confusión de espacios de nombrado. Puedo tener funciones que
// se llamen igual en distintos espacios de nombre. 
pub fn add(a: i32, b:i32) -> i32 {
    a + b
}

// IMPORTANTE
// Cuando un modulo se define en un archivo a parte, y ese archivo
// está dentro de una carpeta, se debe crear un archivo mod.rs en la
// carpeta y definir el modulo asociado al archivo

// Por ejemplo, si se tiene la carpeta calculadora y adentro el archivo
// calculadora.rs (con submodulos basico y avanzado), se debe crear el
// archivo mod.rs dentro de la carpeta y definir mod calculadora
