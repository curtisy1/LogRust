use iced::{
  button,
  Application,
  Button,
  Column,
  Command,
  Container,
  Element,
  Length,
  Rule,
  Text,
};
use rfd::{AsyncFileDialog};
use std::{path::Path};

use logrust_core::{file_handler::{self, LogEntry}};
use crate::style;

pub struct LogUIView {
  files: file_handler::LogEntry,
  button_state: button::State,
  theme: style::Theme,
}

#[derive(Debug, Clone)]
pub enum Message {
  ButtonPressed,
  FilesSelected(Box<Path>),
  ThemeChanged(style::Theme),
}

impl Application for LogUIView {
  type Executor = iced::executor::Default;
  type Message = Message;
  type Flags = ();

  fn new(_flags: ()) -> (LogUIView, Command<Message>) {
    (
      LogUIView {
        theme: style::Theme::Dark,
        files: LogEntry::new(),
        button_state: button::State::new(),
      },
      Command::none(),
    )
  }

  fn title(&self) -> String {
    String::from("LogRust")
  }

  fn update(&mut self, message: Message, _clipboard: &mut iced::Clipboard) -> Command<Message> {
    match message {
      Message::ThemeChanged(theme) => {
        self.theme = theme;

        Command::none()
      },
      Message::ButtonPressed => Command::perform(
        async {
          let opened_files = AsyncFileDialog::new().pick_file().await;
          opened_files.expect("Should have picked a file").path().into()
        },
        Message::FilesSelected,
      ),
      Message::FilesSelected(file) => {
        self.files = file_handler::parse_log_file(std::fs::File::open(file).expect("fsgsgs")).expect("Something failed");

        Command::none()
      }
    }
  }

  fn view(&mut self) -> Element<Message> {
    let LogUIView { files, .. } = self;

    let file_picker = Button::new(&mut self.button_state, Text::new("Choose a file"))
      .style(self.theme)
      .on_press(Message::ButtonPressed);

    let debug_lines = Text::new(format!("Contains {} debug statements", files.line_severity.debug));
    let info_lines = Text::new(format!("Contains {} info statements", files.line_severity.info));
    let error_lines = Text::new(format!("Contains {} error statements", files.line_severity.error));
    let warn_lines = Text::new(format!("Contains {} warn statements", files.line_severity.warn));
    let trace_lines = Text::new(format!("Contains {} trace statements", files.line_severity.trace));
    let other_lines = Text::new(format!("Contains {} other statements", files.line_severity.other));
    let total_files = Text::new(format!("{} total lines", files.total_lines));

    let content = Column::new()
      .spacing(20)
      .padding(20)
      .push(file_picker)
      .push(Rule::horizontal(20))
      .push(trace_lines)
      .push(debug_lines)
      .push(info_lines)
      .push(warn_lines)
      .push(error_lines)
      .push(other_lines)
      .push(total_files);

    Container::new(content)
      .width(Length::Fill)
      .height(Length::Fill)
      .style(self.theme)
      .center_x()
      .center_y()
      .into()
  }
}