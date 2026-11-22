// Agent actions: click, type, scroll, key
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Click { x: i32, y: i32, button: MouseButton },
    DoubleClick { x: i32, y: i32 },
    Type { text: String },
    KeyPress { key: String },
    Scroll { dx: i32, dy: i32 },
    Wait { ms: u64 },
}

#[derive(Debug, Clone, PartialEq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

impl Action {
    pub fn is_destructive(&self) -> bool {
        matches!(self, Action::Type { text } if text.contains("delete") || text.contains("remove"))
    }
}
