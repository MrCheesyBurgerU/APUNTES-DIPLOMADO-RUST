use std::env;

fn read_env() {
    // Esta variable se trae del entorno del SO
    // Se debe configurar si no esta configurada
    let user_name: Result<String, env::VarError> = env::var("NOMBRE_USUARIO");

    match user_name {
        Ok(user) => println!("User name: {}", user),
        Err(e) => println!("NOMBE_USUARIO not defined: {}", e)
    }
}

