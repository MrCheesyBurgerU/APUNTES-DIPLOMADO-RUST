// Metáfora del taller comunitario: Ownership, Borrowing y Lifetimes en Rust

// Imagina un taller comunitario con un banco de herramientas compartidas.
// Este taller tiene reglas estrictas para evitar accidentes, pérdidas o conflictos,
// al igual que Rust impone reglas para manejar la memoria de forma segura.

// ------------------------------------------------------------
// 1. Ownership (Propiedad)
// ------------------------------------------------------------
// Cuando tomas una herramienta del banco, te conviertes en su propietario temporal.
// Nadie más puede usar esa herramienta hasta que la devuelvas.
// En Rust, esto se conoce como "ownership": una variable es dueña de un valor,
// y mover ese valor a otra variable transfiere la propiedad.
//
// let martillo = String::from("Martillo"); // martillo es el dueño
// let nuevo_dueno = martillo;              // Se transfiere la propiedad a nuevo_dueno
// println!("{}", martillo); // ❌ Error: martillo ya no es el dueño

// ------------------------------------------------------------
// 2. Borrowing inmutable (Préstamo sin modificación)
// ------------------------------------------------------------
// A veces solo queremos inspeccionar una herramienta, no modificarla.
// Podemos pedirla prestada de forma inmutable: nadie la puede cambiar, solo mirar.
// En Rust, esto se hace con referencias inmutables: `&T`.
//
// fn inspeccionar(herramienta: &String) {
//     println!("Inspeccionando: {}", herramienta);
// }
//
// let herramienta = String::from("Sierra");
// inspeccionar(&herramienta); // Se presta sin perder ownership
// println!("{}", herramienta); // ✅ Todavía es el dueño

// ------------------------------------------------------------
// 3. Borrowing mutable (Préstamo con modificación)
// ------------------------------------------------------------
// Si queremos usar la herramienta para trabajar (ej. martillar),
// necesitamos un préstamo exclusivo: solo una persona puede modificarla a la vez.
// En Rust, esto es una referencia mutable: `&mut T`.
//
// fn usar(herramienta: &mut String) {
//     herramienta.push_str(" (en uso)");
// }
//
// let mut herramienta = String::from("Destornillador");
// usar(&mut herramienta); // Se presta de forma mutable
// println!("{}", herramienta); // ✅ Se puede seguir usando después

// ------------------------------------------------------------
// 4. Lifetimes (Tiempos de vida)
// ------------------------------------------------------------
// Las herramientas prestadas deben ser devueltas antes de que alguien más las use.
// En Rust, esto se garantiza mediante los "lifetimes": cada referencia tiene
// un tiempo de vida que el compilador verifica para evitar errores como
// acceder a herramientas que ya se devolvieron.
//
// fn usar_durante<'a>(herramienta: &'a String) {
//     // Solo podemos usar la herramienta mientras esté viva
//     println!("Usando durante su lifetime: {}", herramienta);
// }

// ------------------------------------------------------------
// 5. El Vigilante del Taller (El compilador de Rust)
// ------------------------------------------------------------
// En nuestro taller, hay un vigilante que se asegura de que:
// - Nadie use una herramienta ya devuelta (evita use-after-free)
// - Nadie modifique una herramienta mientras otros la miran (evita data races)
// - Todas las herramientas se devuelvan a tiempo (lifetimes correctos)
//
// Este vigilante es el compilador de Rust. Él verifica todas estas reglas
// en tiempo de compilación, sin necesidad de un recolector de basura.
//
// Gracias a esto, el código en Rust es rápido, seguro y libre de errores sutiles
// relacionados con la memoria.


pub fn ownership_transfer_example() {
    // Inicialmente la propiedad de "Drill" la tiene `tool`
    let tool: String = String::from("Drill");

    // La propiedad de `tool` se mueve al invocar la función que la consume
    use_tool(tool);

    // Este print daría error porque `tool` ya no es el dueño del valor
    // println!("Using the tool = {}", tool);

    // Podemos evitar la transferencia usando `clone()`
    // Esto crea un nuevo String con su propio espacio en el heap

    // use_tool(tool.clone());
    // println!("Using the tool [OWNERSHIP TRANSFER EXAMPLE] = {}", tool);
}


pub fn use_tool(tool: String) {
    // Muestra que la herramienta se está usando
    println!("Using the tool [USE_TOOL] = {}", tool);
}


pub fn mutable_borrow_example() {
    // Inicialmente `tool` es dueño del String "Wrench"
    let mut tool: String = String::from("Wrench");

    // Pasamos un préstamo mutable a la función que va a repararla
    repair_tool(&mut tool);

    // Como la propiedad nunca se transfirió, aún podemos usarla aquí
    println!("After repair [MUTABLE BORROW EXAMPLE] = {}", tool);
}


