use crate::models::Company;
use crate::ui::create_company;
use crate::ui::{Counter, CreateCompanyUI};

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

    iced::application("Example", CreateCompanyUI::update, CreateCompanyUI::view)
        .subscription(CreateCompanyUI::subscription)
        .run()
        .unwrap();

    println!("Hello -- {test_company:?}");
}
