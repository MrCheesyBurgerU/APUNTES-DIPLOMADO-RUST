// Hay dos etiquetas principales de crear tests en rust, usando la instruccion
// #[cfg(test)] para que el compilador ejecute la prueba, y anotar las funciones
// con el atributo #[test]. La primera se hace a nivel de modulo, la segunda a nivel
// de funcion

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 6.0 + 2.0;
        assert_eq!(result, 8.0);
    }
}


// La macro assert esta proporcionada por la biblioteca estandar y grantiza que la
// condicion de una prueba evalua como verdadera, si da falso se genera panico
// y la prueba falla

// Tenemos distintos tipos de asserts, se presentan algunos el resto se pueden
// verificar en documentacion
// assert!(true)
// assert_eq!(1,1)
// assert_ne!(1,2)

// Las pruebas se pueden crear de las siguientes formas

// En un proyecto library/crate (lib) es lo recomendado para cubrir la funcionalidad en la librería.
// En un proyecto binary (main) las pruebas van en el folder `tests`.
// Se ejecutan todas las pruebas con la instrucción:
// cargo test

// Se ejecuta una prueba específica indicando el nombre:
// cargo test nombre_prueba

// Para ejecutar pruebas en paralelo o de forma consecutiva:
// cargo test -- --test-threads=1
