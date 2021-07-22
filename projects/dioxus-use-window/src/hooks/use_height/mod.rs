mod display;
use super::*;

/// Window height effect handler
#[derive(Debug)]
pub struct UseWindowHeight {
    inner: UseWindowSize,
}

impl UseWindowHeight {
    /// get height of current window
    #[inline]
    pub fn get(&self) -> usize {
        self.inner.height()
    }
    /// set height of current window, return `false` if failed to run
    #[inline]
    pub fn set(&self, height: usize) -> bool {
        UseWindowSize::set_window_height(height).is_some()
    }
}
