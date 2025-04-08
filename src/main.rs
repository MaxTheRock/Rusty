use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Terminal,
};
use std::{io, time::Duration};
use std::collections::HashSet;

fn get_page_info(page: &str) -> (&'static str, &'static str, &'static str) {
    match page {
        "Home" => (
            "Welcome to your home screen. Here you’ll find your basic stats and property info.",
            "Stats overview",
            "Current property info"
        ),
        "Items" => (
            "This is your inventory. All your collected items will be listed here.",
            "You have no items yet.",
            "Use or discard items here."
        ),
        "City" => (
            "Visit shops, explore zones, and interact with the city here.",
            "City zones overview",
            "Shops and NPCs"
        ),
        "Job" => (
            "Check your current job, salary, and available tasks.",
            "Job title and salary",
            "Current tasks"
        ),
        "Gym" => (
            "Train your stats here. Strength, speed, defense—you name it.",
            "Stat training panel",
            "Recent training log"
        ),
        "Properties" => (
            "Buy, sell, or upgrade your properties.",
            "Owned properties",
            "Market listings"
        ),
        "Education" => (
            "Enroll in courses to gain skills that unlock new opportunities.",
            "Current courses",
            "Completed courses"
        ),
        "Crimes" => (
            "Perform crimes to gain money and experience. Risk vs reward!",
            "Available crimes",
            "Crime success history"
        ),
        "Missions" => (
            "Complete missions for rewards and progression.",
            "Current missions",
            "Completed missions"
        ),
        "Newspaper" => (
            "Read updates, events, and changes in the game world.",
            "Today’s headlines",
            "Archived news"
        ),
        "Jail" => (
            "See your jail status and how to escape or wait it out.",
            "Time remaining",
            "Escape options"
        ),
        "Hospital" => (
            "Check your injuries and time to recover.",
            "Injury status",
            "Recovery tips"
        ),
        "Casino" => (
            "Try your luck with slots, blackjack, and roulette.",
            "Available games",
            "Last win history"
        ),
        "Forums" => (
            "Chat with other players or browse announcements.",
            "Recent threads",
            "Your replies"
        ),
        "Hall of Fame" => (
            "View top players ranked by wealth, strength, and more.",
            "Leaderboard",
            "Your rank"
        ),
        "Faction" => (
            "Manage or join a faction to collaborate with others.",
            "Faction info",
            "Member list"
        ),
        "Recruit Citizens" => (
            "Invite new players and earn rewards.",
            "Referral link",
            "Recruit rewards"
        ),
        "Calendar" => (
            "Track daily and weekly events.",
            "Today’s events",
            "Upcoming events"
        ),
        "Rules" => (
            "Review game rules and avoid punishment.",
            "Most broken rules",
            "Reporting system"
        ),
        _ => (
            "This page is under construction.",
            "Left Box",
            "Right Box"
        )
    }
}

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

            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Length(20), Constraint::Min(0)])
                .split(area);

            // Vertical: Info (5) | Main (flex) | Input (3)
            let right_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(5),    // Info box
                    Constraint::Min(0),       // Content area
                    Constraint::Length(3),    // Input box
                ])
                .split(chunks[1]);

            let content_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ])
                .split(right_chunks[1]);

            // Render menu
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

            // Dynamic page data
            let current_page = menu_items[selected].0;
            let (info_text, left_text, right_text) = get_page_info(current_page);

            // Top Info Box
            let info_paragraph = Paragraph::new(info_text)
                .wrap(Wrap { trim: true })
                .block(Block::default().title("Info").borders(Borders::ALL));
            f.render_widget(info_paragraph, right_chunks[0]);

            // Two side-by-side boxes
            let left_box = Paragraph::new(left_text)
                .block(Block::default().title("Left Box").borders(Borders::ALL));
            let right_box = Paragraph::new(right_text)
                .block(Block::default().title("Right Box").borders(Borders::ALL));
            f.render_widget(left_box, content_chunks[0]);
            f.render_widget(right_box, content_chunks[1]);

            // Bottom Input Box
            let input_box = Paragraph::new(input.as_str())
                .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                .block(Block::default().title("Input").borders(Borders::ALL));
            f.render_widget(input_box, right_chunks[2]);
        })?;

        // Input events
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) => input.push(c),
                    KeyCode::Backspace => {
                        input.pop();
                    }
                    KeyCode::Enter => input.clear(),
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
