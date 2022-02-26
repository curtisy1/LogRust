mod style;

use iced::{button, Button, Column, Text};

#[derive(Debug, Clone, Serialize)]
struct FilePicker {
  theme: style::Theme,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
  ButtonPressed,
}

impl FilePicker {
  fn view(&mut self) -> Button<Message> {
    Button::new(&mut self.button_state, Text::new("Open log files"))
      .style(self.theme)
      .on_press(Message::ButtonPressed)
  }

  fn update(&mut self, message: Message) {
    match message {
      Message::ButtonPressed => {
        let files = rfd::AsyncFileDialog::new().pick_files().await;
      }
    }
  }

  pub async fn pick_files() {
    let opened_files = rfd::AsyncFileDialog::new().pick_files().await; 
    match opened_files {
      Some(f) => f.len,
      None => 0
    }
  }
}
