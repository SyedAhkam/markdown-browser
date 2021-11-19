use iced::{
    Application,
    Settings,
    Command,
    Clipboard,
    Element,
    executor,
    Container,
    Column,
    Row,
    Text,
    Scrollable,
    TextInput,
    Button,
    Length,
    text_input,
    button,
    scrollable
};

use md_browser_protocol::{Response, Markdown};
use crate::make_request;

const WINDOW_DIMEN: (u32, u32) = (1024, 768); // width, height

pub fn start() -> iced::Result {
    App::run(Settings {
        window: iced::window::Settings {
            size:WINDOW_DIMEN, 
            ..Default::default()
        },
        ..Default::default()
    })
}

#[derive(Debug)]
struct App {
    bar: Bar,
    renderer: Renderer
}


#[derive(Debug, Clone)]
enum Message {
    UrlInputChanged(String),
    SubmitButtonPressed
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (App, Command<Self::Message>) {
        (App {
            bar: Bar::new(),
            renderer: Renderer::new()
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Markdown Browser 📗")
    }

    fn update(&mut self, message: Self::Message, _clipboard: &mut Clipboard) -> Command<Self::Message> {
            match message {
                Message::UrlInputChanged(value) => {
                    self.bar.url_text = value;
                },
                Message::SubmitButtonPressed => {
                    self.renderer.fetch_and_render_page(self.bar.url_text.clone());
                }
            };

        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        let App { bar, renderer } = self;

        let main_column = Column::new()
            .push(bar.view())
            .push(renderer.view());

        Container::new(main_column)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

#[derive(Debug, Clone)]
struct Bar {
    url_input: text_input::State,
    url_text: String,
    submit_button: button::State
}

impl Bar {
    fn new() -> Bar {
        Bar { 
            url_input: text_input::State::new(),
            url_text: String::new(),
            submit_button: button::State::new()
        }
    }

    fn view(&mut self) -> Element<Message> {
        let Bar { 
            url_input,
            url_text,
            submit_button
        } = self;

        let row = Row::new()
            .push(TextInput::new(
                url_input,
                "Enter URL",
                url_text,
                Message::UrlInputChanged
            ))
            .push(Button::new(
                submit_button,
                Text::new("GO")
            ).on_press(Message::SubmitButtonPressed));

        Container::new(row)
            .into()
    }
}

#[derive(Debug, Clone)]
struct Renderer {
    scrollable: scrollable::State,
    current_response: Option<Response>
}

impl Renderer {
    fn new() -> Renderer {
        Renderer { scrollable: scrollable::State::new(), current_response: None }
    }

    fn view(&mut self) -> Element<Message> {
        let Renderer { scrollable, .. } = self;

        Scrollable::new(scrollable)
            .push(Text::new(
                    match &self.current_response {
                        Some(r) => r.md.display(),
                        None => String::from("")
                    }
                ))
            .into()
    }

    fn fetch_and_render_page(&mut self, url: String) {
        let response = make_request(&url).unwrap();

        self.current_response = Some(response);
    }
}
