use super::*;  // Import z lib.rs

#[derive(Debug, Clone)]
pub struct ExampleModel {
    choices: Vec<String>,
    cursor: usize,
    selected: Vec<bool>,
}

impl ExampleModel {
    pub fn new() -> Self {
        Self {
            choices: vec![
                "Kup marchewki".to_string(),
                "Kup seler".to_string(),
                "Kup kohlrabi".to_string(),
            ],
            cursor: 0,
            selected: vec![false; 3],
        }
    }
}

impl LumaModel for ExampleModel {
    fn init(&mut self) -> Cmd {
        Cmd::Tick(Duration::from_millis(100))  // Start timera (np. do animacji)
    }

    fn update(&mut self, msg: Msg) -> Cmd {
        match msg {
            Msg::Key(key) => match key.code {
                KeyCode::Char('q') | KeyCode::Esc => Cmd::Quit,
                KeyCode::Up | KeyCode::Char('k') => {
                    if self.cursor > 0 {
                        self.cursor -= 1;
                    }
                    Cmd::None
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if self.cursor < self.choices.len() - 1 {
                        self.cursor += 1;
                    }
                    Cmd::None
                }
                KeyCode::Enter | KeyCode::Char(' ') => {
                    self.selected[self.cursor].flip();
                    Cmd::None
                }
                _ => Cmd::None,
            },
            Msg::Tick => Cmd::Tick(Duration::from_millis(100)),  // Kontynuuj timer
            Msg::Quit => Cmd::Quit,
        }
    }

    fn view(&self) -> String {
        let mut s = "Co kupimy na rynku?\n\n".to_string();

        for (i, choice) in self.choices.iter().enumerate() {
            let cursor = if self.cursor == i { ">" } else { " " };
            let checked = if self.selected[i] { "x" } else { " " };
            s.push_str(&format!("{} [{}] {}\n", cursor, checked, choice));
        }

        s.push_str("\nNaciśnij q aby wyjść.\n");
        s
    }
}
