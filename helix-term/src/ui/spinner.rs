#[cfg(test)]
use mock_instant::Instant;

#[cfg(not(test))]
use std::time::Instant;

use std::collections::HashMap;

use helix_view::editor::BRAILLE_SPINNER_STRINGS;

#[derive(Default, Debug)]
pub struct ProgressSpinners {
    default: Spinner,
    inner: HashMap<usize, Spinner>,
}

impl ProgressSpinners {
    pub fn get(&self, id: usize) -> Option<&Spinner> {
        self.inner.get(&id)
    }

    pub fn get_or_create(&mut self, id: usize) -> &mut Spinner {
        self.inner.entry(id).or_insert_with(|| self.default.clone())
    }

    pub fn new(default: Spinner) -> Self {
        Self {
            default,
            inner: HashMap::new(),
        }
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self::dots(80)
    }
}

#[derive(Clone, Debug)]
pub struct Spinner {
    frames: Vec<String>,
    count: usize,
    start: Option<Instant>,
    interval: u64,
}

impl Spinner {
    /// Creates a new spinner with `frames` and `interval`.
    /// If either the frames count or interval is zero, create an empty spinner
    /// that won't display anything.
    pub fn new(frames: Vec<String>, interval: u64) -> Self {
        let count = frames.len();
        if count == 0 || interval == 0 {
            // disable the spinner
            return Self {
                frames: vec!["".to_string()],
                count: 1,
                interval: 1, // this doesn't matter if count == 1
                start: None,
            };
        }

        Self {
            frames,
            count,
            interval,
            start: None,
        }
    }

    pub fn dots(interval: u64) -> Self {
        Self::new(
            BRAILLE_SPINNER_STRINGS
                .into_iter()
                .map(String::from)
                .collect(),
            interval,
        )
    }

    pub fn start(&mut self) {
        self.start = Some(Instant::now());
    }

    pub fn frame(&self) -> Option<&str> {
        // if 1 == self.count {
        //     // optional short circuit for disabled or length 1 spinner as there is nothing to calculate
        //     return self.start.and(self.frames.get(0).map(|s| s.as_str()))
        // }
        let idx = (self
            .start
            .map(|time| Instant::now().duration_since(time))?
            .as_millis()
            / self.interval as u128) as usize
            % self.count;

        self.frames.get(idx).map(|s| s.as_str())
    }

    pub fn stop(&mut self) {
        self.start = None;
    }

