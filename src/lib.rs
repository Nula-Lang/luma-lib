//! Luma - Framework do budowania TUI w Rust, inspirowany Bubble Tea.
//! Użyj traitu `LumaModel` dla swojego modelu stanu.

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Stdout};
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::{Duration, Instant};

/// Wiadomość (Msg) - zdarzenia w aplikacji.
#[derive(Debug, Clone)]
pub enum Msg {
    Key(KeyEvent),  // Naciśnięcie klawisza
    Tick,           // Timer tick
    Quit,           // Sygnał wyjścia
}

/// Komenda (Cmd) - asynchroniczne akcje (np. timer).
#[derive(Debug, Clone)]
pub enum Cmd {
    None,
    Tick(Duration),  // Uruchom timer po czasie
    Quit,            // Wyjdź
}

/// Trait dla modelu aplikacji - implementuj to dla swojego stanu.
pub trait LumaModel: Sized {
    /// Inicjalizacja - zwróć początkową komendę (np. start timera).
    fn init(&mut self) -> Cmd {
        Cmd::None
    }

    /// Update - przetwórz wiadomość, zwróć (nowy model, komendę).
    fn update(&mut self, msg: Msg) -> Cmd;

    /// View - renderuj UI jako String.
    fn view(&self) -> String;
}

/// Kluczowe zdarzenie (jak tea.KeyMsg w Bubble Tea).
#[derive(Debug, Clone)]
pub struct KeyEvent {
    pub code: KeyCode,
    pub kind: KeyEventKind,
}

/// Program Luma - zarządza pętlą zdarzeń i renderingiem.
pub struct Program<M: LumaModel> {
    model: M,
    stdout: Stdout,
    rx: Option<Receiver<Msg>>,
    tx: Option<Sender<Msg>>,
    should_quit: bool,
    last_tick: Instant,
}

impl<M: LumaModel + 'static> Program<M> {
    /// Nowy program z początkowym modelem.
    pub fn new(mut model: M) -> Self {
        let mut stdout = io::stdout();

        // Włącz raw mode i alternate screen (jak w Bubble Tea).
        enable_raw_mode().unwrap();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();

        // Kanał dla komunikatów (asynchroniczne).
        let (tx, rx) = mpsc::channel();

        // Inicjalizuj model.
        let cmd = model.init();
        Self::dispatch_cmd(&tx, cmd);

        Self {
            model,
            stdout,
            rx: Some(rx),
            tx: Some(tx),
            should_quit: false,
            last_tick: Instant::now(),
        }
    }

    /// Uruchom program (pętla zdarzeń).
    pub fn run(mut self) -> io::Result<()> {
        loop {
            // Sprawdź komunikaty z kanału.
            if let Ok(msg) = self.rx.as_ref().unwrap().try_recv() {
                let cmd = self.model.update(msg.clone());
                if matches!(msg, Msg::Quit) || matches!(cmd, Cmd::Quit) {
                    self.should_quit = true;
                }
                Self::dispatch_cmd(self.tx.as_ref().unwrap(), cmd);
            }

            // Obsługa zdarzeń crossterm (klawisze, resize itp.).
            if event::poll(Duration::from_millis(16))? {  // ~60 FPS jak w Bubble Tea.
                if let Event::Key(key) = event::read()? {
                    let msg = Msg::Key(KeyEvent {
                        code: key.code,
                        kind: key.kind,
                    });
                    let cmd = self.model.update(msg);
                    Self::dispatch_cmd(self.tx.as_ref().unwrap(), cmd);
                }
            }

            // Timer tick.
            if self.last_tick.elapsed() >= Duration::from_millis(100) {
                self.model.update(Msg::Tick);
                self.last_tick = Instant::now();
            }

            // Renderuj widok.
            self.render()?;

            if self.should_quit {
                break;
            }
        }

        // Wyłącz tryby.
        disable_raw_mode()?;
        execute!(
            self.stdout,
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        self.stdout.flush()?;

        Ok(())
    }

    fn dispatch_cmd(tx: &Sender<Msg>, cmd: Cmd) {
        match cmd {
            Cmd::None => {}
            Cmd::Tick(duration) => {
                // Spawn wątek dla timera (asynchronicznie).
                let tx = tx.clone();
                std::thread::spawn(move || {
                    std::thread::sleep(duration);
                    let _ = tx.send(Msg::Tick);
                });
            }
            Cmd::Quit => {
                let _ = tx.send(Msg::Quit);
            }
        }
    }

    fn render(&mut self) -> io::Result<()> {
        // Czyszczenie ekranu.
        execute!(self.stdout, crossterm::cursor::MoveTo(0, 0))?;
        execute!(self.stdout, crossterm::terminal::Clear(crossterm::terminal::ClearType::All))?;

        // Renderuj widok modelu.
        let view = self.model.view();
        print!("{}", view);

        self.stdout.flush()?;
        Ok(())
    }
}
