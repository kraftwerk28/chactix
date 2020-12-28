use cursive::{
    theme::{BorderStyle, Theme},
    view::{Identifiable, Resizable},
    views::{EditView, LinearLayout, ListChild, ListView, TextView},
    Cursive,
};
use message::{ClientMessage, Message as ServerMessage};
use std::error::Error;

use websocket::{ClientBuilder, Message};

fn main() -> Result<(), Box<dyn Error>> {
    let (mut rx, mut tx) = ClientBuilder::new("ws://127.0.0.1:8080")?
        .connect_insecure()?
        .split()?;

    let mut siv = cursive::default();
    tx.send_message(&Message::text("kek"))?;

    siv.set_theme({
        let mut theme = Theme::default();
        theme.shadow = false;
        theme.borders = BorderStyle::Outset;
        theme
    });

    let on_submit = move |cur: &mut Cursive, text: &str| {
        let mut e = cur.find_name::<EditView>("m").unwrap();
        e.set_content("");
        let msg = ClientMessage::Text(text.to_string());
        let raw = &serde_json::to_string(&msg).unwrap();
        let ws_msg = Message::text(raw);
        tx.send_message(&ws_msg).ok();
    };

    siv.add_layer(
        LinearLayout::vertical()
            .child(ListView::new().full_width().full_height().with_name("l"))
            .child(
                EditView::new()
                    .on_submit_mut(on_submit)
                    .with_name("m")
                    .full_width(),
            ),
    );

    std::thread::spawn({
        let mut list = siv.find_name::<ListView>("l").unwrap();
        move || loop {
            use websocket::OwnedMessage::*;
            let msg = rx.recv_message().unwrap();
            match msg {
                Text(text) => {
                    if let Ok(msg) =
                        serde_json::from_str::<ServerMessage>(&text)
                    {
                        if let ServerMessage::Msg(u, m) = msg {
                            let ch = TextView::new(format!("{}: {}", u, m));
                            list.add_child("Message", ch);
                        }
                    }
                }
                Close(reason) => {
                    if let Some(r) = reason {
                        println!(
                            "Connection closed ({}): `{}`",
                            r.status_code, r.reason
                        );
                    }
                }
                _ => (),
            }
        }
    });

    Ok(siv.run())
}
