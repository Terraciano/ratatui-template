pub struct App {
    pub app_title: String,
    pub current_screen: CurrentScreen,
    pub current_input: String,
}

pub enum CurrentScreen {
    Main,
}

impl App {
    pub fn new() -> App {
        App {
            app_title: String::from("My new app"),
            current_screen: CurrentScreen::Main,
            current_input: String::new(),
        }
    }

    pub fn on_char_press(&mut self, value: char) {
        self.current_input.clear();
        self.current_input.push(value);
    }
}
