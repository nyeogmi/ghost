use chiroptui::*;

use super::line_output::LineOutput;

pub struct Chat {
    pub(in super) widget: Widget<WindowState>,
    canvas: Widget<CanvasState>,

    last_sender: Option<String>,
    line_output: LineOutput,
}


// TODO: Joins, leaves, etc
#[derive(Clone)]
pub struct Message {
    pub sigil: String,
    pub sender: String,
    pub meta: String,
    pub body: String,
}

impl Chat {
    pub(in super) fn new() -> Self {
        let line_output = LineOutput::new();
        let canvas = Canvas::new();

        let lo = line_output.share();
        canvas.setup(move |canv| {
            canv.layout_hacks.preferred_height = Some(61);
            canv.draw = Some(Box::new(move |brush, menu| {
                lo.draw(brush, menu)
            }))
        });

        let chat_window = Window::new();
        chat_window.setup(|w| {
            w.set_title("#chat");
            w.set(canvas.share()); // Scrollable::new().setup(|sc| { sc.set(canvas.share()) }));
            w.layout_hacks.preferred_width = Some(33);
        });

        Chat {
            widget: chat_window,
            canvas,

            last_sender: None,
            line_output: line_output,
        }
    }

    pub fn add_message(&mut self, message: Message) {
        if self.last_sender.as_ref() != Some(&message.sender) {
            self.line_output.add_line("".to_string());
            self.line_output.add_line(format!("\u{ffff}\u{01}\u{04}<\u{ffff}\u{01}\u{07}{}\u{ffff}\u{01}\u{26}{}\u{ffff}\u{01}\u{04}>@\u{ffff}\u{01}\u{23}{}", message.sigil, message.sender, message.meta));
            self.line_output.add_line(message.body);
            self.last_sender = Some(message.sender);
        } else {
            self.line_output.add_line(message.body);
        }

    }
}
