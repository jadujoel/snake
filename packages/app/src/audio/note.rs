use std::ops::Add;
use std::ops::Sub;

const NUM: usize = 108;

pub const FREQUENCIES: [f32; NUM] = [
    16.35, 17.32, 18.35, 19.45, 20.60, 21.83, 23.12, 24.50, 25.96, 27.50, 29.14, 30.87, 32.70,
    34.65, 36.71, 38.89, 41.20, 43.65, 46.25, 49.00, 51.91, 55.00, 58.27, 61.74, 65.41, 69.30,
    73.42, 77.78, 82.41, 87.31, 92.50, 98.00, 103.83, 110.00, 116.54, 123.47, 130.81, 138.59,
    146.83, 155.56, 164.81, 174.61, 185.00, 196.00, 207.65, 220.00, 233.08, 246.94, 261.63, 277.18,
    293.66, 311.13, 329.63, 349.23, 369.99, 392.00, 415.30, 440.00, 466.16, 493.88, 523.25, 554.37,
    587.33, 622.25, 659.25, 698.46, 739.99, 783.99, 830.61, 880.00, 932.33, 987.77, 1046.50,
    1108.73, 1174.66, 1244.51, 1318.51, 1396.91, 1479.98, 1567.98, 1661.22, 1760.00, 1864.66,
    1975.53, 2093.00, 2217.46, 2349.32, 2489.02, 2637.02, 2793.83, 2959.96, 3135.96, 3322.44,
    3520.00, 3729.31, 3951.07, 4186.01, 4434.92, 4698.63, 4978.03, 5274.04, 5587.65, 5919.91,
    6271.93, 6644.88, 7040.00, 7458.62, 7902.13,
];

const STRINGS: [&str; NUM] = [
    "C0", "C0#", "D0", "D0#", "E0", "F0", "F0#", "G0", "G0#", "A0", "A0#", "B0", "C1", "C1#", "D1",
    "D1#", "E1", "F1", "F1#", "G1", "G1#", "A1", "A1#", "B1", "C2", "C2#", "D2", "D2#", "E2", "F2",
    "F2#", "G2", "G2#", "A2", "A2#", "B2", "C3", "C3#", "D3", "D3#", "E3", "F3", "F3#", "G3",
    "G3#", "A3", "A3#", "B3", "C4", "C4#", "D4", "D4#", "E4", "F4", "F4#", "G4", "G4#", "A4",
    "A4#", "B4", "C5", "C5#", "D5", "D5#", "E5", "F5", "F5#", "G5", "G5#", "A5", "A5#", "B5", "C6",
    "C6#", "D6", "D6#", "E6", "F6", "F6#", "G6", "G6#", "A6", "A6#", "B6", "C7", "C7#", "D7",
    "D7#", "E7", "F7", "F7#", "G7", "G7#", "A7", "A7#", "B7", "C8", "C8#", "D8", "D8#", "E8", "F8",
    "F8#", "G8", "G8#", "A8", "A8#", "B8",
];

#[derive(Clone, Copy)]
#[allow(dead_code, unused_variables, unused_assignments)]
pub enum Note {
    C0,
    C0Sharp,
    D0,
    D0Sharp,
    E0,
    F0,
    F0Sharp,
    G0,
    G0Sharp,
    A0,
    A0Sharp,
    B0,
    C1,
    C1Sharp,
    D1,
    D1Sharp,
    E1,
    F1,
    F1Sharp,
    G1,
    G1Sharp,
    A1,
    A1Sharp,
    B1,
    C2,
    C2Sharp,
    D2,
    D2Sharp,
    E2,
    F2,
    F2Sharp,
    G2,
    G2Sharp,
    A2,
    A2Sharp,
    B2,
    C3,
    C3Sharp,
    D3,
    D3Sharp,
    E3,
    F3,
    F3Sharp,
    G3,
    G3Sharp,
    A3,
    A3Sharp,
    B3,
    C4,
    C4Sharp,
    D4,
    D4Sharp,
    E4,
    F4,
    F4Sharp,
    G4,
    G4Sharp,
    A4,
    A4Sharp,
    B4,
    C5,
    C5Sharp,
    D5,
    D5Sharp,
    E5,
    F5,
    F5Sharp,
    G5,
    G5Sharp,
    A5,
    A5Sharp,
    B5,
    C6,
    C6Sharp,
    D6,
    D6Sharp,
    E6,
    F6,
    F6Sharp,
    G6,
    G6Sharp,
    A6,
    A6Sharp,
    B6,
    C7,
    C7Sharp,
    D7,
    D7Sharp,
    E7,
    F7,
    F7Sharp,
    G7,
    G7Sharp,
    A7,
    A7Sharp,
    B7,
    C8,
    C8Sharp,
    D8,
    D8Sharp,
    E8,
    F8,
    F8Sharp,
    G8,
    G8Sharp,
    A8,
    A8Sharp,
    B8,
}

