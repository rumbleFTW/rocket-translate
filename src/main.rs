#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]

struct Response {
    params: String,
    response: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Ok {
    message: &'static str,
}

#[get("/")]
fn root() -> Json<Ok> {
    let response: Ok = Ok {
        message: "Hello, World!",
    };
    Json(response)
}

#[get("/add/<num1>/<num2>")]
fn add(num1: &str, num2: &str) -> Json<Response> {
    let n1: f64 = num1.parse().unwrap();
    let n2: f64 = num1.parse().unwrap();
    let response = Response {
        params: format!("{} {}", &num1, &num2),
        response: format!("{}", n1 + n2),
    };
    Json(response)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![root, add])
}
