use dotenv::dotenv;
use serde::Serialize;
use sparkpost::transmission::{
    Attachment, EmailAddress, Message, Options, Recipient, Transmission, TransmissionResponse,
};
use std::env;

use chrono::prelude::*;


#[derive(Debug, Serialize)]
struct Data {
    name: String,
    title: String
}


fn get_api_key() -> String {
    dotenv().ok();
    env::var("SPARK_API").expect("Spark Key Required")
}

fn main() {
    let tm = Transmission::new(get_api_key().to_owned());
    let mut email = Message::new(EmailAddress::new("register@mail.sachinmaharana.dev", "Sachin Maharan"));


    let options = Options {
        open_tracking: true,
        click_tracking: true,
        transactional: false,
        sandbox: false,
        inline_css: false,
        start_time: Some(Utc.ymd(2020, 4, 20).and_hms(0, 0, 0)),
    };

    let recipient = Recipient::with_substitution(
        EmailAddress::new("sachin.nicky@gmail.com", "Sachin Maharana"),
        Data {
            name: "Sachin".into(),
            title: "Maharana".into()
        },
    );

    let attachment = Attachment::from_data(
        "AnImage.png",
        "image/png",
        "iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAAAXNSR0IArs4c6QAAAAlwSFlzAAAWJQAAFiUBSVIk8AAAAXxJREFUOBFjvJVg84P5718WBjLAX2bmPyxMf/+xMDH8YyZDPwPDXwYGJkIaOXTNGdiUtHAqI2jA/18/GUQzGsg3gMfKg4FVQo6BiYcPqyF4XcChaczA4+DP8P//f4b/P3+SZgAzvxCDSGYjAyMjI8PvZw+AoYXdLuyiQLtE0uoZWAREwLb+fnKXQTipkngXcJu7MnACQx8G2FX1GHgs3bDGBlYX8HlFM/z9+JbhzewWhmf1CQyfti9j+PfzBwO/ZxTMTDiNmQKBfmZX1GB42V/K8P38YbDCX/dvMDAwMzPwuYbBNcIYmC4AhfjvXwx/376AqQHTf96+ZPj34xuKGIiDaQBQ8PPBTQwCoZkMjJzcYA3MgqIMAr7xDJ/3rAHzkQnGO7FWf5gZ/qLmBSZmBoHgNAZee1+Gf18/MzCyczJ83LyQ4fPetch6Gf4xMP3FbgBMGdAgJqAr/n37zABMTTBROA0ygAWUJUG5Civ4B8xwX78CpbD6FJiHmf4AAFicbTMTr5jAAAAAAElFTkSuQmCC");

    // complete the email message with details
    email
        .add_recipient(recipient)
        .add_attachment(attachment)
        .options(options)
        .campaign_id("Rust Rocks!")
        .subject("Nice 😎")
        .html("<h1>hello {{name}} {{title}}</h1>")
        .text("This is just a mail by {{name or 'this works'}}");

    let result = tm.send(&email);

    match result {
        Ok(res) => {
            println!("{:?}", &res);
            match res {
                TransmissionResponse::ApiResponse(api_res) => {
                    println!("API Response: \n {:#?}", api_res);
                    //   assert_eq!(1, api_res.total_accepted_recipients);
                    //   assert_eq!(0, api_res.total_rejected_recipients);
                }
                TransmissionResponse::ApiError(errors) => {
                    println!("Response Errors: \n {:#?}", &errors);
                }
            }
        }
        Err(error) => {
            println!("error \n {:#?}", error);
        }
    }
}

