// Un workspace es un conjunto de paquetes (crates) que comparten el mismo Cargo.lock y 
// directorio de salida. Esto implica que se comparten dependencias, compilan todos juntos
// con un solo comando, separan la lógica en paquetes más pequeños pero relacionados 
// y facilitan las pruebas y mantenimiento

// La estructua básica de un workspace tiene el directorio raíz (el workspace), los archivos
// cargo y posteriormente n carpetas, donde cada carpeta es un crate, con su estructura para
// código fuente. Cada crate debe tener su Cargo.toml

// Para crear un workspace se debe editar el archivo Cargo.toml del directorio raíz. Se debe
// eliminar la sección [package]y cambiarla por [workspace] = members = ["crate_a", "crate_b"]

// Tambien se debe cargar el resolver, se usa para los workspaces para trabajar de forma 
// correcta la compatibilidad de las dependencias de los proyectos

// Dentro de las ventajas de usar workspaces se encuentra la compilación de librerias una sola
// vez para todo el proyecto, el uso de comandos globales como cargo build y la modularidad 
// de los proyectos

// Para correr un solo crate en especifico se puede hacer uso del comando cargo run -p crate

// Para revisar un proyecto que hace uso de workspaces se puede revisar el material presente
// en el repositorio de ejercicios, en la sección de clases