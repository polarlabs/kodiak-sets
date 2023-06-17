use kodiak_sets::Sequence;

fn main() {
    let mut seq: Sequence<String> = Sequence::new();
    seq.push("A".to_string());
    seq.push("B".to_string());
    seq.push("C".to_string());

    let mut n: usize = 0;
    // With 79_999_980 insertions, the float representation of the positions of the two elements added last are no longer distinguishable.
    while n < 79_999_980 {
        n += 1;
        seq.insert(n, "A".to_string());
    }

    println!("Element {} at position: {:?}", n-1, seq.position_from(n-1));
    println!("Element {} at position: {:?}", n, seq.position_from(n));
    println!("Element {} at position: {:?}", n+1, seq.position_from(n+1));
}
