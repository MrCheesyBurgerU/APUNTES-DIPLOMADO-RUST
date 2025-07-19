// Rust permite anidar modulos, para acceder a los módulos 
// anidados hay que usar la palabra reservada pub en distintos
// niveles, dependiendo del acceso que se desee

// Se puede defininr una calculadora que tenga un submodulo
// básico y uno avanzado 

pub mod basic {

    // Función que hace parte del modulo calculadora y del submodulo
    // basico
    pub fn add(a: i32, b:i32) -> i32 {
        a + b
    }
}

pub mod advance {

    // Funcion que hace parte del modulo calculadora y del submodulo
    // avanzado
    pub fn divide(a: f32, b: f32) -> f32 {
        a / b
    }
}

// Se pueden seguir anidando módulos desde que se configuren correctamente
// los accesos