mod chat;
mod line_output;
mod ssh;

use chiropterm::{Brush, Menu};
use chiroptui::{Widget, RowState, Row, UI};

pub use chat::{Chat, Message};
pub use ssh::Ssh;

pub struct View {
    pub chat: Chat,
    pub ssh: Ssh,

    widget: Widget<RowState>,
}

impl View {
    pub fn new() -> Self {
        let chat = Chat::new();
        let ssh = Ssh::new();

        let row = Row::new();
        row.setup(|c| {
            c.add(chat.widget.share());
            c.add(ssh.widget.share());
        });

        View {
            chat,
            ssh,
            widget: row
        }
    }

    pub(crate) fn draw(&self, ui: UI, brush: Brush, menu: Menu) {
        self.widget.draw(ui, brush, menu)
    }
}
