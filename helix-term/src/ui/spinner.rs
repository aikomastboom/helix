use std::{collections::HashMap, time::Instant};

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
    frames: Vec<char>,
    count: usize,
    start: Option<Instant>,
    interval: u64,
}

impl Spinner {
    /// Creates a new spinner with `frames` and `interval`.
    /// Expects the frames count and interval to be greater than 0.
    pub fn new(framestring: &str, interval: u64) -> Self {
        let frames: Vec<char> = framestring.chars().collect();
        let count = frames.len();
        assert!(count > 0);
        assert!(interval > 0);

        Self {
            frames,
            count,
            interval,
            start: None,
        }
    }

    pub fn dots(interval: u64) -> Self {
        Self::new("⣾⣽⣻⢿⡿⣟⣯⣷", interval)
    }

    pub fn start(&mut self) {
        self.start = Some(Instant::now());
    }

    pub fn frame(&self) -> Option<String> {
        let idx = if self.count > 1 {
            (self
                .start
                .map(|time| Instant::now().duration_since(time))?
                .as_millis()
                / self.interval as u128) as usize
                % self.count
        } else {
            self.start.and(Some(0))?
        };
        Some(self.frames.get(idx)?.to_string())
    }

    pub fn stop(&mut self) {
        self.start = None;
    }

    pub fn is_stopped(&self) -> bool {
        self.start.is_none()
    }
}
