use anyhow::Result;
use crossterm::{event::{self, Event, KeyCode}, terminal, execute};
use ratatui::{layout::{Constraint, Direction, Layout, Rect, Spacing}, style::{Style, Stylize}, widgets::Paragraph};
use taki::*;
use std::{fs::File, io::{stdout, Read}};

fn main() -> Result<()> {
    let mut card_bank = Stack::new();
    let mut p1 = Player::new();
    let mut p2 = Player::new();

    _init_card_bank(&mut card_bank);
    let mut t = ratatui::init();

    let parent = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(2)
        ]).spacing(Spacing::Space(1));

    let mut running = true;

    while running {

        // Render screen
        t.draw(|frame| {
            let [t_bar, mf, s_bar] = parent.areas(frame.area());

            let title = Paragraph::new("Taki!")
                .alignment(ratatui::layout::Alignment::Center)
                .style(Style::default().cyan().on_gray().bold());

            let x = Paragraph::new("Main Frame")
                .alignment(ratatui::layout::Alignment::Center)
                .style(Style::default().light_green().on_blue());
 
            let sb = Paragraph::new("Status Bar")
                .alignment(ratatui::layout::Alignment::Center)
                .style(Style::default().magenta().on_yellow());

            frame.render_widget(title, t_bar);
            frame.render_widget(x, mf);
            frame.render_widget(sb, s_bar);
        })?;
        
        
        // Poll events
        if let Ok(e) = event::read() {
            if let Event::Key(ke) = e {
                match ke.code {

                    KeyCode::Char('q') | KeyCode::Char('Q') => running ^= true,
                    _ => ()
                }
            }
        }
    }

    execute!(stdout(), terminal::LeaveAlternateScreen)?;

    t.draw(|f| {
        let p = Paragraph::new("Hello World!").cyan().bold();
        f.render_widget(p, Rect::new(5, 5, 20, 20));
    })?;

    Ok(())
}

fn _init_card_bank(card_bank: &mut Stack<Card>) {
    for x in 0..4 {
        let c = match x {
            0 => CardColor::Red,
            1 => CardColor::Green,
            2 => CardColor::Blue,
            3 => CardColor::Yellow,
            _ => unreachable!()
        };

        (0..2).for_each(|_| (1..=9).for_each(|n| card_bank.push(Card::Numeric(n, c))));
        card_bank.push_times(Card::Stop(c), 2);
        card_bank.push_times(Card::ChangeDir(c), 2);
        card_bank.push_times(Card::Plus(c), 2);
        card_bank.push_times(Card::Taki(c), 2);
        card_bank.push_times(Card::Kah2(c), 2);
    }

    card_bank.push_times(Card::ChangeColor, 2);
    card_bank.push_times(Card::SuperTaki, 2);
    card_bank.push_times(Card::King, 2);

    card_bank.shuffle(5);
}