#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pitch {
    letter: usize,
    accidental: i8,
}

const NATURAL: [i8; 7] = [0, 2, 4, 5, 7, 9, 11];
const LETTERS: [&str; 7] = ["C", "D", "E", "F", "G", "A", "B"];

impl Pitch {
    pub fn pc(self) -> u8 {
        (NATURAL[self.letter] + self.accidental).rem_euclid(12) as u8
    }

    pub fn transpose(self, semitones: u8, degrees: usize) -> Self {
        let letter = (self.letter + degrees) % 7;
        let target = (self.pc() + semitones) % 12;
        let accidental = (target as i8 - NATURAL[letter] + 6).rem_euclid(12) - 6;
        Self { letter, accidental }
    }

    pub fn name(self) -> String {
        let accidental = match self.accidental {
            -2 => "bb",
            -1 => "b",
            0 => "",
            1 => "#",
            2 => "##",
            _ => unreachable!("unsupported accidental"),
        };
        format!("{}{accidental}", LETTERS[self.letter])
    }

    #[allow(dead_code)]
    pub fn at(self, midi: u8) -> String {
        let octave =
            (i16::from(midi) - i16::from(self.accidental) - i16::from(NATURAL[self.letter])) / 12
                - 1;
        format!("{}{octave}", self.name())
    }
}

pub fn root(pc: u8, intervals: &[u8], degrees: &[usize]) -> Pitch {
    (0..7)
        .flat_map(|letter| (-1..=1).map(move |accidental| Pitch { letter, accidental }))
        .filter(|pitch| pitch.pc() == pc)
        .min_by_key(|&pitch| {
            intervals
                .iter()
                .zip(degrees)
                .map(|(&interval, &degree)| {
                    let accidental = pitch.transpose(interval, degree).accidental;
                    i16::from(accidental).pow(2)
                })
                .sum::<i16>()
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chord_spelling_preserves_letters_and_sounding_pitches() {
        let c = root(0, &[0, 3, 7], &[0, 2, 4]);
        assert_eq!(c.transpose(3, 2).name(), "Eb");
        let db = root(1, &[0, 4, 7], &[0, 2, 4]);
        assert_eq!(db.name(), "Db");
        assert_eq!(db.transpose(4, 2).name(), "F");
        assert_eq!(db.transpose(7, 4).name(), "Ab");
        for pc in 0..12 {
            let tonic = root(pc, &[0, 2, 4, 5, 7, 9, 11], &[0, 1, 2, 3, 4, 5, 6]);
            for (degree, interval) in [0, 2, 4, 5, 7, 9, 11].into_iter().enumerate() {
                let tone = tonic.transpose(interval, degree);
                assert_eq!(tone.pc(), (pc + interval) % 12);
                assert_eq!(tone.letter, (tonic.letter + degree) % 7);
            }
        }
    }

    #[test]
    fn octaves_follow_written_notes() {
        let b_sharp = Pitch {
            letter: 6,
            accidental: 1,
        };
        let c_flat = Pitch {
            letter: 0,
            accidental: -1,
        };
        assert_eq!(b_sharp.at(60), "B#3");
        assert_eq!(c_flat.at(59), "Cb4");
    }
}
