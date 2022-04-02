use chiroptui::*;

pub struct Ssh {
    pub(in super) widget: Widget<WindowState>,
}

impl Ssh {
    pub(in super) fn new() -> Self {
        let ssh_label = Label::new().setup(|l| { l.set_text("Hello, world!") });

        let ssh_window = Window::new();
        ssh_window.setup(|w| {
            w.set_title("ssh: ~");
            w.set(ssh_label.share());
            w.layout_hacks.preferred_width = Some(53);
        });

        Ssh {
            widget: ssh_window,
        }
    }
}