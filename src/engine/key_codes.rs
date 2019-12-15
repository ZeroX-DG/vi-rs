#[cfg(target_os = "linux")]
pub mod keycodes {
    // alphabet
    pub const KeyA: u32 = 38;
    pub const KeyB: u32 = 56;
    pub const KeyC: u32 = 54;
    pub const KeyD: u32 = 40;
    pub const KeyE: u32 = 26;
    pub const KeyF: u32 = 41;
    pub const KeyG: u32 = 42;
    pub const KeyH: u32 = 43;
    pub const KeyI: u32 = 31;
    pub const KeyJ: u32 = 44;
    pub const KeyK: u32 = 45;
    pub const KeyL: u32 = 46;
    pub const KeyM: u32 = 58;
    pub const KeyN: u32 = 57;
    pub const KeyO: u32 = 32;
    pub const KeyP: u32 = 33;
    pub const KeyQ: u32 = 24;
    pub const KeyR: u32 = 27;
    pub const KeyS: u32 = 39;
    pub const KeyT: u32 = 28;
    pub const KeyU: u32 = 30;
    pub const KeyV: u32 = 55;
    pub const KeyW: u32 = 25;
    pub const KeyX: u32 = 53;
    pub const KeyY: u32 = 29;
    pub const KeyZ: u32 = 52;
    // numbers
    pub const Key1: u32 = 10;
    pub const Key2: u32 = 11;
    pub const Key3: u32 = 12;
    pub const Key4: u32 = 13;
    pub const Key5: u32 = 14;
    pub const Key6: u32 = 15;
    pub const Key7: u32 = 16;
    pub const Key8: u32 = 17;
    pub const Key9: u32 = 18;
    pub const Key0: u32 = 19;
    // other
    pub const Key_ESC: u32       = 9;
    pub const Key_DELETE: u32    = 22;
    pub const Key_TAB: u32       = 23;
    pub const Key_ENTER: u32     = 36;
    pub const Key_RETURN: u32    = 36;
    pub const Key_SPACE: u32     = 65;
    pub const Key_LEFT: u32      = 113;
    pub const Key_RIGHT: u32     = 114;
    pub const Key_DOWN: u32      = 116;
    pub const Key_UP: u32        = 111;
    pub const Key_BACKSPACE: u32 = 65288;
}
