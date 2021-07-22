use super::*;

/// Build window size hook with config
#[derive(Debug, Copy, Clone)]
pub struct UseWindowBuilder {
    missing_x: usize,
    missing_y: usize,
}

impl Default for UseWindowBuilder {
    fn default() -> Self {
        Self { missing_x: 375, missing_y: 812 }
    }
}

impl UseWindowBuilder {
    /// hooks for window's size with config
    ///
    /// # Arguments
    ///
    /// returns: [`WindowSize`]
    ///
    /// # Examples
    ///
    /// ```
    /// use dioxus::prelude::*;
    /// use dioxus_use_window::use_window_size;
    ///
    /// fn App(cx: Scope) -> Element {
    ///     let size = use_window_size(&cx);
    ///
    ///     cx.render(rsx!(
    ///         h1 { "Window size: {size}" }
    ///     ))
    /// }
    /// ```
    pub fn use_window_size<'a, 'b>(&'a self, cx: &'b ScopeState) -> &'b mut WindowSize {
        let x = self.missing_x as f64;
        let y = self.missing_y as f64;
        let hook = match WindowSize::new(cx, x, y) {
            None => {
                let data = WindowSizeData { x, y };
                WindowSize { data: Rc::new(RefCell::new(data)), listen_window: None }
            }
            Some(s) => s,
        };
        cx.use_hook(|_| hook)
    }
}
