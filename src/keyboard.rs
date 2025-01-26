use std::{
    ptr,
    sync::{Mutex, OnceLock},
};

use core_foundation::base::TCFType;

static KEYS: OnceLock<Mutex<Vec<Key>>> = OnceLock::new();
static CALLBACK: OnceLock<Mutex<Option<Box<dyn Fn(Key, bool) + Send>>>> = OnceLock::new();

/// Enum representing a key on a keyboard
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Key {
    // A-Z
    A = 4,
    B = 5,
    C = 6,
    D = 7,
    E = 8,
    F = 9,
    G = 10,
    H = 11,
    I = 12,
    J = 13,
    K = 14,
    L = 15,
    M = 16,
    N = 17,
    O = 18,
    P = 19,
    Q = 20,
    R = 21,
    S = 22,
    T = 23,
    U = 24,
    V = 25,
    W = 26,
    X = 27,
    Y = 28,
    Z = 29,

    // 0-9
    N1 = 30,
    N2 = 31,
    N3 = 32,
    N4 = 33,
    N5 = 34,
    N6 = 35,
    N7 = 36,
    N8 = 37,
    N9 = 38,
    N0 = 39,

    // Punctuation
    Hyphen = 45,
    Equal = 46,
    LeftBracket = 47,
    RightBracket = 48,
    Backslash = 49,
    Semicolon = 51,
    Apostrophe = 52,
    Grave = 53,
    Comma = 54,
    Period = 55,
    Slash = 56,

    // Special keys
    Space = 44,
    Tab = 43,
    Enter = 40,
    Backspace = 42,
    CapsLock = 57,

    // Function keys
    F1 = 58,
    F2 = 59,
    F3 = 60,
    F4 = 61,
    F5 = 62,
    F6 = 63,
    F7 = 64,
    F8 = 65,
    F9 = 66,
    F10 = 67,
    F11 = 68,
    F12 = 69,
    F13 = 104,
    F14 = 105,
    F15 = 106,
    F16 = 107,
    F17 = 108,
    F18 = 109,
    F19 = 110,
    F20 = 111,
    F21 = 112,
    F22 = 113,
    F23 = 114,
    F24 = 115,

    // Arrow keys
    Up = 82,
    Down = 81,
    Left = 80,
    Right = 79,

    // Modifier keys
    LeftCtrl = 224,
    LeftShift = 225,
    LeftAlt = 226,
    LeftGui = 227,
    RightCtrl = 228,
    RightShift = 229,
    RightAlt = 230,
    RightGui = 231,

    // Gamepad keys
    DPadUp = 0b0100_0000_0000_0000_0000_0000_0000_0001,
    DPadDown = 0b0100_0000_0000_0000_0000_0000_0000_0010,
    DPadLeft = 0b0100_0000_0000_0000_0000_0000_0000_0011,
    DPadRight = 0b0100_0000_0000_0000_0000_0000_0000_0100,
    Start = 0b0100_0000_0000_0000_0000_0000_0000_0101,
    Back = 0b0100_0000_0000_0000_0000_0000_0000_0110,
    LeftThumb = 0b0100_0000_0000_0000_0000_0000_0000_0111,
    RightThumb = 0b0100_0000_0000_0000_0000_0000_0000_1000,
    LeftShoulder = 0b0100_0000_0000_0000_0000_0000_0000_1001,
    RightShoulder = 0b0100_0000_0000_0000_0000_0000_0000_1010,
    North = 0b0100_0000_0000_0000_0000_0000_0000_1011,
    East = 0b0100_0000_0000_0000_0000_0000_0000_1100,
    South = 0b0100_0000_0000_0000_0000_0000_0000_1101,
    West = 0b0100_0000_0000_0000_0000_0000_0000_1110,
    LeftTrigger = 0b0100_0000_0000_0000_0000_0000_0000_1111,
    RightTrigger = 0b0100_0000_0000_0000_0000_0000_0001_0000,
    LeftJoystick = 0b0100_0000_0000_0000_0000_0000_0001_0001,
    RightJoystick = 0b0100_0000_0000_0000_0000_0000_0001_0010,

    // Unknown key
    Unknown = 0,
}

