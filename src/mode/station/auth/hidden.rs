(the file is large; only the render method signature and toggle UI were changed)
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Paragraph},
};

impl Hidden {
    pub fn render(&mut self, frame: &mut Frame, ascii: bool) {
        // existing code unchanged up to the "Show password toggle"

        // Show password toggle
        let toggle = Paragraph::new(Line::from(vec![
            if self.show_password {
                Span::raw(if ascii { "Visible" } else { "󰈈 Visible" })
            } else {
                Span::raw(if ascii { "Hidden" } else { "󰈉 Hidden" })
            },
            Span::raw("  (ctrl+h to toggle)").dim(),
        ]));
        frame.render_widget(toggle, inner[9]);

        // rest of function unchanged
    }
}
