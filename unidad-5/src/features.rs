// Las características (features) son partes opcionales del código 
// que se pueden activar o desactivar desde el archivo Cargo.toml.
//
// Sirven para:
// - Activar dependencias opcionales.
// - Incluir partes específicas del código de forma condicional.
// - Cambiar el comportamiento del crate según lo que se necesite.
// - Hacer la compilación más ligera al incluir solo lo necesario.

// Se definen en Cargo.toml bajo la sección [features]:

/*

[package]
name = "foo"

[features]
derive = ["syn"]  // 'derive' es una característica que depende de 'syn'

[dependencies]
syn = { version = "1", optional = true }  // 'syn' solo se usa si 'derive' está activado

*/

// En este ejemplo, el crate 'syn' no se compila por defecto.
// Solo se compila si otro proyecto activa la característica 'derive'.

/*

# Proyecto que depende de 'foo'

[package]
name = "bar"

[dependencies]
foo = { version = "1", features = ["derive"] }

*/

// En ese caso, 'bar' activa la feature 'derive' del crate 'foo',
// y por eso se compila también la dependencia opcional 'syn'.

// También es posible excluir o ignorar ciertas partes del código
// usando compilación condicional con el atributo `cfg`.

// `cfg` es una macro que permite incluir o excluir código
// dependiendo de las características activadas.

// Ejemplo de código condicional con #[cfg(feature = "...")]

#[cfg(feature = "derive")]
fn imprimir_aviso() {
    println!("La característica 'derive' está activada.");
}

#[cfg(not(feature = "derive"))]
fn imprimir_aviso() {
    println!("La característica 'derive' NO está activada.");
}

fn main() {
    // Llamamos la función que depende de si 'derive' está activada o no
    imprimir_aviso();
}
