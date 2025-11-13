use iced::widget::{button, column, text};
use iced::{Center, Element, Task};

/*
pub fn main() -> iced::Result {
    iced::run(Counter::update, Counter::view)
}
 */

#[derive(Default, Debug)]
pub struct Counter {
    value: i64,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
    ChangeScreen,
}

impl Counter {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Increment => {
                self.value += 1;
                Task::none()
            }
            Message::Decrement => {
                self.value -= 1;
                Task::none()
            }
            _ => Task::none(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        column![
            button("Increment").on_press(Message::Increment),
            text(self.value).size(50),
            button("Decrement").on_press(Message::Decrement),
            button("Change Screen").on_press(Message::ChangeScreen)
        ]
        .padding(20)
        .align_x(Center)
        .into()
    }
}
