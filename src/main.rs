#[macro_use] extern crate rocket;

use rocket::{ Config };

mod models; use self::models::user::{ User::* };

/**
 * @title ---- START SERVICE ----
 * @brief Función de incio del servidor
 * @param _ parmetro de tipado débil
 */

#[launch]
fn rocket() -> _ {
    // datos del servicio web
    let config = Config::figment()
        .merge(("address", "127.0.0.1"))
        .merge(("port", 8081));

    // levantando el servicio web: rocket::[build()|custom()]
    rocket::custom(config)
        .mount("/users", routes![getUser, setUser])
}