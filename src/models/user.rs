/**
 * ----------------------------------------------------------
 * ESQUEMA DE USUARIOS
 * ----------------------------------------------------------
 * @brief Esquema del dato User que se espera manjerar
 * @trait id identificador
 * @trait username Nombre del usuario
 */

#[allow(dead_code)]
struct UserSchema {
    id: u128,
    username: String,
}

/**
 * ----------------------------------------------------------
 * DEFINICIONES DE LOS HANDLERS
 * ----------------------------------------------------------
 * @brief Conjunto de funciones correspondientes a User
 */

#[allow(non_snake_case)]
pub mod User {
    #[get("/user/<name>")]
    #[allow(non_snake_case)]
    pub fn getUser(name: String) -> String {
        format!("{}", name).to_string()
    }

    #[get("/create")]
    #[allow(non_snake_case)]
    pub fn setUser() -> &'static str {
        "set User!"
    }
}