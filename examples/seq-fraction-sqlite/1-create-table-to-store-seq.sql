--
-- An example table to store a Sequence
--
CREATE TABLE IF NOT EXISTS sequence (
    id INTEGER NOT NULL PRIMARY KEY,
    numerator INTEGER NOT NULL,
    denominator INTEGER NOT NULL,
    pos TEXT GENERATED ALWAYS AS (numerator / CAST(denominator AS REAL)) STORED,
    payload TEXT
) STRICT;

CREATE INDEX IF NOT EXISTS
idx_sequence_pos
ON sequence (
    pos
);
