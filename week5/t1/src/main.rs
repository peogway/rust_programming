#[macro_use] extern crate rocket;

use rocket::form::Form;
use rocket::response::content::RawHtml;

#[derive(FromForm)]
struct Feedback {
    good: Option<String>,
    bad: Option<String>,
}

#[get("/")]
fn page() -> RawHtml<&'static str> {
    RawHtml(r#"
    <h1>How are you?</h1>
    <form action="/answer" method="post">
        <label for="good">Good</label>
        <input type="checkbox" id="good" name="good" value="true">

        <label for="bad">Bad</label>
        <input type="checkbox" id="bad" name="bad" value="true">

        <input type="submit" value="Submit">
    </form>
    "#)
}

#[post("/answer", data = "<form_data>")]
fn answer(form_data: Form<Feedback>) -> RawHtml<String> {
    let form = form_data.into_inner();
    let good = form.good.as_deref().unwrap_or("").is_empty() == false;
    let bad = form.bad.as_deref().unwrap_or("").is_empty() == false;

    let msg = if good && bad {
        "Can you really be having both a good and a bad day at the same time?"
    } else if good {
        "Hey, I am glad to hear that. Keep on rockin'! :)"
    } else if bad {
        "I'm sorry to hear that. I hope things get better for you. :("
    } else {
        "You did not share your feelings. :("
    };

    RawHtml(format!(r#"<div id="response"><h2>{}</h2></div>"#, msg))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![page, answer])
}
