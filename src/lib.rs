use colored::*;
use std::io::{self, Write};

pub use colored::Color as Colors;

/// This enum represents the different display modes for the progress bar.
#[derive(Debug)]
pub enum DisplayMode {
    Inline,
    NewLine,
}

/// A progress bar that can be displayed in the console.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use rich_progress_bar::RichProgressBar;
/// use rich_progress_bar::Colors;
/// use rich_progress_bar::DisplayMode;
///
/// let mut progress = RichProgressBar::new();
/// progress
///     .set_color(Colors::Black)
///     .set_bar_length(80)
///     .set_display_mode(DisplayMode::Inline)
///     .set_total(100);
///
/// for _ in 0..100 {
///     progress.inc();
///     std::thread::sleep(std::time::Duration::from_millis(150));
/// }
/// ```
///
#[derive(Debug)]
pub struct RichProgressBar {
    total: u64,
    current: u64,
    bar_length: usize,
    color: colored::Color,
    display_mode: DisplayMode,
}

impl RichProgressBar {
    /// Creates a new progress bar with default settings.
    ///
    /// # Returns
    ///
    /// A new `RichProgressBar` instance with default settings.
    pub fn new() -> RichProgressBar {
        RichProgressBar {
            total: 100,
            current: 0,
            bar_length: 90,
            color: Colors::White,
            display_mode: DisplayMode::Inline,
        }
    }

    /// Retrieves the current progress
    ///
    /// # Returns
    ///
    /// The current progress number
    pub fn get_current(&self) -> u64 {
        self.current
    }

    /// Sets the color of the progress bar.
    ///
    /// # Arguments
    ///
    /// * `color` - A `Colors` enum representing the color of the progress bar.
    ///
    /// # Returns
    ///
    /// The current `RichProgressBar` instance.
    pub fn set_color(&mut self, color: Colors) -> &mut Self {
        self.color = color;
        self
    }

    /// Retrieves the current color of the progress bar.
    ///
    /// # Returns
    ///
    /// The current color setting of the progress bar as a `Colors` enum.
    pub fn get_color(&self) -> Colors {
        self.color
    }

    /// Sets the total value of the progress bar.
    ///
    /// # Arguments
    ///
    /// * `total` - A `u64` representing the total value of the progress bar.
    ///
    /// # Returns
    ///
    /// The current `RichProgressBar` instance.
    pub fn set_total(&mut self, total: u64) -> &mut Self {
        self.total = total;
        self
    }

    /// Retrieves the total value of the progress bar.
    ///
    /// # Returns
    ///
    /// The total value of the progress bar as a `u64`.
    pub fn get_total(&self) -> u64 {
        self.total
    }

    /// Sets the display mode of the progress bar.
    ///
    /// # Arguments
    ///
    /// * `display_mode` - A `DisplayMode` enum representing the display mode of the progress bar.
    ///
    /// # Returns
    ///
    /// The current `RichProgressBar` instance.
    pub fn set_display_mode(&mut self, display_mode: DisplayMode) -> &mut Self {
        self.display_mode = display_mode;
        self
    }

    /// Retrieves the display mode of the progress bar.
    ///
    /// # Returns
    ///
    /// The current display mode of the progress bar as a `DisplayMode` enum.
    pub fn get_display_mode(&self) -> &DisplayMode {
        &self.display_mode
    }

    /// Sets the length of the progress bar.
    ///
    /// # Arguments
    ///
    /// * `bar_length` - A `usize` representing the length of the progress bar.
    ///
    /// # Returns
    ///
    /// The current `RichProgressBar` instance.
    pub fn set_bar_length(&mut self, bar_length: usize) -> &mut Self {
        self.bar_length = bar_length;
        self
    }

    /// Retrieves the length of the progress bar.
    ///
    /// # Returns
    ///
    /// The current length of the progress bar as a `usize`.
    pub fn get_bar_length(&self) -> usize {
        self.bar_length
    }

    /// Increments the current value of the progress bar by 1.
    ///
    /// If the current value is less than the total value, it increments the current value and displays the progress bar.
    pub fn inc(&mut self) -> Result<(), std::io::Error> {
        if self.current < self.total {
            self.current += 1;
        }
        self.display()
    }

    /// Displays the progress bar.
    ///
    /// It calculates the percentage of the current value to the total value and displays the progress bar accordingly.
    fn display(&self) -> Result<(), std::io::Error>{
        let percentage = self.current as f64 / self.total as f64;
        let filled_len = (self.bar_length as f64 * percentage).round() as usize;
        let bar: String = "=".repeat(filled_len) + &" ".repeat(self.bar_length - filled_len);

        match self.display_mode {
            DisplayMode::Inline => {
                self.display_inline(&bar, percentage)
            },
            DisplayMode::NewLine => {
                self.display_new_inline(&bar, percentage);
                Ok(())
            },
        }
    }

    /// Displays the progress bar in inline mode.
    ///
    /// # Arguments
    ///
    /// * `bar` - A `&str` representing the progress bar.
    /// * `percentage` - A `f64` representing the percentage of the current value to the total value.
    fn display_inline(&self, bar: &str, percentage: f64) -> Result<(), std::io::Error>{
        print!(
            "\r[{}] {}%",
            bar.to_string().color(self.color),
            (100.0 * percentage).round().to_string().color(self.color)
        );
    
        io::stdout().flush()
    }

    /// Displays the progress bar in new line mode.
    ///
    /// # Arguments
    ///
    /// * `bar` - A `&str` representing the progress bar.
    /// * `percentage` - A `f64` representing the percentage of the current value to the total value.
    fn display_new_inline(&self, bar: &str, percentage: f64) {
        println!("\r[{:<50}] {}%", bar, (100.0 * percentage).round());
    }
}
