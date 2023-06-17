# seq-fraction-sqlite

This examples shows how you can leverage `kodiak-sets::Sequence` with SQLite.

## How To

First create the database with the provided bash script. If you are interested in how the database is created have a look 
at the .sql file.

Run the bash script.

```
. ./init-db.sh
```

Then, run cargo to build and run the example.

```
cargo run
```

Note: if you want to compare performance measures, remember to run `cargo build` first.

```
sqlite3 test.db
sqlite> .headers on
sqlite> SELECT * FROM sequence ORDER BY pos DESC LIMIT 5;
id|numerator|denominator|pos|payload
100003|3|1|3.0|C
100002|2|1|2.0|B
100001|200001|100001|1.9999900001|A
100000|199999|100000|1.99999|A
99999|199997|99999|1.9999899999|A
sqlite> .exit
```

## Performance considerations

Using SQlite with sqlx without setting any pragmas has a huge performance penalty especially for INSERTS.

The following test results show the impact of pragmas based on this example.

### Without any pragmas and n = 1000.

```
real    0m7.197s
user    0m1.356s
sys     0m1.043s
```

### With journal_mode set to OFF and n = 1000.

```
"PRAGMA journal_mode = OFF;"

real    0m2.904s
user    0m1.261s
sys     0m0.372s
```

### In addition: synchronous OFF and n = 1000.

```
"PRAGMA journal_mode = OFF;"
"PRAGMA synchronous = OFF;"

real    0m0.517s
user    0m0.550s
sys     0m0.117s
```

### Setting cache_size to 1 million in addition and n = 1000.

```
"PRAGMA journal_mode = OFF;"
"PRAGMA synchronous = OFF;"
"PRAGMA cache_size = 1000000;"

real    0m0.508s
user    0m0.538s
sys     0m0.118s
```

### Holding temp_store in memory and n = 1000.

```
"PRAGMA journal_mode = OFF;"
"PRAGMA synchronous = OFF;"
"PRAGMA cache_size = 1000000;"
"PRAGMA temp_store = MEMORY;"

real    0m0.496s
user    0m0.548s
sys     0m0.108s
```

### Using state of the art settings, ensuring that SQLite is resilient to failures of the application, the operating system and the machine and n = 1000.

```
"PRAGMA journal_mode = WAL;"
"PRAGMA synchronous = NORMAL;"
"PRAGMA cache_size = 1000000;"
"PRAGMA temp_store = MEMORY;"
"PRAGMA locking_mode = NORMAL;"

real    0m0.530s
user    0m0.502s
sys     0m0.137s
```

And now with 100.000 inserts.

```
real    0m28.320s
user    0m34.028s
sys     0m6.134s
```

And finally with 1.000.000 inserts.

```
real    5m34.549s
user    6m36.576s
sys     1m14.094s
```
