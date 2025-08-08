// En rust todo es un tipo, los traits definen la funcionaliad de un tipo
// en particular y se pueden compartir con otros tipos (es parecido a las
// interfaces, protocolos, clases abstractas o contratos)

// Los traits son muy similares a un contrato o compromiso, el contrato define
// una funcion de cierta forma y todos los otros programas que quieran usarlo
// deben usar el comportamiento implementado o implementarlo ellos mismos

// El comportamiento de un tipo consiste en los  métodos que podemos invocar en 
// ese tipo, los traits permiten agrupar las firmas de metodos para definir un 
// conjunto de comportamientos necesarios para lograr un proposito


use std::fmt::Debug;

#[derive(Debug)]
pub struct File;

// Se define la firma del metodo leer, la cual la deben cumplir todos los tipos
// que deseen implementar ese metodo
pub trait Read {
    fn read(self: &Self) -> Result<usize, String> {
        // Se puede hacer una implementación por defecto para el trait
        // Por ejemplo, que siempre devuleva Ok(1)
        Ok(1)
    }
}

// Se realiza la  implementación del trait para el tipo de dato específico, 
// en este caso file, al implementar se sobreescribe con el codigo que uno pone

impl Read for File {
    fn read(self: &Self) -> Result<usize, String> {
        // Aca debería ir la implementación de la función,
        // En este caso se simula exito para el result con valor 0
        Ok(0)
        // Si no quiero sobreescribir solo hago el impl for y nada mas
    }
}


// Los traits se pueden pasar como parametro a otras funciones. En este caso item
// puede ser cualquier tipo de dato que implemente el trait Read

pub fn use_trait(item: &impl Read) -> Result<usize, String> {
    // Se usa el trait sobre el tipo de dato que lo implementa
    let result = item.read();
    result
}

// Tambien se puede usar la sintaxis de limite de trait, donde se especifica
// el trait que debe implementar el tipo de dato generico, en vez de pasar el impl

pub fn use_trait_limit<T: Read>(item: &T) -> Result<usize, String> {
    // Se usa el trait sobre el tipo de dato que lo implementa
    let result = item.read();
    result
}


// Se pueden especificar mas de un trait, tanto como patametor como por sintaxid
// de limite

pub fn use_multiple_trait<T>(item: &(impl Read + Debug)) {
    // Esta es la firma de una función especificando que el tipo de dato
    // debe implementar tanto el trait read como el trait debug
}

pub fn use_multiple_trait_limit<T: Read + Debug>(item: &T) {
    // Lo mismo que la funion anterior pero usando notacion de limite
}


// Esta sintaxis se vuelve complicada debido a la longitud, entonces se puede
// hacer de la siguiente forma

pub fn use_traits<T>(item: &T) where T: Read + Debug {}


// Otra cosa que se puede hacer es hacer un retorno que implementa un trait
// Esta función retorna un tipo que implementa el trait Read

pub fn get_reader() -> impl Read {
    File
}


