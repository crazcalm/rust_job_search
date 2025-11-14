use crate::models::Company;
use crate::ui::create_company;
use crate::ui::{Application, Counter, CreateCompanyUI};

pub mod db;
pub mod models;
pub mod ui;

const DB_PATH: &str = "/home/crazcalm/.config/rust-job-search/rust_job_search.sqlite3";

fn main() {
    /*
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
}
