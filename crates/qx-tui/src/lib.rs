mod stateful_list;

use std::io::stdout;

use color_eyre::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use itertools::Itertools;
use qx_core::{banner, Configuration, Environment};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, Padding, Paragraph, Wrap},
    Frame, Terminal,
};

use stateful_list::StatefulList;

struct State {
    environments: StatefulList,
}

pub enum Choice<'a> {
    Boot(&'a Environment),
    Continue,
    Quit,
}

pub fn run_loop(configuration: &Configuration) -> Result<Choice> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let indexed_environments: Vec<_> = configuration
        .envs
        .iter()
        .sorted_by_key(|(k, _)| k.to_string())
        .collect();
    let indexed_environment_names: Vec<_> = indexed_environments
        .iter()
        .map(|(_, v)| format!("{v}"))
        .collect();

    let mut state = State {
        environments: StatefulList::new(indexed_environment_names),
    };

    state.environments.select_first_if_exists();

    let mut choice = Choice::Continue;
    while matches!(choice, Choice::Continue) {
        terminal.draw(|frame| ui(&indexed_environments, frame, &mut state))?;
        choice = handle_events(&indexed_environments, &mut state)?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    Ok(choice)
}

fn handle_events<'a>(
    environments: &[(&'a String, &'a Environment)],
    state: &mut State,
) -> Result<Choice<'a>> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Up => state.environments.select_previous(),
                    KeyCode::Down => state.environments.select_next(),
                    KeyCode::Enter => {
                        if let Some(selected) = state.environments.selected_index() {
                            // Validate environment!
                            let (_, environment) = environments[selected];
                            return Ok(Choice::Boot(environment));
                        }
                    }
                    KeyCode::Char('q') => return Ok(Choice::Quit),
                    _ => (),
                }
            }
        }
    }

    Ok(Choice::Continue)
}

fn ui<'a>(environments: &[(&'a String, &'a Environment)], frame: &mut Frame, state: &mut State) {
    let main_areas = Layout::new(
        Direction::Vertical,
        [Constraint::Min(0), Constraint::Length(1)],
    )
    .split(frame.size());

    let areas = Layout::new(
        Direction::Horizontal,
        [Constraint::Min(35), Constraint::Min(0)],
    )
    .split(main_areas[0]);

    let center_areas = Layout::new(
        Direction::Vertical,
        [Constraint::Percentage(50), Constraint::Percentage(50)],
    )
    .split(areas[1]);

    frame.render_widget(Paragraph::new(banner()), areas[0]);

    let list = List::new(state.environments.iter().collect::<Vec<_>>())
        .block(
            Block::default()
                .title("Choose an environment")
                .borders(Borders::ALL)
                .padding(Padding::uniform(1)),
        )
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">> ")
        .repeat_highlight_symbol(true);

    frame.render_stateful_widget(list, center_areas[0], state.environments.state_mut());

    if let Some(value) = state.environments.selected_index() {
        let (_, env) = environments.get(value).unwrap();
        let actions: Vec<_> = env.actions.iter().map(|a| a.to_pretty_string()).collect();
        let text = actions.iter().map(|a| format!("- {a}")).join("\n");

        let paragraph = Paragraph::new(text)
            .block(
                Block::default()
                    .title("Actions")
                    .borders(Borders::ALL)
                    .padding(Padding::uniform(1)),
            )
            .wrap(Wrap { trim: true });

        frame.render_widget(paragraph, center_areas[1]);
    }

    let status = Paragraph::new("UP/DOWN - Move cursor    ENTER - Select    Q - Quit")
        .alignment(Alignment::Center);
    frame.render_widget(status, main_areas[1]);
}