#[allow(dead_code)]
impl Key {
    /// Converts a key code to a `Key` enum
    /// Returns `Key::Unknown` if the key code is not recognized
    ///
    /// # Arguments
    ///
    /// * `code` - The key code
    ///
    /// # Returns
    ///
    /// The `Key` enum corresponding to the key code
    pub fn from_code(code: u8) -> Option<Self> {
        match code {
            4 => Some(Self::A),
            5 => Some(Self::B),
            6 => Some(Self::C),
            7 => Some(Self::D),
            8 => Some(Self::E),
            9 => Some(Self::F),
            10 => Some(Self::G),
            11 => Some(Self::H),
            12 => Some(Self::I),
            13 => Some(Self::J),
            14 => Some(Self::K),
            15 => Some(Self::L),
            16 => Some(Self::M),
            17 => Some(Self::N),
            18 => Some(Self::O),
            19 => Some(Self::P),
            20 => Some(Self::Q),
            21 => Some(Self::R),
            22 => Some(Self::S),
            23 => Some(Self::T),
            24 => Some(Self::U),
            25 => Some(Self::V),
            26 => Some(Self::W),
            27 => Some(Self::X),
            28 => Some(Self::Y),
            29 => Some(Self::Z),
            30 => Some(Self::N1),
            31 => Some(Self::N2),
            32 => Some(Self::N3),
            33 => Some(Self::N4),
            34 => Some(Self::N5),
            35 => Some(Self::N6),
            36 => Some(Self::N7),
            37 => Some(Self::N8),
            38 => Some(Self::N9),
            39 => Some(Self::N0),

            40 => Some(Self::Enter),
            42 => Some(Self::Backspace),
            43 => Some(Self::Tab),
            44 => Some(Self::Space),
            57 => Some(Self::CapsLock),

            45 => Some(Self::Hyphen),
            46 => Some(Self::Equal),
            47 => Some(Self::LeftBracket),
            48 => Some(Self::RightBracket),
            49 => Some(Self::Backslash),
            51 => Some(Self::Semicolon),
            52 => Some(Self::Apostrophe),
            53 => Some(Self::Grave),
            54 => Some(Self::Comma),
            55 => Some(Self::Period),
            56 => Some(Self::Slash),

            58 => Some(Self::F1),
            59 => Some(Self::F2),
            60 => Some(Self::F3),
            61 => Some(Self::F4),
            62 => Some(Self::F5),
            63 => Some(Self::F6),
            64 => Some(Self::F7),
            65 => Some(Self::F8),
            66 => Some(Self::F9),
            67 => Some(Self::F10),
            68 => Some(Self::F11),
            69 => Some(Self::F12),
            70 => Some(Self::F13),
            71 => Some(Self::F14),
            72 => Some(Self::F15),
            73 => Some(Self::F16),
            74 => Some(Self::F17),
            75 => Some(Self::F18),
            76 => Some(Self::F19),
            77 => Some(Self::F20),
            78 => Some(Self::F21),
            79 => Some(Self::F22),
            80 => Some(Self::F23),
            81 => Some(Self::F24),

            82 => Some(Self::Up),
            83 => Some(Self::Down),
            84 => Some(Self::Left),
            85 => Some(Self::Right),

            224 => Some(Self::LeftCtrl),
            225 => Some(Self::LeftShift),
            226 => Some(Self::LeftAlt),
            227 => Some(Self::LeftGui),
            228 => Some(Self::RightCtrl),
            229 => Some(Self::RightShift),
            230 => Some(Self::RightAlt),
            231 => Some(Self::RightGui),

            _ => Some(Self::Unknown),
        }
    }
}

