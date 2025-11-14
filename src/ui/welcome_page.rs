use iced::event::{self, Event};
use iced::keyboard::{self, key};
use iced::widget::{self, button, column, container, keyed_column, row, text, text_input};
use iced::{Center, Element, Fill, Subscription, Task};

#[derive(Debug, Default)]
pub struct WelcomePageUI {}

#[derive(Debug, Clone)]
pub enum Message {
    Companies,
    JobPostings,
    Contacts,
}

impl WelcomePageUI {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Companies => {
                println!("Companies was selected");
                Task::none()
            }
            Message::JobPostings => {
                println!("JobPostings was selected");
                Task::none()
            }
            Message::Contacts => {
                println!("Contacts was selected");
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        column![
            text("Welcome Page"),
            button("View Companies").on_press(Message::Companies),
            button("View Job Postings").on_press(Message::JobPostings),
            button("View Contacts").on_press(Message::Contacts),
        ]
        .into()
    }
}
