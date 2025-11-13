use iced::event::{self, Event};
use iced::keyboard::{self, key};
use iced::widget::{self, button, column, container, row, text, text_input};
use iced::{Center, Element, Subscription, Task};

use crate::ui::counter;
use crate::ui::create_company;

#[derive(Debug)]
pub struct Application {
    screen: Screen,
}

impl Default for Application {
    fn default() -> Self {
        Application {
            screen: Screen::Counter(counter::Counter::default()),
        }
    }
}

#[derive(Debug)]
enum Screen {
    Counter(counter::Counter),
    CreateCompany(create_company::CreateCompanyUI),
}

#[derive(Debug)]
pub enum Message {
    Counter(counter::Message),
    CreateCompany(create_company::Message),
    Event(Event),
}

impl Application {
    pub fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::Event)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Counter(message) => {
                if let Screen::Counter(counter_instance) = &mut self.screen {
                    match message {
                        counter::Message::ChangeScreen => {
                            self.screen =
                                Screen::CreateCompany(create_company::CreateCompanyUI::default());
                            Task::none()
                        }
                        _ => {
                            let _ = counter_instance.update(message);
                            Task::none()
                        }
                    }
                } else {
                    Task::none()
                }
            }
            Message::CreateCompany(message) => {
                if let Screen::CreateCompany(create_company_instance) = &mut self.screen {
                    match message {
                        _ => {
                            let _ = create_company_instance.update(message);
                            Task::none()
                        }
                    }
                } else {
                    Task::none()
                }
            }
            Message::Event(event) => {
                /*
                       if let Screen::CreateCompany(create_company_instance) = &mut self.screen {
                           create_company_instance.update(create_company::Message::Event(event));
                           Task::none()
                       } else {
                           Task::none()
                   }
                */

                match event {
                    Event::Keyboard(keyboard::Event::KeyPressed {
                        key: keyboard::Key::Named(key::Named::Tab),
                        modifiers,
                        ..
                    }) => {
                        println!("Tab was pushed from APPLICATION APP");

                        if modifiers.shift() {
                            widget::focus_previous()
                        } else {
                            widget::focus_next()
                        }
                    }
                    _ => Task::none(),
                }
            }
            _ => Task::none(),
        }
    }
    pub fn view(&self) -> Element<Message> {
        match &self.screen {
            Screen::Counter(counter) => counter.view().map(Message::Counter),
            Screen::CreateCompany(create_company) => {
                create_company.view().map(Message::CreateCompany)
            }
        }
    }
}
