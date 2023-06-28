# Examples

### [seq-avg](seq-avg)

Demonstrates how fast 32- and 64-bit floats are running out of accuracy when defining an element's position in a sequence is based on 
the average of the predecessor's and postXXX's positions. In this context, "fast" is defined as how often an element could be placed between two existing 
elements in a worst case scenario, i.e. repetitive insert at the same position.

Spoiler: f32 allows 23 insertions, f64 allows 52 insertions.

### [seq-fraction](seq-fraction)

Uses `kodiak-sets` to position an element in a sequence, with position defined as a fraction. The fraction is calculated 
by separately adding the numerator and denominator of the predecessor's and postXXX's positions. For example the position of an element between the 
elements at positions 1/1 and 2/1, is 3/2.

Spoiler: kodiak-sets::sequence allows 79.999.979 insertions.

### [seq-fraction-psql](seq-fraction-psql)

Planned for a later version. Have a look at the roadmap.

### [seq-fraction-sqlite](seq-fraction-sqlite)

Interested in how to use kodiak-sets::sequence with sqlite? Have a look at this example.

### [seq-using-serde-derive](seq-using-serde-derive)

Shows how to leverage serde's `#[derive(Serialize, Deserialize)]` for a sequence.