impl Add<Note> for Note {
    type Output = Note;

    fn add(self, rhs: Note) -> Self::Output {
        let note = self as u8 + rhs as u8;
        if note > Note::B8 as u8 {
            Note::B8
        } else {
            unsafe { std::mem::transmute(note) }
        }
    }
}

impl Sub<Note> for Note {
    type Output = Note;

    fn sub(self, rhs: Note) -> Self::Output {
        let note = self as u8 - rhs as u8;
        if note < Note::C0 as u8 {
            Note::C0
        } else {
            unsafe { std::mem::transmute(note) }
        }
    }
}

impl std::ops::Rem<Note> for Note {
    type Output = Note;
    fn rem(self, rhs: Note) -> Self::Output {
        let note = self as u8 % rhs as u8;
        unsafe { std::mem::transmute(note) }
    }
}


impl ToString for Note {
    fn to_string(&self) -> String {
        STRINGS[*self as usize].to_string()
    }
}

impl Note {
    pub fn to_frequency(self) -> f32 {
        FREQUENCIES[self as usize]
    }
}

#[allow(dead_code, unused_variables, unused_assignments)]
pub const NOTES: [Note; NUM] = [
    Note::C0,
    Note::C0Sharp,
    Note::D0,
    Note::D0Sharp,
    Note::E0,
    Note::F0,
    Note::F0Sharp,
    Note::G0,
    Note::G0Sharp,
    Note::A0,
    Note::A0Sharp,
    Note::B0,
    Note::C1,
    Note::C1Sharp,
    Note::D1,
    Note::D1Sharp,
    Note::E1,
    Note::F1,
    Note::F1Sharp,
    Note::G1,
    Note::G1Sharp,
    Note::A1,
    Note::A1Sharp,
    Note::B1,
    Note::C2,
    Note::C2Sharp,
    Note::D2,
    Note::D2Sharp,
    Note::E2,
    Note::F2,
    Note::F2Sharp,
    Note::G2,
    Note::G2Sharp,
    Note::A2,
    Note::A2Sharp,
    Note::B2,
    Note::C3,
    Note::C3Sharp,
    Note::D3,
    Note::D3Sharp,
    Note::E3,
    Note::F3,
    Note::F3Sharp,
    Note::G3,
    Note::G3Sharp,
    Note::A3,
    Note::A3Sharp,
    Note::B3,
    Note::C4,
    Note::C4Sharp,
    Note::D4,
    Note::D4Sharp,
    Note::E4,
    Note::F4,
    Note::F4Sharp,
    Note::G4,
    Note::G4Sharp,
    Note::A4,
    Note::A4Sharp,
    Note::B4,
    Note::C5,
    Note::C5Sharp,
    Note::D5,
    Note::D5Sharp,
    Note::E5,
    Note::F5,
    Note::F5Sharp,
    Note::G5,
    Note::G5Sharp,
    Note::A5,
    Note::A5Sharp,
    Note::B5,
    Note::C6,
    Note::C6Sharp,
    Note::D6,
    Note::D6Sharp,
    Note::E6,
    Note::F6,
    Note::F6Sharp,
    Note::G6,
    Note::G6Sharp,
    Note::A6,
    Note::A6Sharp,
    Note::B6,
    Note::C7,
    Note::C7Sharp,
    Note::D7,
    Note::D7Sharp,
    Note::E7,
    Note::F7,
    Note::F7Sharp,
    Note::G7,
    Note::G7Sharp,
    Note::A7,
    Note::A7Sharp,
    Note::B7,
    Note::C8,
    Note::C8Sharp,
    Note::D8,
    Note::D8Sharp,
    Note::E8,
    Note::F8,
    Note::F8Sharp,
    Note::G8,
    Note::G8Sharp,
    Note::A8,
    Note::A8Sharp,
    Note::B8,
];
