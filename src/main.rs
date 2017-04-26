#![feature(plugin)]
#![plugin(rocket_codegen)]


extern crate rocket;
extern crate debt_tools;

use debt_tools::add;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<num1>/<num2>")]
fn adder(num1: i32, num2: i32) -> String{
    (add(num1, num2)).to_string()
}

fn main() {
    rocket::ignite()
    	.mount("/", routes![index])
    	.mount("/adder", routes![adder])
    	.launch();
}