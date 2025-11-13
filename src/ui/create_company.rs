use iced::event::{self, Event};
use iced::keyboard::{self, key};
use iced::widget::{self, button, column, container, row, text, text_input};
use iced::{Center, Element, Subscription, Task};

#[derive(Default, Debug)]
pub struct CreateCompanyUI {
    name: String,
    address: String,
    website: String,
    phone: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Name(String),
    Address(String),
    Website(String),
    Phone(String),
    Save,
    Cancel,
    Event(Event),
}

impl CreateCompanyUI {
    pub fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::Event)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Name(data) => {
                self.name = data;
                Task::<Message>::none()
            }
            Message::Address(data) => {
                self.address = data;
                Task::none()
            }
            Message::Website(data) => {
                self.website = data;
                Task::none()
            }
            Message::Phone(data) => {
                self.phone = data;
                Task::none()
            }
            Message::Save => {
                println!("Save was pushed");
                println!("{self:?}");
                Task::none()
            }
            Message::Cancel => {
                println!("Cancel was pushed");
                Task::none()
            }
            Message::Event(event) => match event {
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key: keyboard::Key::Named(key::Named::Tab),
                    modifiers,
                    ..
                }) => {
                    println!("Tab was pushed");

                    if modifiers.shift() {
                        widget::focus_previous()
                    } else {
                        widget::focus_next()
                    }
                }
                _ => Task::none(),
            },
            _ => Task::none(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        container(
            column![
                text("Create a new Company"),
                row![
                    text("Name:"),
                    text_input("Change the name", &self.name).on_input(Message::Name)
                ]
                .spacing(10),
                row![
                    text("Address:"),
                    text_input("Address", &self.address).on_input(Message::Address)
                ]
                .spacing(10),
                row![
                    text("Website:"),
                    text_input("Website", &self.website).on_input(Message::Website)
                ]
                .spacing(10),
                row![
                    text("Phone:"),
                    text_input("Phone", &self.phone).on_input(Message::Phone),
                ]
                .spacing(10),
                row![
                    button("Cancel").on_press(Message::Cancel),
                    button("Save").on_press(Message::Save),
                ]
                .spacing(10)
            ]
            .padding(20)
            .spacing(10)
            .align_x(Center),
        )
        .into()
    }
}

/*
let test_company = Company {
        id: None,
        name: Some("name".to_string()),
        address: None,
        website: None,
        phone: None,
    };
*/