pub fn repair_tool(tool: &mut String) {
    // Simulamos una reparación agregando una nota a la herramienta
    tool.push_str(" (repaired)");

    // Mostramos el nuevo estado de la herramienta dentro de la función
    println!("Repairing tool [REPAIR_TOOL] = {}", tool);
}


pub fn immutable_borrow_example() {
    // Inicialmente `tool` es dueño del String "Hammer"
    let tool: String = String::from("Hammer");

    // Pasamos un préstamo inmutable a la función que va a inspeccionarla
    view_tool(&tool);

    // Como no se transfirió propiedad ni se modificó, podemos seguir usándola
    println!("After viewing [INMUTABLE BORROW EXAMPLE] = {}", tool);
}

pub fn view_tool(tool: &String) {
    // Inspeccionamos la herramienta sin modificarla
    println!("Viewing tool [VIEW_TOOL] = {}", tool);
}


pub fn multiple_mutable_borrows() {
    // Declaramos una herramienta como mutable
    let mut tool = String::from("Saw");

    // Intentamos prestarla de forma mutable a dos personas al mismo tiempo
    let person1 = &mut tool;
    let person2 = &mut tool; // ❌ Error: ya hay un préstamo mutable activo

    // Esto no compila: Rust no permite más de un préstamo mutable a la vez
    // println!("Person 1 is using [MULTIPLE MUTABLE BORROWS] = {}", person1);
    // println!("Person 2 is using [MULTIPLE MUTABLE BORROWS] = {}", person2);

    // Además, tampoco podemos usar la variable original `tool` aquí,
    // porque su propiedad está prestada de forma exclusiva a una de las variables.
    // println!("Trying to use tool directly [MULTIPLE MUTABLE BORROWS] = {}", tool);

    // Si se usa el préstamo y luego finaliza su scope,
    // podemos volver a usar `tool` sin problema. Ejemplo correcto:
    let person1 = &mut tool;
    println!("Person 1 is using [MULTIPLE MUTABLE BORROWS] = {}", person1); // Se usa y luego termina

    // Ahora el préstamo anterior terminó, y podemos pedir otro préstamo mutable
    let person2 = &mut tool;
    println!("Person 2 is using [MULTIPLE MUTABLE BORROWS] = {}", person2);

    // Finalmente, volvemos a tener acceso a tool directamente
    println!("Tool after both uses [MULTIPLE MUTABLE BORROWS] = {}", tool);

    // Esto demuestra que los préstamos mutables deben ser secuenciales,
    // no simultáneos, para respetar la seguridad en el acceso exclusivo.
}


pub fn multiple_immutable_borrows() {
    // Declaramos una herramienta (no necesitamos mut porque no la vamos a modificar)
    let tool = String::from("Chisel");

    // Realizamos dos préstamos inmutables simultáneos
    let person1 = &tool;
    let person2 = &tool;

    // Ambos pueden ver la herramienta sin problemas
    println!("Person 1 is viewing [MULTIPLE INMUTABLE BORROWS] = {}", person1);
    println!("Person 2 is viewing [MULTIPLE INMUTABLE BORROWS] = {}", person2);
}


pub fn dangling_reference_example() {

    // CASO 1: Referencia colgante (esto causaría error)
    //
    // let reference: &String;
    //
    // {
    //    let tool = String::from("Screwdriver");
    //    reference = &tool; // ❌ Error: `tool` no vive lo suficiente
    // }
    //
    // println!("Trying to use reference = {}", reference);

    // CASO 2: Solución usando clone (nueva copia con su propio espacio en el heap)
    let reference: String;

    {
        let tool = String::from("Screwdriver");
        // Clonamos la herramienta, creando una nueva copia que puede vivir fuera del scope
        reference = tool.clone();
    }

    println!("Using cloned tool [DANGLING REFERENCE EXAMPLE] = {}", reference);

    // CASO 3: Solución transfiriendo la propiedad directamente (sin &)
    let tool: String;

    {
        let temp = String::from("Hammer");
        // Transferimos la propiedad (ownership) a la variable `tool` del scope externo
        tool = temp;
        // Si aca quisiera imprimir la herramienta no funcionaria, ya no soy dueño 
    }

    println!("Using owned tool [DANGLING REFERENCE EXAMPLE] = {}", tool);

    // Ambos casos (clone y move) son válidos porque no se guarda una referencia
    // a una variable que ha sido destruida.
}


