use kodiak_sets::Sequence;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Person {
    name: String
}

fn main() {
    let mut seq: Sequence<Person> = Sequence::new();
    seq.push(Person { name: "Anton".to_string() });
    seq.push(Person { name: "Ben".to_string() });
    seq.push(Person { name: "Christoph".to_string() });

    // Serialize it to a JSON string.
    let persons = serde_json::to_string(&seq).unwrap();

    // Print
    println!("{}", persons);
}
