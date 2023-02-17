use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Corner, Direction, Layout, Alignment},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
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

/// This struct holds the current state of the app. In particular, it has the `items` field which is a wrapper
/// around `ListState`. Keeping track of the items state let us render the associated widget with its state
/// and have access to features such as natural scrolling.
///
/// Check the event handling at the bottom to see how to change the state on incoming events.
/// Check the drawing logic for items on how to specify the highlighting style for selected items.
struct App<'a> {
    items: StatefulList<(&'a str, usize)>,
}

impl<'a> App<'a> {
    fn create(number_of_items: usize) -> App<'a> {
        let mut items: Vec<(&'a str, usize)> = Vec::new();
        for n in 0..number_of_items {
            items.push(("", n));
        };//Vec<(&'a str, usize)>
        App {
            items: StatefulList::with_items(items),
        }
    }

}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    let tick_rate = Duration::from_millis(250);
    
    let list_items: Vec<ListItem<>> = vec![ListItem::new("balls"),ListItem::new(" ")];

    let app = App::create(list_items.len());
    let res = run_app(&mut terminal, app, tick_rate,list_items);

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
    list_items: Vec<ListItem<>>,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &mut app, list_items.clone()))?;

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
            last_tick = Instant::now()
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App, list_items: Vec<ListItem<>>) {
    // Create two chunks with equal horizontal screen space

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
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

        let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Length(3),
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

    let items = List::new(list_items)
        .block(Block::default().borders(Borders::ALL).title("List"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    // We can now render the item list
    let lower_right_pane =
            create_block("hiright");

    f.render_stateful_widget(items, bottom_chunks[0], &mut app.items.state);
    f.render_widget(lower_right_pane, bottom_chunks[1]);
    
    let paragraph = Paragraph::new("Quiz Me!")
    .style(Style::default())
    .block(create_block(""))
    .alignment(Alignment::Center);

    f.render_widget(paragraph, chunks[0])

}