use luma::{Cmd, KeyEvent, LumaModel, Msg, Program};
use std::io;

struct SimpleModel {
    counter: i32,
    input: String, // Simulate Nula command input
}

impl SimpleModel {
    fn new() -> Self {
        SimpleModel {
            counter: 0,
            input: String::new(),
        }
    }
}

impl LumaModel for SimpleModel {
    fn init(&mut self) -> Cmd {
        // Start a timer for periodic updates (e.g., to simulate Nula processing)
        Cmd::Tick(std::time::Duration::from_millis(100))
    }

    fn update(&mut self, msg: Msg) -> Cmd {
        match msg {
            Msg::Key(KeyEvent { code, .. }) => match code {
                crossterm::event::KeyCode::Char('q') => Cmd::Quit,
                crossterm::event::KeyCode::Up => {
                    self.counter += 1;
                    Cmd::None
                }
                crossterm::event::KeyCode::Down => {
                    self.counter -= 1;
                    Cmd::None
                }
                crossterm::event::KeyCode::Char(c) => {
                    self.input.push(c); // Simulate entering a Nula command
                    Cmd::None
                }
                crossterm::event::KeyCode::Enter => {
                    // Simulate processing a Nula command (clear input for now)
                    self.input.clear();
                    Cmd::None
                }
                _ => Cmd::None,
            },
            Msg::Tick => {
                // Periodic update (e.g., could process Nula state here)
                Cmd::Tick(std::time::Duration::from_millis(100))
            }
            Msg::Quit => Cmd::Quit,
        }
    }

    fn view(&self) -> String {
        format!(
            "Nula CLI\nCounter: {}\nInput: {}\nPress Up/Down to change counter, type to enter Nula commands, Enter to submit, 'q' to quit.",
            self.counter, self.input
        )
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let model = SimpleModel::new();
    let mut program = Program::new(model);
    program.run()?;
    Ok(())
}
