/// Describes different ways we can get the progress for a time period.
pub trait Progress {
    fn new() -> Self;

    /// Returns the percentage for the current progress.
    fn percent(&self) -> u32;

    /// Prints a progress bar for the current progress.
    fn print(&self);
}

pub trait OffsetProgress: {
    fn new(end: u32) -> Self;

    /// Returns the percentage for the current progress.
    fn percent(&self) -> u32;

    /// Prints a progress bar for the current progress.
    fn print(&self);
}