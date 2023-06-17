use crate::Sequence;

pub fn setup_seq_empty() -> Sequence<String> {
    let seq: Sequence<String> = Sequence::new();
    seq
}

pub fn setup_seq_abc() -> Sequence<String> {
    let mut seq: Sequence<String> = Sequence::new();

    seq.push("A".to_string());
    seq.push("B".to_string());
    seq.push("C".to_string());

    seq
}
