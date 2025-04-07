use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Terminal,
};
use std::{io, time::Duration};
use std::collections::HashSet;

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let raw_menu_items = vec![
        "Home", "Items", "City", "Job", "Gym", "Properties", "Education",
        "Crimes", "Missions", "Newspaper", "Jail", "Hospital", "Casino",
        "Forums", "Hall of Fame", "Faction", "Recruit Citizens", "Calendar", "Rules",
    ];

    // Example statuses
    let unread: HashSet<&str> = ["Newspaper", "Crimes", "Messages"].into_iter().collect();
    let important: HashSet<&str> = ["Hospital", "Jail", "Crimes"].into_iter().collect();

    let menu_items: Vec<(&str, Color)> = raw_menu_items
        .iter()
        .map(|label| {
            let color = if important.contains(label) {
                Color::Red
            } else if unread.contains(label) {
                Color::Green
            } else {
                Color::Gray
            };
            (*label, color)
        })
        .collect();

    let mut selected = 0;
    let mut state = ListState::default();
    state.select(Some(selected));

    let mut input = String::new();

    loop {
        terminal.draw(|f| {
            let area = f.area();

            // Main layout: nav | main
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Length(20), Constraint::Min(0)])
                .split(area);

            // Split main content into: content + input
            let right_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(1), Constraint::Length(3)])
                .split(chunks[1]);

            // Colored menu
            let menu: Vec<ListItem> = menu_items
                .iter()
                .map(|(label, color)| {
                    ListItem::new((*label).to_string())
                        .style(Style::default().fg(*color))
                })
                .collect();

            let list = List::new(menu)
                .block(Block::default().title("Menu").borders(Borders::ALL))
                .highlight_style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol("> ");

            f.render_stateful_widget(list, chunks[0], &mut state);

            // Content box
            let content = match menu_items[selected].0 {
                "Home" => {
                    "╔═══ Home Tutorial ═══════════════════════════════════════════════════════════════════════════════════════════════════════════════════╗\n\
                     ║ This is your home, you can get here at any time by clicking it on the navigation panel. It shows most of your details, for instance ║\n\
                     ║ your attacking stats and the property you are living in. If you ever need information about yourself, here's where to come.         ║\n\
                     ╚═════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════╝"
                },
                "Items" => "You have no items yet.",
                "City" => "Explore the city here.",
                "Job" => "Apply for a job or check your tasks.",
                "Gym" => "Train to improve your stats.",
                "Properties" => "View or manage your properties.",
                "Education" => "Enroll in a course to gain knowledge.",
                "Crimes" => "Commit crimes and earn rewards.",
                "Missions" => "Complete missions to progress.",
                "Newspaper" => "Read the latest news.",
                "Jail" => "You're not in jail. Yet.",
                "Hospital" => "Check your health status.",
                "Casino" => "Try your luck in the casino.",
                "Forums" => "Engage with the community.",
                "Hall of Fame" => "Top players are listed here.",
                "Faction" => "Manage or join a faction.",
                "Recruit Citizens" => "Invite others to your city.",
                "Calendar" => "See upcoming events.",
                "Rules" => "Follow the rules!",
                _ => "Unknown page.",
            };

            let paragraph = Paragraph::new(content)
                .block(Block::default().title("Content").borders(Borders::ALL));
            f.render_widget(paragraph, right_chunks[0]);

            // Input box
            let input_box = Paragraph::new(input.as_str())
                .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                .block(Block::default().title("Input").borders(Borders::ALL));
            f.render_widget(input_box, right_chunks[1]);
        })?;

        // Input events
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) => {
                        input.push(c);
                    }
                    KeyCode::Backspace => {
                        input.pop();
                    }
                    KeyCode::Enter => {
                        if menu_items[selected].0 == "Exit" {
                            break;
                        }
                        input.clear();
                    }
                    KeyCode::Esc => break,
                    KeyCode::Up => {
                        if selected > 0 {
                            selected -= 1;
                            state.select(Some(selected));
                        }
                    }
                    KeyCode::Down => {
                        if selected < menu_items.len() - 1 {
                            selected += 1;
                            state.select(Some(selected));
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
