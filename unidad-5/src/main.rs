// Para usar el modulo definido en otro archivo es necesario declararlo
// La implementación se lleva a cabo en el otro archivo
mod calculator_2;

// Para usar la calculadora con modulo anidados
mod nested_calculator;

// Se puede empezar definiendo un modulo en el mismo archivo que se va a usar
// Por ejemplo se puede definir el modulo calculator_1 usando la palabra
// reservada mod y añadiendo pub a las funciones para que sean públicas

mod calculator_1 {

    // Funcion definida dentro del módulo pero visible para los demás
    pub fn add(a: i32, b:i32) -> i32 {
        a + b
    }
}

fn main() {

    // Se puede usar la función add definida en el modulo calculator_1
    println!("5 + 6 [CALCULATOR_1] = {}", calculator_1::add(5, 6));

    // Tambíen su puede usar la fucnión add definida en modulo externo
    println!("5 + 6 [CALCULATOR_2] = {}", calculator_2::add(5, 6));

    // Accediendo a los modulos anidados dentro del modulo calculadora
    println!("5 + 6 [NESTED_CALCULATOR] = {}",  nested_calculator::basic::add(5, 6));
    println!("5 / 6 [NESTED_CALCULATOR] = {}",  nested_calculator::advance::divide(5.0, 6.0));
}
