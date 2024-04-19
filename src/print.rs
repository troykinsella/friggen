use std::time::SystemTime;

use colored::{ColoredString, Colorize};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PrintTheme {
    ThisFriggenKicksAss,
    ThisFriggenSucks,
}

impl PrintTheme {
    #[inline]
    pub fn fg(&self, text: &str) -> ColoredString {
        match self {
            Self::ThisFriggenKicksAss => text.bright_blue().bold(),
            Self::ThisFriggenSucks => text.yellow().bold(),
        }
    }

    #[inline]
    pub fn bg(&self, text: &str) -> ColoredString {
        match self {
            Self::ThisFriggenKicksAss => text.green(),
            Self::ThisFriggenSucks => text.red(),
        }
    }
}

#[derive(Debug)]
pub struct OutputPrinter {
    theme: PrintTheme,
    quiet: bool,
}

impl OutputPrinter {
    #[inline]
    pub fn new(theme: PrintTheme, quiet: bool) -> Self {
        Self { theme, quiet }
    }

    #[inline]
    pub fn with_theme(&self, theme: PrintTheme) -> Self {
        Self {
            theme,
            quiet: self.quiet,
        }
    }

    #[inline]
    pub fn print_header(&self, text: &str) {
        if self.quiet {
            return;
        }

        println!(
            "{} {} {}",
            self.theme.bg("○──("),
            self.theme.fg(text),
            self.theme.bg(")──○"),
        );
    }

    #[inline]
    pub fn print_timed_header(&self, text: &str, start: SystemTime) {
        if self.quiet {
            return;
        }

        let elapsed = SystemTime::now().duration_since(start).unwrap();
        println!(
            "{} {} {} {} {}",
            self.theme.bg("○──("),
            self.theme.fg(text),
            self.theme.bg(")──("),
            self.theme.fg(&format!("{:.3} sec.", elapsed.as_secs_f32())),
            self.theme.bg(")──○"),
        );
    }

    #[inline]
    pub fn print_section_header(&self, title: &str) {
        if self.quiet {
            return;
        }

        println!(
            "{} {} {}",
            self.theme.bg("╭──("),
            self.theme.fg(title),
            self.theme.bg(")──○")
        );
    }

    #[inline]
    pub fn print_section_line(&self, line: &str) {
        if self.quiet {
            return;
        }

        println!("{} {}", self.theme.bg("│"), line);
    }

    #[inline]
    pub fn print_section_footer(&self) {
        if self.quiet {
            return;
        }

        println!("{}", self.theme.bg("╰──○"));
    }
}
