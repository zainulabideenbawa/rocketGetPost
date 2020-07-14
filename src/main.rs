#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket::request::Form;
use rocket::response::content::Html;

#[derive(FromForm)]
struct Number {
    number: i32,
}

#[get("/hello/<number>")]
fn hello(number: i32) -> String {
    format!("the number you passed is {}", number)
}

#[get("/value")]
fn value() -> Html<&'static str> {
    Html(
        r#"
    <form style='margin: 100px 0px; margin-bottom: 80px;' action='/num' method='post'>
    <input style='padding: 5px 10px; border-radius: 3px;  height: 40px;
    width: 500px; font-size: 24px; border: 1px solid #9e00c5;' type='number' placeholder='Enter number'
        name='number' id='number'>
    <input style='font-size: 24px; padding:5px 12px; 
    border: none; height: 40px;

    border-radius: 3px;  color: white; background-color: #9e00c5
    ;' type='Submit'>
</form>
    "#,
    )
}

#[post("/num", data = "<number>")]
fn req(number: Form<Number>) -> Html<String> {
    let s = format!(
        "<p style='font-size: 24px; color:#770494; font-weight: bolder;'>
   The number you entered is {} and the result is :
   {}</p>",
        number.number,
        number.number * 2
    );
    Html(s)
}

#[get("/")]
fn home() -> Html<&'static str> {
    Html(
        r#"
        <!DOCTYPE html>
        <html>

        <head>
            <title>Home</title>
        </head>
        <body>
        <div style="background-color:black;color:white;padding:20px;">
        <a href='/value' style='padding: 10px 20px; margin-top: 10px; 
        font-size: 24px; color: white; background-color: #9e00c5; text-decoration: none; border-radius: 5px;'>Get
            Started</a>
        </body>
        </html>
    "#,
    )
}

fn main() {
    rocket::ignite()
        .mount("/", routes![hello, home, value, req])
        .launch();
}
