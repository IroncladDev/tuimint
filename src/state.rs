pub struct AppState {
    pub count: i32,
    pub should_quit: bool
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            count: 0,
            should_quit: false
        }
    }

    pub fn increment(&mut self) {
        self.count += 1;
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
