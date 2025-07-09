
#[derive(Debug)] // Necesario para imprimir la estructura con {:?}
struct Tool {
    // Permiten agrupar datos relacionados. Sirve para organizar,
    // validar y estandarizar la información

    // Usamos String para que la estructura sea dueña de los datos
    name: String,
    status: String,
}


impl Tool {
    // Constructor: crea una nueva herramienta con estado "new"
    pub fn new(name: &str) -> Self {
        Tool {
            name: String::from(name),
            status: String::from("new"),
        }
    }

    // Método para mostrar el estado actual de la herramienta
    pub fn show(&self) {
        println!("Tool: {}, Status: {}", self.name, self.status);
    }

    // Método para cambiar el estado de la herramienta (requiere &mut self)
    pub fn repair(&mut self) {
        self.status.push_str(" (repaired)");
    }
}


pub fn tool_usage_example() {
    // Creamos una nueva herramienta usando la función asociada ::new
    let mut tool = Tool::new("Drill");

    // Mostramos su estado actual usando un método
    tool.show();

    // Reparamos la herramienta (se modifica su estado interno)
    tool.repair();

    // Volvemos a mostrar el estado actualizado
    tool.show();

    // También podemos imprimir la estructura completa con {:?}
    println!("Full tool info [TOOL USAGE EXAMPLE] = {:?}", tool);
}


pub fn access_struct_fields_example() {
    // Creamos una herramienta
    let tool = Tool::new("Hammer");

    // Accedemos directamente a los atributos públicos de la instancia
    println!("Accessing fields [ACCESS STRUCT FIELDS] = name: {}, status: {}", tool.name, tool.status);

    // Podemos combinar este acceso con el método show si lo deseamos
    tool.show();
}


pub fn struct_update_syntax_example() {
    // .. permite crear nuevas instancias de una estructura reutilizando
    // los valores de otra instancia para los campos restantes
    // Es útil cuando se quiere cambiar uno o varios campos

    // Creamos una herramienta original
    let original_tool = Tool::new("Saw");

    // Creamos una nueva herramienta, cambiando solo el nombre
    let renamed_tool = Tool {
        name: String::from("Circular Saw"),
        ..original_tool
    };

    // Mostramos el estado de la nueva herramienta
    println!("Renamed tool [STRUCT UPDATE SYNTAX] = {:?}", renamed_tool);

    // ⚠️ No podemos usar original_tool aquí porque `name` (de tipo String) fue movido
    // println!("{:?}", original_tool); // Esto causaría error

    // Creamos otra herramienta nueva, cambiando solo el estado
    // Ahora usamos renamed_tool como base
    let repaired_tool = Tool {
        status: String::from("repaired"),
        ..renamed_tool
    };

    // Mostramos el estado de la herramienta con nuevo estado
    println!("Repaired tool [STRUCT UPDATE SYNTAX] = {:?}", repaired_tool);

    // renamed_tool ya no es válido aquí porque se movió su campo status
    // println!("{:?}", renamed_tool); //También causaría error
}
