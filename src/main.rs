use iced::{Application, Settings};
use logrust_ui::view::LogUIView;


fn main() -> iced::Result {
  LogUIView::run(Settings::default())
}
