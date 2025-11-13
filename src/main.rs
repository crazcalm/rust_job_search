use crate::models::Company;
use crate::ui::create_company;
use crate::ui::{Application, Counter, CreateCompanyUI};

pub mod models;
pub mod ui;

fn main() {
    let test_company = Company {
        id: None,
        name: Some("name".to_string()),
        address: None,
        website: None,
        phone: None,
    };

    iced::application("Example", Application::update, Application::view)
        .subscription(Application::subscription)
        .run()
        .unwrap();

    println!("Hello -- {test_company:?}");
}
