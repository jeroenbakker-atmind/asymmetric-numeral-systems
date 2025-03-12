use std::thread::current;

use crate::probability::ProbabilityTable;

pub struct Probability {
    pub numerator: usize,
    pub denominator: usize,
}

pub struct Channel {
    pub byte: u8,
    pub probability: Probability,
}

#[derive(Default, Clone)]
pub struct ChannelPattern {
    pub offsets: Vec<usize>,
}

#[derive(Default, Debug)]
pub struct Element {
    key: usize,
    parent_ref: usize,
    channel_refs: Vec<usize>,
    added_byte: u8,
}

#[derive(Default)]
pub struct EncodingTable {
    channels: Vec<Channel>,
    pattern: Vec<ChannelPattern>,
    pattern_size: usize,
}

impl EncodingTable {
    /// Call after filling channels to construct the pattern.
    fn init_pattern(&mut self) {
        let pattern_size = self.channels[0].probability.denominator;
        let mut channel_patterns = vec![ChannelPattern::default(); self.channels.len()];
        let mut channel_count = vec![0; self.channels.len()];
        let mut elements = 0;
        let mut cursor = 0;

        while elements < pattern_size {
            if channel_count[cursor] < self.channels[cursor].probability.numerator {
                channel_patterns[cursor].offsets.push(elements);
                elements += 1;
                channel_count[cursor] += 1;
            }
            cursor = (cursor + 1) % self.channels.len();
        }

        self.pattern = channel_patterns;
        self.pattern_size = pattern_size;
    }

    fn generate_element(&self, key: usize) -> Element {
        let mut result = Element::default();
        result.key = key;
        result.parent_ref = self.generate_parent_ref(key);
        result.added_byte = self.channels[self.determine_channel_index(key)].byte;
        for channel_index in 0..self.channels.len() {
            let generate_ref = self.generate_ref(key, channel_index);
            result.channel_refs.push(generate_ref);
        }

        result
    }
    fn generate_parent_ref(&self, key: usize) -> usize {
        let channel_index = self.determine_channel_index(key);
        let channel_pattern = &self.pattern[channel_index];
        for offset_index in 0..channel_pattern.offsets.len() {
            let offset = channel_pattern.offsets[offset_index];
            let base = key - offset;
            let d = base / self.pattern_size;
            if base % self.pattern_size == 0 {
                return d * channel_pattern.offsets.len() + offset_index;
            }
        }
        0
    }
    /// Determine the channel index of the current key.
    ///
    /// Which was the last chosen channel to get from the parent to the given key.
    /// go over each channel and check if it has an exact match.
    fn determine_channel_index(&self, key: usize) -> usize {
        if key == 0 {
            return 0;
        }
        for channel_index in 0..self.channels.len() {
            let channel_pattern = &self.pattern[channel_index];
            for offset in &channel_pattern.offsets {
                if key < *offset {
                    continue;
                }
                if (key - offset) % self.pattern_size == 0 {
                    return channel_index;
                }
            }
        }
        0
    }

    fn generate_ref(&self, key: usize, channel_index: usize) -> usize {
        let pattern_size = self.pattern_size;
        let pattern = &self.pattern[channel_index];
        let pattern_index = key / pattern.offsets.len();
        let offset_index = key % pattern.offsets.len();
        let start = pattern_index * pattern_size;
        let offset = pattern.offsets[offset_index];
        start + offset
    }

    pub fn encode(&self, bytes: &[u8]) -> usize {
        let mut current_key = 0;

        for byte in bytes {
            let mut channel_index = 0;
            for channel in &self.channels {
                if channel.byte == *byte {
                    break;
                }
                channel_index += 1;
            }
            let next_key = self.generate_ref(current_key, channel_index);
            println!(
                "{:?}, byte={}, channel_index={}, result={}",
                self.generate_element(current_key),
                byte,
                channel_index,
                next_key
            );
            current_key = next_key;
        }

        current_key
    }

    pub fn decode(&self, key: usize) -> Vec<u8> {
        let mut result = vec![];
        let mut current_key = key;
        while current_key != 0 {
            let channel_index = self.determine_channel_index(current_key);
            let byte = self.channels[channel_index].byte;
            result.push(byte);

            let parent = self.generate_parent_ref(current_key);
            current_key = parent;
        }
        result.reverse();
        result
    }
}

impl From<&ProbabilityTable> for EncodingTable {
    fn from(value: &ProbabilityTable) -> Self {
        let mut result = EncodingTable {
            channels: value.as_channels(),
            ..Default::default()
        };
        result.init_pattern();
        result
    }
}
