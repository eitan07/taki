use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute, terminal,
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Offset, Rect, Spacing},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Paragraph},
};
use std::io::stdout;
use taki::{disp::CardSetLoader, *};

fn main() -> Result<()> {
    let mut card_bank = Stack::new();
    let mut p1 = Player::new();
    // let mut p2 = Player::new();

    let cardset = CardSetLoader::load_from_file("assets/cards_design.txt")?;

    execute!(stdout(), terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    _init_card_bank(&mut card_bank, 5);
    let mut t = ratatui::init();

    let parent_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Fill(1)])
        .spacing(Spacing::Space(1));

    let mut running = true;

    for _ in 0..8 {
        p1.cards.push(card_bank.pop().unwrap());
        p1.cards_count += 1;
    }

    let mut p1_cards_widgets: Vec<CardWidget>;
    
    let back_card_widget = CardWidget::new(&cardset, &Card::CardsBack);
    while running {
        // if event::poll(Duration::from_millis(500))? {

        p1_cards_widgets = p1
        .cards
        .clone()
        .into_iter()
        .map(|card| CardWidget::new(&cardset, &card))
        .collect::<Vec<_>>();

        // Render screen
        t.draw(|mainframe| {
            let [titlearea, mainarea] = parent_layout.areas(mainframe.area());

            let title = Paragraph::new("Taki!")
                .alignment(ratatui::layout::Alignment::Center)
                .style(Style::default().cyan().on_gray().bold());

            let cards_view = Rect {
                x: mainarea.width / 2 - 4 * p1.cards_count as u16 + 1,
                y: mainarea.height - 10,
                height: 10,
                width: 8 * p1.cards_count as u16 + 3,
            };
            let cards_block = Block::new()
                .title("Your Cards")
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Plain)
                .border_style(Color::White);

            let card_bank_view = Rect {
                x: 3 * (mainarea.width / 4),
                y: mainarea.height / 2,
                width: 9,
                height: 8,
            };
            let card_bank_block = Block::new()
                .title("Bank")
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Plain)
                .border_style(Color::White);

            for i in 0..p1.cards_count {
                mainframe.render_widget(
                    p1_cards_widgets[i].clone(),
                    Rect {
                        x: cards_view.x + i as u16 * 8 + 2,
                        y: cards_view.y + 2,
                        height: 6,
                        width: 7,
                    },
                );
            }

            mainframe.render_widget(
                back_card_widget.clone(),
                card_bank_view.offset(Offset { x: 1, y: 1 }),
            );

            mainframe.render_widget(title, titlearea);
            mainframe.render_widget(cards_block, cards_view);
            mainframe.render_widget(card_bank_block, card_bank_view);
        })?;

        // Poll events
        if let Ok(e) = event::read() {
            if let Event::Key(ke) = e {
                match ke.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => running ^= true,
                    KeyCode::Char(' ') => { p1.cards.push(card_bank.pop().unwrap()); p1.cards_count += 1 },
                    _ => (),
                }
            }
        }
    }

    terminal::disable_raw_mode()?;
    execute!(stdout(), terminal::LeaveAlternateScreen)?;

    Ok(())
}

fn _init_card_bank(card_bank: &mut Stack<Card>, shiters: usize) {
    for x in 0..4 {
        let c = match x {
            0 => CardColor::Red,
            1 => CardColor::Green,
            2 => CardColor::Blue,
            3 => CardColor::Yellow,
            _ => unreachable!(),
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

    card_bank.shuffle(shiters);
}
