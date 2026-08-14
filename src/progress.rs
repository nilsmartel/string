const WIDTH: usize = 30;

/// A progress bar
/// Rendered on stderr,
/// so it stays out of the way of the results on stdout.
pub struct ProgressBar {
    items: usize,
    done: usize,
    enabled: bool,
}

impl ProgressBar {
    pub fn new(items: usize, enabled: bool) -> Self {
        ProgressBar {
            items,
            done: 0,
            enabled: enabled && items > 0,
        }
    }

    /// Count one finished item and redraw the bar.
    pub fn tick(&mut self) {
        self.done += 1;
        self.draw();
    }

    fn draw(&mut self) {
        if !self.enabled {
            return;
        }

        let done = self.done.min(self.items);
        let filled = done * WIDTH / self.items;

        // eprint! rather than a raw stderr handle, so the test harness captures it
        eprintln!(
            "\r[{}{}] {}/{}",
            "#".repeat(filled),
            "-".repeat(WIDTH - filled),
            done,
            self.items,
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn counts_up_to_completion() {
        let mut progress = ProgressBar::new(4, true);
        for _ in 0..4 {
            progress.tick();
        }

        assert_eq!(progress.done, 4);
    }
}
