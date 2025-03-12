use std::collections::HashMap;

use crate::encoding::{Channel, Probability};

#[derive(Default)]
pub struct ProbabilityTable {
    map: HashMap<u8, usize>,
    num_elements: usize,
}

impl ProbabilityTable {
    pub fn new(input: &[u8]) -> ProbabilityTable {
        let mut result = ProbabilityTable::default();

        for char in input {
            let entry = result.map.entry(*char).or_default();
            *entry += 1;
        }
        result.num_elements = input.len();

        result
    }

    pub fn as_markdown(&self) -> String {
        let mut ordered = vec![];
        ordered.extend(self.map.iter());
        ordered.sort_by_key(|(byte, _count)| **byte);

        let mut lines = vec![];
        lines.push("| Byte | Char | Count |".to_string());
        lines.push("| ---- | ---- | ----- |".to_string());
        for (byte, count) in ordered {
            let ch = if *byte == 10 {
                "\\n".to_string()
            } else {
                String::from(char::from_u32(*byte as u32).unwrap())
            };

            lines.push(format!("|  {:>3} | {:>4} | {:>5} |", byte, ch, count));
        }
        lines.join("\n")
    }

    pub fn as_channels(&self) -> Vec<Channel> {
        let mut ordered = vec![];
        ordered.extend(self.map.iter());
        ordered.sort_by_key(|(byte, count)| -(**count as i32));

        ordered
            .iter()
            .map(|(byte, count)| Channel {
                byte: **byte,
                probability: Probability {
                    numerator: **count,
                    denominator: self.num_elements,
                },
            })
            .collect::<Vec<Channel>>()
    }
}
