// En Rust, un crate es la unidad mínima de compilación.
//
// Hay dos tipos de crates:
// - Crate binario: tiene una función main y produce un ejecutable.
// - Crate de librería: define funciones reutilizables y genera un archivo .rlib.

// Las librerías no tienen función main.
// El archivo de entrada es `lib.rs` (para librerías) o `main.rs` (para ejecutables).

// Puedes crear una librería nueva con:
// $ cargo new mi_libreria --lib

// Un paquete (package) es un proyecto completo que puede tener uno o más crates.
// Se define con un archivo `Cargo.toml` en la carpeta raíz del proyecto.

// Dentro de un crate de librería, puedes organizar el código en módulos.
// Para usar un módulo en otro archivo, se usa la palabra reservada `use`.

// A continuación, simulamos en un solo archivo cómo se usarían módulos
// y funciones como si estuviéramos en un proyecto real.

// ----------------------
// Simulando lib.rs
// ----------------------

// Definimos un módulo llamado `utilidades`
mod utilidades {
    // Una función pública dentro del módulo
    pub fn despedirse(nombre: &str) -> String {
        format!("Adiós, {}!", nombre)
    }
}

// Otra función pública fuera del módulo
pub fn saludar(nombre: &str) -> String {
    format!("Hola, {}!", nombre)
}

// ----------------------
// Simulando main.rs
// ----------------------

fn main() {
    // Importamos la función desde el módulo
    use utilidades::despedirse;

    let saludo = saludar("Carlos");
    println!("{}", saludo);

    let despedida = despedirse("Carlos");
    println!("{}", despedida);
}