    pub fn is_stopped(&self) -> bool {
        self.start.is_none()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use mock_instant::MockClock;
    use std::time::Duration;

    #[test]
    fn test_default_spinner() {
        let mut s = Spinner::dots(80);
        assert_eq!(s.is_stopped(), true);
        assert_eq!(s.frame(), None);
        s.start();
        assert_eq!(s.is_stopped(), false);
        assert_eq!(s.frame(), Some(BRAILLE_SPINNER_STRINGS[0]));
        s.stop();
        assert_eq!(s.is_stopped(), true);
        s.start();
        assert_eq!(s.frame(), Some(BRAILLE_SPINNER_STRINGS[0]));
        MockClock::advance(Duration::from_millis(40));
        assert_eq!(s.frame(), Some(BRAILLE_SPINNER_STRINGS[0]));
        MockClock::advance(Duration::from_millis(40));
        for idx in 1..8 {
            assert_eq!(s.frame(), Some(BRAILLE_SPINNER_STRINGS[idx]));
            MockClock::advance(Duration::from_millis(80));
        }
        // looping back to beginning
        assert_eq!(s.frame(), Some(BRAILLE_SPINNER_STRINGS[0]));
    }

    #[test]
    fn test_empty_spinner() {
        let mut s = Spinner::new(vec![], 80);
        assert_eq!(s.is_stopped(), true);
        assert_eq!(s.frame(), None);
        s.start();
        assert_eq!(s.is_stopped(), false);
        assert_eq!(s.frame(), Some(""));
        s.stop();
        assert_eq!(s.is_stopped(), true);
        s.start();
        assert_eq!(s.frame(), Some(""));
        MockClock::advance(Duration::from_millis(40));
        assert_eq!(s.frame(), Some(""));
        MockClock::advance(Duration::from_millis(40));
        for _idx in 1..8 {
            assert_eq!(s.frame(), Some(""));
            MockClock::advance(Duration::from_millis(80));
        }
        // looping back to beginning
        assert_eq!(s.frame(), Some(""));
    }

    #[test]
    fn test_zero_interval() {
        const SIMPLE_SPINNER_STRINGS: [char; 4] = ['-', '\'', '|', '/'];

        let mut s = Spinner::new(
            SIMPLE_SPINNER_STRINGS
                .into_iter()
                .map(String::from)
                .collect(),
            0,
        );

        assert_eq!(s.is_stopped(), true);
        assert_eq!(s.frame(), None);
        s.start();
        assert_eq!(s.is_stopped(), false);
        assert_eq!(s.frame(), Some(""));
        s.stop();
        assert_eq!(s.is_stopped(), true);
        s.start();
        assert_eq!(s.frame(), Some(""));
        MockClock::advance(Duration::from_millis(40));
        assert_eq!(s.frame(), Some(""));
        MockClock::advance(Duration::from_millis(40));
        for _idx in 1..4 {
            assert_eq!(s.frame(), Some(""));
            MockClock::advance(Duration::from_millis(80));
        }
        // looping back to beginning
        assert_eq!(s.frame(), Some(""));
    }

    #[test]
    fn test_length_one_spinner() {
        const SINGLE_SPINNER_STRINGS: [char; 1] = ['X'];

        let mut s = Spinner::new(
            SINGLE_SPINNER_STRINGS
                .into_iter()
                .map(String::from)
                .collect(),
            80,
        );

        assert_eq!(s.is_stopped(), true);
        assert_eq!(s.frame(), None);
        s.start();
        assert_eq!(s.is_stopped(), false);
        assert_eq!(s.frame(), Some("X"));
        s.stop();
        assert_eq!(s.is_stopped(), true);
        s.start();
        assert_eq!(s.frame(), Some("X"));
        MockClock::advance(Duration::from_millis(40));
        assert_eq!(s.frame(), Some("X"));
        MockClock::advance(Duration::from_millis(40));
        for _idx in 1..4 {
            assert_eq!(s.frame(), Some("X"));
            MockClock::advance(Duration::from_millis(80));
        }
        // keep returning same
        assert_eq!(s.frame(), Some("X"));
    }

    #[test]
    fn test_custom_char_spinner() {
        const SIMPLE_SPINNER_CHARS: [char; 4] = ['-', '\\', '|', '/'];

        let mut s = Spinner::new(
            SIMPLE_SPINNER_CHARS.into_iter().map(String::from).collect(),
            80,
        );

        assert_eq!(s.is_stopped(), true);
        assert_eq!(s.frame(), None);
        s.start();
        assert_eq!(s.is_stopped(), false);
        assert_eq!(s.frame(), Some("-"));
        s.stop();
        assert_eq!(s.is_stopped(), true);
        s.start();
        assert_eq!(s.frame(), Some("-"));
        MockClock::advance(Duration::from_millis(40));
        assert_eq!(s.frame(), Some("-"));
        MockClock::advance(Duration::from_millis(40));

        assert_eq!(s.frame(), Some("\\"));
        MockClock::advance(Duration::from_millis(80));
        assert_eq!(s.frame(), Some("|"));
        MockClock::advance(Duration::from_millis(80));
        assert_eq!(s.frame(), Some("/"));
        MockClock::advance(Duration::from_millis(80));

        // looping back to beginning
        assert_eq!(s.frame(), Some("-"));
    }

    #[test]
    fn test_custom_string_spinner() {
        const SIMPLE_SPINNER_STRINGS: [&str; 4] = ["-=", "\\-", "|=", "/-"];

        let mut s = Spinner::new(
            SIMPLE_SPINNER_STRINGS
                .into_iter()
                .map(String::from)
                .collect(),
            80,
        );

        assert_eq!(s.is_stopped(), true);
        assert_eq!(s.frame(), None);
        s.start();
        assert_eq!(s.is_stopped(), false);
        assert_eq!(s.frame(), Some("-="));
        s.stop();
        assert_eq!(s.is_stopped(), true);
        s.start();
        assert_eq!(s.frame(), Some("-="));
        MockClock::advance(Duration::from_millis(40));
        assert_eq!(s.frame(), Some("-="));
        MockClock::advance(Duration::from_millis(40));

        assert_eq!(s.frame(), Some("\\-"));
        MockClock::advance(Duration::from_millis(80));
        assert_eq!(s.frame(), Some("|="));
        MockClock::advance(Duration::from_millis(80));
        assert_eq!(s.frame(), Some("/-"));
        MockClock::advance(Duration::from_millis(80));
        // looping back to beginning
        assert_eq!(s.frame(), Some("-="));
    }
}
