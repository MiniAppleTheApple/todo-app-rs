use iced::widget::{button, column, text, text_input, row};
use iced::{Alignment, Element, Sandbox, Settings, Color};
use iced::window;

pub fn main() -> iced::Result {
    App::run(Settings {
        window: window::Settings {
            size: (500, 500),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Debug, Clone)]
struct Task {
    name: String,
    is_done: bool,
}

struct App {
    tasks: Vec<Task>,
    input_value: String,
}

#[derive(Debug, Clone)]
enum Message {
    RemoveTask(usize),
    DoneTask(usize),
    UnDoneTask(usize),
    InputChanged(String),
    CreateTask,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self {
            tasks: Vec::new(),
            input_value: String::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Todo - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::RemoveTask(index) => {
                self.tasks.remove(index);
            },
            Message::DoneTask(index) => {
                match self.tasks.get_mut(index) {
                    Some(ref mut task) => {
                        task.is_done = true;
                    },
                    None => {},
                }
            },
            Message::UnDoneTask(index) => {
                match self.tasks.get_mut(index) {
                    Some(ref mut task) => {
                        task.is_done = false;
                    },
                    None => {},
                }
            },
            Message::InputChanged(value) => {
                self.input_value = value
            },
            Message::CreateTask => {
                self.tasks.push(Task {name: self.input_value.clone(), is_done: false});
                self.input_value.clear()
            },
        }
    }

    fn view(&self) -> Element<Message> {
        let tasks: Element<_> = column(
            self.tasks.
                iter().
                enumerate().
                map(|(i, task)| {
                    let color = if task.is_done {
                        Color::from([0.5, 0.5, 0.5])
                    } else {
                        Color::from([0.0, 0.0, 0.0])
                    };
                    row![
                        text(&task.name).size(30).style(color),
                        button("Done").on_press(Message::DoneTask(i)),
                        button("Remove").on_press(Message::RemoveTask(i)),
                    ]
                    .spacing(20)
                    .align_items(Alignment::Center)
                    .into()
                }).
                collect::<Vec<_>>()).
            spacing(20).
            into();
        column![
            text("Todo").size(70),
            text_input(
                    "What needs to be done?",
                    &self.input_value, 
                    Message::InputChanged,
                )
                .padding(15)
                .size(30)
                .on_submit(Message::CreateTask),
            button("Add task").on_press(Message::CreateTask),
            tasks, 
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}