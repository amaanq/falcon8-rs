#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum KeyCode {
    // https://gist.github.com/MightyPork/6da26e382a7ad91b5496ee55fdc73db2
    Disable,      // No key pressed
    ErrOvf, // Keyboard Error Roll Over - used for all slots if too many keys are pressed ("Phantom key")
    PostFail, // Keyboard POST Fail
    ErrUndefined, // Keyboard Error Undefined
    A,      // Keyboard a and A
    B,      // Keyboard b and B
    C,      // Keyboard c and C
    D,      // Keyboard d and D
    E,      // Keyboard e and E
    F,      // Keyboard f and F
    G,      // Keyboard g and G
    H,      // Keyboard h and H
    I,      // Keyboard i and I
    J,      // Keyboard j and J
    K,      // Keyboard k and K
    L,      // Keyboard l and L
    M,      // Keyboard m and M
    N,      // Keyboard n and N
    O,      // Keyboard o and O
    P,      // Keyboard p and P
    Q,      // Keyboard q and Q
    R,      // Keyboard r and R
    S,      // Keyboard s and S
    T,      // Keyboard t and T
    U,      // Keyboard u and U
    V,      // Keyboard v and V
    W,      // Keyboard w and W
    X,      // Keyboard x and X
    Y,      // Keyboard y and Y
    Z,      // Keyboard z and Z

    One,   // Keyboard 1 and !
    Two,   // Keyboard 2 and @
    Three, // Keyboard 3 and #
    Four,  // Keyboard 4 and $
    Five,  // Keyboard 5 and %
    Six,   // Keyboard 6 and ^
    Seven, // Keyboard 7 and &
    Eight, // Keyboard 8 and *
    Nine,  // Keyboard 9 and (
    Zero,  // Keyboard 0 and )

    Enter,      // Keyboard Return (ENTER)
    Esc,        // Keyboard ESCAPE
    Backspace,  // Keyboard DELETE (Backspace)
    Tab,        // Keyboard Tab
    Space,      // Keyboard Spacebar
    Minus,      // Keyboard - and _
    Equal,      // Keyboard = and +
    Leftbrace,  // Keyboard [ and {
    Rightbrace, // Keyboard ] and }
    Backslash,  // Keyboard \ and |
    Hashtilde,  // Keyboard Non-US # and ~
    Semicolon,  // Keyboard ; and :
    Apostrophe, // Keyboard ' and "
    Grave,      // Keyboard ` and ~
    Comma,      // Keyboard , and <
    Dot,        // Keyboard . and >
    Slash,      // Keyboard / and ?
    Capslock,   // Keyboard Caps Lock

    F1,  // Keyboard F1
    F2,  // Keyboard F2
    F3,  // Keyboard F3
    F4,  // Keyboard F4
    F5,  // Keyboard F5
    F6,  // Keyboard F6
    F7,  // Keyboard F7
    F8,  // Keyboard F8
    F9,  // Keyboard F9
    F10, // Keyboard F10
    F11, // Keyboard F11
    F12, // Keyboard F12

    Sysrq,      // Keyboard Print Screen
    Scrolllock, // Keyboard Scroll Lock
    Pause,      // Keyboard Pause
    Insert,     // Keyboard Insert
    Home,       // Keyboard Home
    Pageup,     // Keyboard Page Up
    Delete,     // Keyboard Delete Forward
    End,        // Keyboard End
    Pagedown,   // Keyboard Page Down
    Right,      // Keyboard Right Arrow
    Left,       // Keyboard Left Arrow
    Down,       // Keyboard Down Arrow
    Up,         // Keyboard Up Arrow

    Numlock,    // Keyboard Num Lock and Clear
    KPSlash,    // Keypad /
    KPAsterisk, // Keypad *
    KPMinus,    // Keypad -
    KPPlus,     // Keypad +
    KPEnter,    // Keypad ENTER
    KP1,        // Keypad 1 and End
    KP2,        // Keypad 2 and Down Arrow
    KP3,        // Keypad 3 and PageDn
    KP4,        // Keypad 4 and Left Arrow
    KP5,        // Keypad 5
    KP6,        // Keypad 6 and Right Arrow
    KP7,        // Keypad 7 and Home
    KP8,        // Keypad 8 and Up Arrow
    KP9,        // Keypad 9 and Page Up
    KP0,        // Keypad 0 and Insert
    KPDot,      // Keypad . and Delete

    _102ND,  // Keyboard Non-US \ and |
    Compose, // Keyboard Application
    Power,   // Keyboard Power
    KPEqual, // Keypad =

    F13, // Keyboard F13
    F14, // Keyboard F14
    F15, // Keyboard F15
    F16, // Keyboard F16
    F17, // Keyboard F17
    F18, // Keyboard F18
    F19, // Keyboard F19
    F20, // Keyboard F20
    F21, // Keyboard F21
    F22, // Keyboard F22
    F23, // Keyboard F23
    F24, // Keyboard F24

    Open,              // Keyboard Execute
    Help,              // Keyboard Help
    Props,             // Keyboard Menu
    Front,             // Keyboard Select
    Stop,              // Keyboard Stop
    Again,             // Keyboard Again
    Undo,              // Keyboard Undo
    Cut,               // Keyboard Cut
    Copy,              // Keyboard Copy
    Paste,             // Keyboard Paste
    Find,              // Keyboard Find
    _MUTE,             // Keyboard Mute
    _VOLUP,            // Keyboard Volume Up
    _VOLDN,            // Keyboard Volume Down
    _CAPSLOCK,         // Keyboard Locking Caps Lock
    _NUMLOCK,          // Keyboard Locking Num Lock
    _SCROLLLOCK,       // Keyboard Locking Scroll Lock
    _KPCOMMA,          // Keypad Comma
    _EQUALS,           // Keypad Equal Sign
    _RO,               // Keyboard International1
    _KATAKANAHIRAGANA, // Keyboard International2
    _YEN,              // Keyboard International3
    _HENKAN,           // Keyboard International4
    _MUHENKAN,         // Keyboard International5
    _KPJPCOMMA,        // Keyboard International6
    _I7,               // Keyboard International7
    _I8,               // Keyboard International8
    _I9,               // Keyboard International9
    _HANGEUL,          // Keyboard LANG1
    _HANJA,            // Keyboard LANG2
    _KATAKANA,         // Keyboard LANG3
    _HIRAGANA,         // Keyboard LANG4
    _ZENKAKUHANKAKU,   // Keyboard LANG5
    _L6,               // Keyboard LANG6
    _L7,               // Keyboard LANG7
    _L8,               // Keyboard LANG8
    _L9,               // Keyboard LANG9
    _AE,               // Keyboard Alternate Erase
    _SYSREQ,           // Keyboard SysReq/Attention
    _CANCEL,           // Keyboard Cancel
    _CLEAR,            // Keyboard Clear
    _PRIOR,            // Keyboard Prior
    _RETURN,           // Keyboard Return
    _SEPARATOR,        // Keyboard Separator
    _OUT,              // Keyboard Out
    _OPER,             // Keyboard Oper
    _CLEARAGAIN,       // Keyboard Clear/Again
    _CRSEL,            // Keyboard CrSel/Props
    MediaLaunch,       // Media Launch
    MediaStop,         // Media Stop
    MediaPrevious,     // Media Previous
    PlayPause,         // Play/Pause
    MediaNext,         // Media Next
    Mute,              // Mute
    VolumeDown,        // Volume Up
    VolumeUp,          // Volume Down

    Layer1 = 0xB4, // Switch to Layer 1
    Layer2,        // Switch to Layer 2,
    Layer3,        // Switch to Layer 3,
    Layer4,        // Switch to Layer 4,
    Layer5,        // Switch to Layer 5,
    LayerCycle,    // Cycle through the 5 layers.

    LEDBrightnessCycle = 0xBB, // Cycle through the brightness levels.
    LedModeCycle,              // Cycle through the LED modes.

    MouseLeftClick = 0xBE, // Mouse left click
    MouseRightClick,       // Mouse right click
    MouseMiddleClick,      // Mouse middle click

    MouseBackward = 0xC3, // Mouse backward click
    MouseForward,         // Mouse forward click

    DoNotUseLaunchProgram = 0xF7, // Launch a program. (Doesn't work on Linux yet.)
    SetMacro,                     // Set a macro (Doesn't work on Linux yet.)

    LockUnlockWholeS = 0xFA, // Lock/Unlock the whole keyboard.

    // _
    // _
    // _
    // _
    // _
    // _
    // _
    // _
    // _
    // _
    // _
    // _ // Keypad 00
    // _ // Keypad 000
    // _ // Thousands Separator
    // _ // Decimal Separator
    // _ // Currency Unit
    // _ // Currency Sub-unit
    // _ // Keypad (
    // _ // Keypad )
    // _ // Keypad {
    // _ // Keypad }
    // _ // Keypad Tab
    // _ // Keypad Backspace
    // _ // Keypad A
    // _ // Keypad B
    // _ // Keypad C
    // _ // Keypad D
    // _ // Keypad E
    // _ // Keypad F
    // _ // Keypad XOR
    // _ // Keypad ^
    // _ // Keypad %
    // _ // Keypad <
    // _ // Keypad >
    // _ // Keypad &
    // _ // Keypad &&
    // _ // Keypad |
    // _ // Keypad ||
    // _ // Keypad :
    // _ // Keypad #
    // _ // Keypad Space
    // _ // Keypad @
    // _ // Keypad !
    // _ // Keypad Memory Store
    // _ // Keypad Memory Recall
    // _ // Keypad Memory Clear
    // _ // Keypad Memory Add
    // _ // Keypad Memory Subtract
    // _ // Keypad Memory Multiply
    // _ // Keypad Memory Divide
    // _ // Keypad +/-
    // _ // Keypad Clear
    // _ // Keypad Clear Entry
    // _ // Keypad Binary
    // _ // Keypad Octal
    // _ // Keypad Decimal
    // _ // Keypad Hexadecimal
    // _
    // _
    Leftctrl,  // Keyboard Left Control
    Leftshift, // Keyboard Left Shift
    Leftalt,   // Keyboard Left Alt
    Leftmeta,  // Keyboard Left GUI (AKA Windows Key/Super Key)
               // Rightctrl,  // Keyboard Right Control
               // Rightshift, // Keyboard Right Shift
               // Rightalt,   // Keyboard Right Alt
               // Rightmeta,  // Keyboard Right GUI (AKA Windows Key/Super Key)

               // _MEDIA_PLAYPAUSE
               // _MEDIA_STOPCD
               // _MEDIA_PREVIOUSSONG
               // _MEDIA_NEXTSONG
               // _MEDIA_EJECTCD
               // _MEDIA_VOLUMEUP
               // _MEDIA_VOLUMEDOWN
               // _MEDIA_MUTE
               // _MEDIA_WWW
               // _MEDIA_BACK
               // _MEDIA_FORWARD
               // _MEDIA_STOP
               // _MEDIA_FIND
               // _MEDIA_SCROLLUP
               // _MEDIA_SCROLLDOWN
               // _MEDIA_EDIT
               // _MEDIA_SLEEP
               // _MEDIA_COFFEE
               // _MEDIA_REFRESH
               // _MEDIA_CALC
}

impl std::fmt::Display for KeyCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} ({})", self, *self as u8)
    }
}
