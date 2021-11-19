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
    text_input
};

const WINDOW_DIMEN: (u32, u32) = (1024, 768); // width, height

pub fn main() -> iced::Result {
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
enum BarMessage {

}

#[derive(Debug, Clone)]
enum RendererMessage {

}

#[derive(Debug, Clone)]
enum Message {
    BarMessage(BarMessage),
    RendererMessage(RendererMessage)
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

    fn update(&mut self, _message: Self::Message, _clipboard: &mut Clipboard) -> Command<Self::Message> {
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
struct Bar;

impl Bar {
    fn new() -> Bar {
        Bar
    }

    fn view(&mut self) -> Element<Message> {
        Text::new("Hello from Bar")
            .into()
    }
}

#[derive(Debug, Clone)]
struct Renderer;

impl Renderer {
    fn new() -> Renderer {
        Renderer
    }

    fn view(&mut self) -> Element<Message> {
        Text::new("Hello from Renderer")
            .into()
    }
}
