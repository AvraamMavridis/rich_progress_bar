use rich_progress_bar::RichProgressBar;
use rich_progress_bar::Colors;
use rich_progress_bar::DisplayMode;

fn main(){
  let mut progress = RichProgressBar::new();
  progress
    .set_color(Colors::Red)
    .set_bar_length(80)
    .set_display_mode(DisplayMode::Inline)
    .set_total(100);
  
  for _ in 0..100 {
    progress.inc();
    std::thread::sleep(std::time::Duration::from_millis(150));
  }
}


