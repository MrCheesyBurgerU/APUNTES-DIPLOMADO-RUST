// En Rust, los errores irrecuperables se representan comúnmente mediante panic
// Un panic indica que ha ocurrido una situación en tiempo de ejecución que el programa no puede manejar
// Estos errores suelen deberse a fallos de programación (bugs), como acceder fuera de los límites de un vector
// o hacer unwrap sobre un valor None sin verificar previamente

// Cuando ocurre un panic, Rust inicia un proceso llamado "unwinding" o desenrrollado de la pila de llamadas
// Durante el desenrrollado, se van liberando los recursos y ejecutando los destructores (`drop`) de cada función
// en orden inverso al que fueron llamadas, hasta que se alcanza el punto de entrada del hilo o del programa

// Este mecanismo permite limpiar correctamente la memoria antes de terminar el programa,
// pero también tiene un costo en rendimiento. Si se desea evitar el desenrrollado,
// se puede configurar el compilador para abortar inmediatamente con `panic = "abort"` en Cargo.toml

// El desenrrollado solo ocurre si el panic no es atrapado mediante mecanismos como `catch_unwind`
// Si no se captura, el programa terminará abruptamente después de liberar los recursos.

pub fn fail_hard() {
    // Provca un panic de forma explicita y termina el programa
    panic!("Se produjo un error irrecuperable");
}


pub fn access_out_of_bounds() {
    let data: Vec<i32> = vec![10, 20, 30];
    // Provoca panic: el índice 99 está fuera del rango válido
    let value: i32 = data[99];

    // Este código no se alcanza, el programa termina antes
    println!("Valor: {}", value);
}

