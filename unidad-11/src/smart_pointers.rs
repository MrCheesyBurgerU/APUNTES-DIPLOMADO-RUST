// Un puntero es una variable que guarda la dirección de memoria de un dato
// Un puntero inteligente controla la memoria automáticamente (drop), agrega control
// de referencias, mutabilidad y evita fugas de memoria y problemas de 
// puntero colgante

// En otras palabras, un puntero inteligente es una estructura de datos que 
// actua como un puntero pero cuenta con metadatos y funcionalidades adiconales

// El primer tipo de puntero inteligente que se puede usar es Box<T>, el cual 
// sirve para apuntar a datos en el heap. Permite almacenar datos en el heap
// en lugar de almacenarlos en el stack. En el stack solo vive el puntero que
// apunta a los datos en el heap. En otras palabras, guarda la dirección de 
// memoria de un dato. Se usan cuando el tamaño del tipo no se puede conocer en
// tiempo de compilación y cuando se tiene una gran cantidad de datos y se desea
// transferir la propiedad

// Por ejemplo, podemos forzar que un entero que por defecto se crea en el stack 
// viva en el heap haciendo uso del puntero recien mencionado

use std::rc::Rc;

fn int_to_heap() {
    let number: Box<i32> = Box::new(42);
    println!("The number in the heap is: {}", number);
}

// Otro tipo de puntero inteligente es Rc<T>. Hay casos en los que un mismo valor 
// puede tener multiples propietarios, se debe habilitar la propiedad multiple
// explicitamente mediante este tipo de apuntador. Es una abreviatura para el termino
// conteo de referencias. Regisra el numero de referencias a un valor determinado si
// este sigue en uso, si no hay ninguna referencia el valor se puede limpiar de forma
// segura. Se usa cuando se asignan datos en el monton para que varias partes del 
// programa los lean o los usen, o en esecenarios de un solo subproceso

fn rc_use() {
    let data: Rc<String> = Rc::new(String::from("Bank Account"));
    // Aca se puede ver como la variable tiene multiples dueños
    let customer1: Rc<String> = Rc::clone(&data);
    let customer2: Rc<String> = Rc::clone(&data);

    println!("Customer share: {}", customer1);
    // Encontramos tres referencias, la original y las dos de los clientes
    println!("References counter: {}", Rc::strong_count(&data));
}

// Otro tipo es RefCell<T>. El patron de mutabilidad de rust permite modificar datos 
// a traves de una referencia inmutable. Esto usualmente esta prohibido, pero se logra
// encapsulando codigo inseguro en una api segura, como la que ofrece este apuntador. 
// Es un patron de diseño que permite modificar datos aun con una referencia inmutable,
// se utiliza codigo inseguro que le indica al compilador que la verificacion de reglas
// se haga en tiepo de ejecución y no en compilacion

use std::cell::RefCell;

#[derive(Debug)]
struct account {
    balance: f64
}

fn ref_cell_use() {
    let account = Rc::new(RefCell::new(account { balance : 1000.0}));
    // Aca se puede ver como la variable tiene multiples dueños
    let customer1  = Rc::clone(&account);
    let customer2  = Rc::clone(&account);

    customer1.borrow_mut().balance += 500.0;
    customer2.borrow_mut().balance -= 200.0;

    println!("Final balance: {:?}", account.borrow().balance);
}

// Se pueden consultar otro tipo de apuntadores inteligentes en la documentación 
// de Rust, como Arc