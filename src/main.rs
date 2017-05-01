#![feature(plugin)]
#![plugin(rocket_codegen, serde_derive)]

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::JSON;

#[derive(Serialize, Deserialize, Debug)]
struct Thing {
    field1: String,
    field2: u16,
}

#[post("/mappings/<id>", format = "application/json", data = "<thing>")]
fn post_mapping(id: &str, thing: JSON<Thing>) -> String {
    "".into()
}

fn main() {
    let serialized = serde_json::to_string(&Thing{field1:"".into(), field2: 1}).unwrap();
    let deserialized: Thing = serde_json::from_str(&serialized).unwrap();
    println!("{}", serialized);
    println!("{:?}", deserialized);
    rocket::ignite().mount("/", routes![post_mapping]).launch();
}