#[cfg(unix)]
use {
    core_foundation::{
        base::{kCFAllocatorDefault, CFRelease, CFTypeRef},
        runloop::{kCFRunLoopDefaultMode, CFRunLoop},
    },
    io_kit_sys::{
        hid::{
            base::IOHIDValueRef,
            element::{IOHIDElementGetUsage, IOHIDElementGetUsagePage},
            keys::{kIOHIDDeviceKey, kIOHIDOptionsTypeNone},
            manager::{
                kIOHIDManagerOptionNone, IOHIDManagerCreate, IOHIDManagerOpen,
                IOHIDManagerRegisterInputValueCallback, IOHIDManagerScheduleWithRunLoop,
                IOHIDManagerSetDeviceMatching,
            },
            value::{IOHIDValueGetElement, IOHIDValueGetIntegerValue},
        },
        ret::kIOReturnSuccess,
        IOServiceMatching,
    },
};

#[cfg(unix)]
const USAGE_PAGE_KEYBOARD: u32 = 0x07;

/// Input callback for input thread
#[cfg(unix)]
extern "C" fn input_value_callback(
    _context: *mut std::ffi::c_void,
    _result: i32,
    _sender: *mut std::ffi::c_void,
    value: IOHIDValueRef,
) {
    unsafe {
        // Get the element associated with the value
        let element = IOHIDValueGetElement(value);

        // Get the usage page and usage
        let usage_page = IOHIDElementGetUsagePage(element);
        let usage = IOHIDElementGetUsage(element);

        // Get the value (pressed or released)
        let pressed = IOHIDValueGetIntegerValue(value);

        if usage_page == USAGE_PAGE_KEYBOARD {
            let key = Key::from_code(usage as u8);

            match key {
                Some(key) => match key {
                    Key::Unknown => {}
                    key => {
                        if pressed == 0 {
                            KEYS.get().unwrap().lock().unwrap().retain(|k| k != &key);

                            match CALLBACK.get().unwrap().lock().unwrap().as_ref() {
                                Some(func) => func(key, false),
                                _ => {}
                            }
                        } else {
                            KEYS.get().unwrap().lock().unwrap().push(key.clone());

                            match CALLBACK.get().unwrap().lock().unwrap().as_ref() {
                                Some(func) => func(key, true),
                                _ => {}
                            }
                        }
                    }
                },
                _ => {}
            }
        }
    }
}

/// Get the current set of keys being pressed
pub fn keys() -> Vec<Key> {
    KEYS.get().unwrap().lock().unwrap().clone()
}

/// Set the keyboard callback
pub fn set_callback(callback: Option<Box<dyn Fn(Key, bool) + Send>>) {
    *CALLBACK.get().unwrap().lock().unwrap() = callback;
}

/// Run the keyboard thread for listening to inputs
pub fn run() -> Result<(), &'static str> {
    KEYS.get_or_init(|| Mutex::new(Vec::new()));
    CALLBACK.get_or_init(|| Mutex::new(None));

    #[cfg(unix)]
    unsafe {
        std::thread::spawn(move || {
            // Create the HID Manager
            let manager = IOHIDManagerCreate(kCFAllocatorDefault, kIOHIDManagerOptionNone);
            if manager.is_null() {
                return Err("Failed to create IOHIDManager.");
            }

            // Set device matching to keyboards
            let matching_dict = IOServiceMatching(kIOHIDDeviceKey);
            IOHIDManagerSetDeviceMatching(manager, matching_dict);

            // Register the input callback
            IOHIDManagerRegisterInputValueCallback(manager, input_value_callback, ptr::null_mut());

            // Schedule with the run loop
            IOHIDManagerScheduleWithRunLoop(
                manager,
                CFRunLoop::get_current().as_concrete_TypeRef(),
                kCFRunLoopDefaultMode,
            );

            // Open the HID Manager
            let result = IOHIDManagerOpen(manager, kIOHIDOptionsTypeNone);
            if result != kIOReturnSuccess {
                eprintln!("Failed to open IOHIDManager. Error code: {}", result);
                CFRelease(manager as CFTypeRef);
                return Err("Failed to open IOHIDManager");
            }

            println!("Listening for keyboard events. Press keys to see output.");

            // Start the run loop
            CFRunLoop::run_current();

            // Clean up
            CFRelease(manager as CFTypeRef);

            Ok(())
        });
    }

    Ok(())
}
