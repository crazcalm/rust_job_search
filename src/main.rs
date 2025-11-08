use crate::models::Company;

pub mod models;

fn main() {
    let test_company = Company {
        id: None,
        name: Some("name".to_string()),
        address: None,
        website: None,
        phone: None,
    };

    println!("Hello -- {test_company:?}");
}
