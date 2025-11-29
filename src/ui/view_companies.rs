use iced::event::{self, Event};
use iced::keyboard::{self, key};
use iced::widget::{self, button, column, container, keyed_column, row, text, text_input};
use iced::{Center, Element, Fill, Subscription, Task};

use crate::db;
use crate::models::Company;
use crate::DB_PATH;

#[derive(Debug)]
pub struct ViewCompaniesUI {
    rows: Vec<RowUI>,
}

impl Default for ViewCompaniesUI {
    fn default() -> Self {
        let conn = db::get_connection(db::ConnectionType::Path(DB_PATH.to_string()))
            .expect("unable to get database connection");

        let companies = Company::get_all(&conn).expect("Unable to get Companies from the DB");

        let mut rows = Vec::new();
        for (id, company) in companies.iter().enumerate() {
            rows.push(RowUI::new(id, company.clone())); // TODO: think about using a reference over cloning it.
        }
        Self { rows }
    }
}
#[derive(Debug, Clone)]
pub enum Message {
    RowMessage(usize, RowMessage),
    AddCompany,
    WelcomePage,
}

impl ViewCompaniesUI {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::RowMessage(i, message) => {
                if let Some(row) = self.rows.get_mut(i) {
                    row.update(message);
                }

                Task::none()
            }
            Message::AddCompany => {
                println!("AddCompany via ViewCompaniesUI::update");
                Task::none()
            }
            Message::WelcomePage => {
                println!("WelcomePage via ViewCompaniesUI::update");
                Task::none()
            }
            _ => Task::none(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let rows = keyed_column(self.rows.iter().enumerate().map(|(i, row)| {
            (
                row.id,
                row.view(i)
                    .map(move |message| Message::RowMessage(i, message)),
            )
        }));

        container(
            column![
                text("View Companies"),
                rows,
                row![
                    button("Welcome Page").on_press(Message::WelcomePage),
                    button("Add Company").on_press(Message::AddCompany),
                ]
            ]
            .spacing(10),
        )
        .width(Fill)
        .into()
    }
}

#[derive(Debug)]
pub struct RowUI {
    id: usize,
    company: Company,
}

#[derive(Debug, Clone)]
pub enum RowMessage {
    ClickedButton,
}

impl RowUI {
    pub fn new(id: usize, company: Company) -> Self {
        Self { id, company }
    }

    pub fn update(&mut self, message: RowMessage) -> Task<RowMessage> {
        match message {
            RowMessage::ClickedButton => {
                println!("Button was pushed for {self:?}");
                Task::none()
            }
            _ => Task::none(),
        }
    }

    pub fn view(&self, _i: usize) -> Element<RowMessage> {
        row![
            text(self.company.name.clone().unwrap_or_default()),
            text(self.company.website.as_ref().unwrap().to_string()),
            button("hit me").on_press(RowMessage::ClickedButton),
        ]
        .padding(10)
        .spacing(20)
        .width(Fill)
        .into()
    }
}
