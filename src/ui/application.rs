use iced::event::{self, Event};
use iced::keyboard::{self, key};
use iced::widget::{self, button, column, container, row, text, text_input};
use iced::{Center, Element, Subscription, Task};

use crate::ui::create_company;
use crate::ui::view_companies;
use crate::ui::welcome_page;

#[derive(Debug)]
pub struct Application {
    screen: Screen,
}

impl Default for Application {
    fn default() -> Self {
        Application {
            screen: Screen::WelcomePage(welcome_page::WelcomePageUI::default()),
        }
    }
}

#[derive(Debug)]
enum Screen {
    WelcomePage(welcome_page::WelcomePageUI),
    CreateCompany(create_company::CreateCompanyUI),
    ViewCompanies(view_companies::ViewCompaniesUI),
}

#[derive(Debug)]
pub enum Message {
    WelcomePage(welcome_page::Message),
    CreateCompany(create_company::Message),
    ViewCompanies(view_companies::Message),
    Event(Event),
}

impl Application {
    pub fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::Event)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::WelcomePage(message) => {
                if let Screen::WelcomePage(welcome_page_instance) = &mut self.screen {
                    match message {
                        welcome_page::Message::Companies => {
                            self.screen =
                                Screen::ViewCompanies(view_companies::ViewCompaniesUI::default());
                            Task::none()
                        }
                        welcome_page::Message::JobPostings => {
                            println!("JobPostings was selected from main application");
                            Task::none()
                        }
                        welcome_page::Message::Contacts => {
                            println!("Contacts was selected from main application");
                            Task::none()
                        }
                    }
                } else {
                    Task::none()
                }
            }
            Message::ViewCompanies(message) => {
                if let Screen::ViewCompanies(view_companies_instance) = &mut self.screen {
                    // Right now, all messages get passed on, but that will change
                    // once we have buttons that point to different pages.
                    match message {
                        view_companies::Message::WelcomePage => {
                            self.screen =
                                Screen::WelcomePage(welcome_page::WelcomePageUI::default());
                            Task::none()
                        }

                        view_companies::Message::AddCompany => {
                            self.screen =
                                Screen::CreateCompany(create_company::CreateCompanyUI::default());
                            Task::none()
                        }

                        _ => {
                            let _ = view_companies_instance.update(message);
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
                        create_company::Message::Cancel => {
                            self.screen =
                                Screen::ViewCompanies(view_companies::ViewCompaniesUI::default());
                            Task::none()
                        }

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
            Screen::CreateCompany(create_company) => {
                create_company.view().map(Message::CreateCompany)
            }
            Screen::ViewCompanies(view_companies) => {
                view_companies.view().map(Message::ViewCompanies)
            }
            Screen::WelcomePage(welcome) => welcome.view().map(Message::WelcomePage),
        }
    }
}
