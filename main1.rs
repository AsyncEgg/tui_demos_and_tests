use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io, time::{Duration, Instant}};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs, Paragraph, List, ListItem, ListState},
    Frame, Terminal,
};

struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

struct App<'a> {
    items: StatefulList<(&'a str, usize)>,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            items: StatefulList::with_items(vec![
                ("Item0", 1),
                ("Item1", 2),
                ("Item2", 1),
                ("Item3", 3),
                ("Item4", 1),
                ("Item5", 4),
                ("Item6", 1),
                ("Item7", 3),
                ("Item8", 1),
                ("Item9", 6),
                ("Item10", 1),
                ("Item11", 3),
                ("Item12", 1),
                ("Item13", 2),
                ("Item14", 1),
                ("Item15", 1),
                ("Item16", 4),
                ("Item17", 1),
                ("Item18", 5),
                ("Item19", 4),
                ("Item20", 1),
                ("Item21", 2),
                ("Item22", 1),
                ("Item23", 3),
                ("Item24", 1),
            ]),
        }
    }

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(250);
    let app = App::new();
    let res = run_app(&mut terminal, app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Left => app.items.unselect(),
                    KeyCode::Down => app.items.next(),
                    KeyCode::Up => app.items.previous(),
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>) {
    let create_block = |title| {
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title(Span::styled(
                title,
                Style::default().add_modifier(Modifier::BOLD),
            ))
    };

    let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints(
                [
                    Constraint::Percentage(12),
                    Constraint::Percentage(88),
                ]
                .as_ref(),
            )
            .split(f.size());
        let bottom_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints(
                [
                    Constraint::Percentage(30),
                    Constraint::Percentage(50),
                ]
                .as_ref(),
            )
            .split(chunks[1]);
        
        let items: Vec<ListItem> = vec![ListItem::new("poo"),ListItem::new("poo"),ListItem::new("poo"),ListItem::new("poo")];

        let lower_left_pane = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("List"))
                .highlight_style(
                    Style::default()
                        .bg(Color::LightGreen)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");

        

        f.render_widget(lower_left_pane, bottom_chunks[0]);
        f.render_widget(lower_right_pane, bottom_chunks[1]);
        
        let paragraph = Paragraph::new("Quiz Me!")
        .style(Style::default())
        .block(create_block(""))
        .alignment(Alignment::Center);

        f.render_widget(paragraph, chunks[0])

    
}
