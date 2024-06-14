use rich_progress_bar::RichProgressBar;
use rich_progress_bar::Colors;

#[test]
fn test_progress_bar_starts_at_zero() {
    let progress = RichProgressBar::new();
    assert_eq!(progress.get_current(), 0);
}

#[test]
fn test_progress_bar_color() {
    let mut progress = RichProgressBar::new();
    progress.set_color(Colors::Green);
    assert_eq!(progress.get_color(), Colors::Green);
}

#[test]
fn test_increment_progress() {
    let mut progress = RichProgressBar::new();
    progress.set_total(10);
    progress.inc();
    assert_eq!(progress.get_current(), 1);
}

