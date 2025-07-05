#[macro_use]
extern crate rocket;
use rocket::form::{Form, FromForm};
use rocket::response::content::RawHtml;
use std::fs;
use std::io::{self, Write};

#[derive(FromForm)]
struct Data {
    content: String,
}


#[post("/message", data = "<form_data>")]
fn answer(form_data: Form<Data>) -> RawHtml<String> {
    let content = form_data.into_inner();
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("data.txt")
        .expect("Failed to open file");
    writeln!(file, "{}", content.content).expect("Failed to write to file");
    RawHtml("<p>Message received.</p>".to_string())
}

#[get("/")]
fn page() -> RawHtml<&'static str> {
    RawHtml(r#"
    <form action="/message" method="post">
        <label for="label">Post something </label>
        <input type="text" id="content" name="content" required>

        <input type="submit" value="Submit">
    </form>
    "#)
}

#[get("/message")]
fn get_message() -> RawHtml<String> {
    match fs::read_to_string("data.txt") {
        Ok(content) => RawHtml(format!(r#"<p>{}</p>"#, content)),
        Err(_) => RawHtml("<p>No message found.</p>".to_string()),
    }
}


#[launch]
pub fn rocket() -> _ {
    rocket::build().mount("/", routes![page, answer, get_message])
}
