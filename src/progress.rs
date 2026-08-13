use std::time::{Duration, Instant};

const WIDTH: usize = 30;
const REDRAW_EVERY: Duration = Duration::from_millis(50);

/// A progress bar on stderr, so it stays out of the way of the results on stdout.
pub struct Progress {
    total: usize,
    done: usize,
    enabled: bool,
    last_draw: Option<Instant>,
    on_screen: bool,
}

impl Progress {
    pub fn new(total: usize, enabled: bool) -> Self {
        Progress {
            total,
            done: 0,
            enabled: enabled && total > 0,
            last_draw: None,
            on_screen: false,
        }
    }

    /// Count one finished item and redraw the bar.
    pub fn tick(&mut self) {
        self.done += 1;

        let due = match self.last_draw {
            None => true,
            Some(last) => last.elapsed() >= REDRAW_EVERY,
        };

        // always draw the final state, no matter how recently we drew
        if due || self.done >= self.total {
            self.draw();
        }
    }

    /// Wipe the bar off the line, so something else can be printed without landing on top of it.
    pub fn clear(&mut self) {
        if !self.on_screen {
            return;
        }

        eprint!("\r{}\r", " ".repeat(WIDTH + 24));
        self.on_screen = false;
        self.last_draw = None;
    }

    pub fn finish(&mut self) {
        self.clear();
        self.enabled = false;
    }

    fn draw(&mut self) {
        if !self.enabled {
            return;
        }

        let done = self.done.min(self.total);
        let filled = done * WIDTH / self.total;

        // eprint! rather than a raw stderr handle, so the test harness captures it
        eprint!(
            "\r[{}{}] {}/{} ({}%)",
            "#".repeat(filled),
            "-".repeat(WIDTH - filled),
            done,
            self.total,
            done * 100 / self.total,
        );

        self.last_draw = Some(Instant::now());
        self.on_screen = true;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn counts_up_to_completion() {
        let mut progress = Progress::new(4, true);
        for _ in 0..4 {
            progress.tick();
        }
        progress.finish();

        assert_eq!(progress.done, 4);
    }

    /// an empty run must not divide by zero
    #[test]
    fn empty_total_is_harmless() {
        let mut progress = Progress::new(0, true);
        progress.tick();
        progress.clear();
        progress.finish();
    }

    /// ticking past the total must not panic on the repeat() of a negative remainder
    #[test]
    fn overshooting_the_total_is_harmless() {
        let mut progress = Progress::new(2, true);
        for _ in 0..5 {
            progress.tick();
        }
        progress.finish();
    }

    #[test]
    fn disabled_never_touches_the_screen() {
        let mut progress = Progress::new(10, false);
        progress.tick();

        assert!(!progress.on_screen);
    }
}
