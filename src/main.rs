mod view;

use std::process::exit;

use chiropterm::*;
use chiroptui::*;
use euclid::*;
use view::{View, Message};

const ASPECT_CONFIG: AspectConfig = AspectConfig {
    pref_min_term_size: size2(80, 50),  // but expect ~112x60
    pref_max_term_size: size2(256, 256),
};

pub fn main() {
    let mut io = IO::new(
        "ghost".to_string(),
        ASPECT_CONFIG,
        |_| exit(0)
    );

    main_loop(&mut io)
}

fn main_loop(io: &mut IO) {
    let theme = Theme::W95_FRUITY;
    let ui = UI::new(theme);

    let mut view = View::new();

    view.chat.add_message(Message { 
        sigil: "@".to_string(),
        sender: "Nyeogmi".to_string(), 
        body: "Hey! It's me, a bat! I thought I should send this message.".to_string(),
        meta: "127.0.0.1".to_string(),
    });
    view.chat.add_message(Message { 
        sigil: "@".to_string(),
        sender: "Nyeogmi".to_string(), 
        body: "I hope you like bats too!".to_string(),
        meta: "127.0.0.1".to_string(),
    });
    view.chat.add_message(Message { 
        sigil: " ".to_string(),
        sender: "Patrick".to_string(), 
        body: "BOX".to_string(),
        meta: "192.168.0.14".to_string(),
    });
    view.chat.add_message(Message { 
        sigil: " ".to_string(),
        sender: "Patrick".to_string(), 
        body: "BOX".to_string(),
        meta: "192.168.0.14".to_string(),
    });

    io.menu(|out, menu: Menu| {
        out.brush().fill(FSem::new().color(ui.theme().base.wallpaper));
        view.draw(ui.share(), out.brush().region(out.rect().inflate(-2, -2)), menu)
    });
}