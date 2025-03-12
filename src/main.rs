use encoding::EncodingTable;
use probability::ProbabilityTable;
use text::HELLO as TEST;

mod encoding;
mod probability;
mod text;

fn main() {
    let table = ProbabilityTable::new(TEST.as_bytes());
    println!("{}", table.as_markdown());
    let encoder = EncodingTable::from(&table);
    let encoded = encoder.encode(TEST.as_bytes());
    println!("encoded: {:?}", encoded);
    let decoded = encoder.decode(encoded);
    println!("decoded: {:?}", decoded);
}
