use std::fmt;

use iced::event::{self, Event};
use iced::keyboard::{self, key};
use iced::widget::{self, button, column, container, pick_list, row, text, text_input};
use iced::{Center, Element, Subscription, Task};

use crate::db;
use crate::models::JobPosting;
use crate::DB_PATH;
use url::Url;

#[derive(Default, Debug)]
pub struct CreateJobPostingUI {
    url: String,
    date_applied: Option<String>, // TODO: figure out setting dates
    description: String,
    interviewed: bool,
    company: CompanyDropDown,
    contact: ContactDropDown,
}

#[derive(Debug, Clone)]
pub enum Message {
    Url(String),
    DateApplied(String),
    Description(String),
    Interviewed(bool),
    Company(CompanyDropDown),
    ContactId(i64),
    Save,
    Cancel,
    Event(Event),
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum CompanyDropDown {
    #[default]
    None,
    Some((i64, String)),
}

impl fmt::Display for CompanyDropDown {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompanyDropDown::None => write!(f, "None"),
            CompanyDropDown::Some((_, name)) => write!(f, "{name}"),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum ContactDropDown {
    #[default]
    None,
    Some((i64, String)),
}

impl CreateJobPostingUI {
    pub fn reset(&mut self) {
        self.url = String::new();
        self.date_applied = None;
        self.description = String::new();
        self.interviewed = false;
        self.company = CompanyDropDown::default();
        self.contact = ContactDropDown::default();
    }
}

impl CreateJobPostingUI {
    pub fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::Event)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        Task::none()
    }

    pub fn view(&self) -> Element<Message> {
        container(column![
            text("Add a new Job Posting"),
            row![
                text("URL:"),
                text_input("Add URL", &self.url).on_input(Message::Url)
            ],
            row![text("Date Applied")],
            row![text("Description:")],
            row![text("Interviewed")],
            row![
                text("Company"),
                pick_list(
                    [
                        CompanyDropDown::None,
                        CompanyDropDown::Some((1, "Test_name".to_string())),
                        CompanyDropDown::Some((2, "test 2".to_string()))
                    ],
                    Some(self.company.clone()),
                    Message::Company
                )
            ],
            row![text("Contact")],
        ])
        .into()
    }
}
