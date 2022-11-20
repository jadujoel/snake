mod note;

fn note_to_frequency(note: &note::Note) -> f32 {
    let mut frequency = 440.0;
    let mut octave = 4;
    let mut note_index = 0;
    for (i, n) in note::NOTES.iter().enumerate() {
        if n == &note.note {
            note_index = i;
            break;
        }
    }
    while octave < note.octave {
        frequency *= 2.0;
        octave += 1;
    }
    while octave > note.octave {
        frequency /= 2.0;
        octave -= 1;
    }
    frequency *= note::FREQUENCIES[note_index];
    frequency
}
