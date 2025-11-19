//use std::path;

use crate::models::Company;
use crate::ui::create_company;
use crate::ui::{Application, Counter, CreateCompanyUI, ViewCompaniesUI, WelcomePageUI};

pub mod db;
pub mod models;
pub mod ui;

const DB_PATH: &str = "rust_job_search.sqlite3";

fn main() {
    /*
    let db_path = path::absolute(DB_PATH);
    println!("{db_path:?}");


    let conn = db::get_connection(db::ConnectionType::Path(DB_PATH.to_string())).unwrap();

    let mut test_company = Company {
        id: None,
        name: Some("name".to_string()),
        address: Some("address".to_string()),
        website: Some(url::Url::parse("https://fake_Website.com").unwrap()),
        phone: Some("1111111111".to_string()),
    };
    test_company.save(&conn);
    */
    iced::application("Example", Application::update, Application::view)
        .subscription(Application::subscription)
        .run()
        .unwrap();

    /*
    iced::application("Example", WelcomePageUI::update, WelcomePageUI::view)
        .run()
    .unwrap();
    */
}
