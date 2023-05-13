use once_cell::sync::Lazy;
use tui::{
    style::{Color, Modifier, Style},
    widgets::{BorderType, Borders},
};

pub struct Input {
    pub title: &'static str,
    pub style: Style,
    pub borders: Borders,
    pub border_type: BorderType,
}

pub struct List {
    pub normal_letter: Style,
    pub matched_letter: Style,
    pub faded_letter: Style,
    pub borders: Borders,
    pub border_type: BorderType,
    pub highligh_symbol: &'static str,
}

pub struct Styles {
    pub v_margin: u16,
    pub h_margin: u16,
    pub input: Input,
    pub list: List,
}

static STYLES: Lazy<Styles> = Lazy::new(|| Styles {
    v_margin: 1,
    h_margin: 6,
    input: Input {
        title: "Search",
        style: Style::default().fg(Color::Yellow),
        borders: Borders::ALL,
        border_type: BorderType::Plain,
    },
    list: List {
        normal_letter: Style::default().add_modifier(Modifier::ITALIC),
        matched_letter: Style::default().add_modifier(Modifier::BOLD),
        faded_letter: Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::BOLD),
        borders: Borders::ALL,
        border_type: BorderType::Thick,
        highligh_symbol: ">",
    },
});

pub fn styles() -> &'static Styles {
    &STYLES
}
