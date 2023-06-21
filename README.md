# kodiak-sets

[![GitHub Top Language]][lang]
[![Static unsafe]][unsafe]
[![crates.io License]][license-mit]
[![GitHub License]][license-apache]


[![GitHub Latest Release]][github-releases]
[![GitHub Commits]][github-commits]


[![Code Coverage]][codecov]
[![GitHub Build Status]][github-actions-cargo-test]
[![Static docs coverage]][docs]
[![docs.rs]][docs]
[![Libraries.io Dep Status]][libraries]


[![GitHub Security Schedule]][github-actions-cargo-audit-on-schedule]
[![GitHub Security Push]][github-actions-cargo-audit-on-push]


[![GitHub Open Issues]][github-issues]
[![GitHub Closed Issues]][github-issues]


[![crates.io Latest]][crates]
[![crates.io Recent]][crates]

[Code Coverage]: https://img.shields.io/codecov/c/github/polarlabs/kodiak-sets?label=code%20coverage&logo=codecov&logoColor=ffffff&style=flat-square 
[codecov]: https://codecov.io/github/polarlabs/kodiak-sets

[crates.io Recent]: https://img.shields.io/crates/dr/kodiak-sets?logo=docs.rs&color=67001f&style=flat-square
[crates.io Latest]: https://img.shields.io/crates/v/kodiak-sets?label=latest&logo=docs.rs&style=flat-square
[crates]: https://crates.io/crates/kodiak-sets

[crates.io License]: https://img.shields.io/crates/l/kodiak-sets?logo=docs.rs&color=007ec6&style=flat-square
[GitHub License]: https://img.shields.io/github/license/polarlabs/kodiak-sets?logo=github&color=007ec6&style=flat-square
[license-mit]: https://choosealicense.com/licenses/mit/
[license-apache]: https://choosealicense.com/licenses/apache-2.0/

[Static docs coverage]: https://img.shields.io/badge/docs%20coverage-100%25-success.svg?logo=rust&logoColor=ffffff&style=flat-square
[docs.rs]: https://img.shields.io/docsrs/kodiak-sets?logo=docs.rs&style=flat-square
[docs]: https://docs.rs/kodiak-sets

[GitHub Build Status]: https://img.shields.io/github/actions/workflow/status/polarlabs/kodiak-sets/cargo-test.yml?branch=main&logo=github&label=tests&style=flat-square
[github-actions-cargo-test]: https://github.com/polarlabs/kodiak-sets/actions/workflows/cargo-test.yml

[GitHub Security Schedule]: https://img.shields.io/github/actions/workflow/status/polarlabs/kodiak-sets/cargo-audit-on-schedule.yml?branch=main&logo=clockify&logoColor=ffffff&label=security%20audit%20(scheduled%20daily)&style=flat-square
[github-actions-cargo-audit-on-schedule]: https://github.com/polarlabs/kodiak-sets/actions/workflows/cargo-audit-on-schedule.yml

[GitHub Security Push]: https://img.shields.io/github/actions/workflow/status/polarlabs/kodiak-sets/cargo-audit-on-push.yml?branch=main&logo=github&label=security%20audit%20(on%20push)&style=flat-square
[github-actions-cargo-audit-on-push]: https://github.com/polarlabs/kodiak-sets/actions/workflows/cargo-audit-on-push.yml

[GitHub Top Language]: https://img.shields.io/github/languages/top/polarlabs/kodiak-sets?color=dea584&logo=rust&style=flat-square
[lang]: https://www.rust-lang.org/

[GitHub Latest Release]: https://img.shields.io/github/v/release/polarlabs/kodiak-sets?include_prereleases&sort=semver&logo=github&label=latest&style=flat-square
[github-releases]: https://github.com/polarlabs/kodiak-sets/releases

[GitHub Commits]: https://img.shields.io/github/commits-since/polarlabs/kodiak-sets/latest?include_prereleases&sort=semver&logo=github&style=flat-square
[github-commits]: https://github.com/polarlabs/kodiak-sets/commits

[GitHub Open Issues]: https://img.shields.io/github/issues-raw/polarlabs/kodiak-sets?logo=github&style=flat-square
[GitHub Closed Issues]: https://img.shields.io/github/issues-closed-raw/polarlabs/kodiak-sets?logo=github&style=flat-square
[github-issues]: https://github.com/polarlabs/kodiak-sets/issues

[Libraries.io Dep Status]: https://img.shields.io/librariesio/github/polarlabs/kodiak-sets?logo=libraries.io&logoColor=ffffff&style=flat-square
[libraries]: https://libraries.io/cargo/kodiak-sets

[Static unsafe]: https://img.shields.io/badge/unsafe-forbidden-success.svg?logo=rust&logoColor=ffffff&style=flat-square
[unsafe]: (https://github.com/rust-secure-code/safety-dance/)

Get things organized with these powerful, yet easy to use sets. The first set implemented in 0.1.0 is `Sequence`. 
From a generic perspective, sequences are ordered sets of elements, with each element at a unique position. `Sequence` allows 
to add and remove elements at any position, virtually infinitely.

At first, a `Vec<T>` looks most appropriate for implementing a sequence. However, when thinking about persisting the sequence, 
there are some challenges in practice:

- How to store a sequence in a database and retrieve it efficiently?
- How to add elements to a sequence at an arbitrary position efficiently, e.g. without having to change the position of all following elements?
- How to change the position of a single element without the need to change other elements' positions?
- How to avoid the need to rebalance the sequence, i.e. having to rewrite the position of all elements?
- How to scale the approach to hundreds of thousands of operations on the sequence, such as adding, moving and removing elements?

The objective of `kodiak-sets` implementation of `Sequence` is to solve these challenges. In 0.1.0 it only offers support for `Sequence`. 
In the future, other types of sets might be added.

We use the "mediant fraction" algorithm to define a position within a `Sequence`. The mediant fraction is a mathematical concept 
that has been known and used for a long time. It is a method of finding a fraction that lies between two given fractions by 
taking the sum of the numerators and denominators separately.

The crate is a building block of the Kodiak project, thus the naming of the crate. Kodiak supports sequences of entities at a very large scale. 
However, the functionality provided by `kodiak-sets` is useful on its own and might be of interest for other projects as well. 
That's why we deliver it as a separate crate. So, feel free to use it. If you consider using `kodiak-sets` in your project but are missing 
something or have any other concerns, don't hesitate to file an issue on GitHub.

We are looking forward to your feedback.

---

You may be looking for:

- [API documentation](https://docs.rs/kodiak-sets/)
- [Release notes](https://github.com/polarlabs/kodiak-sets/releases)

---

# Impressions

todo: show two examples of sequences supported by `kodiak-sets`.

Provide additional examples in EXAMPLES.md and link to it.

Demonstrate capabilities!

# Known issues / limitations
- 🏗️ Version 0.1.0 does not yet power other projects, so API has not yet proven it's power in practice.
- 🚧 Code is fully covered by unit tests, however, some integration tests are still missing.
- Documentation has still _some_ room for improvement.
- 🐧 Version 0.1.x is developed and tested on Linux only.

# Roadmap and future considerations

## Version 0.4.0 (planned)
- Example showing how to use this library with MySQL.

## Version 0.3.0 (planned)
- Example showing how to use this library with PostgreSQL.

## Version 0.2.0 (planned)
- Support iterators for `Sequence`
- Support the trait `Clone`

## Version 0.1.0 (delivered)
- Initial release.
- Examples to demonstrate the power of fraction based positioning.
- Examples showing how to use this library with SQlite.

# Additional resources

- Homepage polarlabs: [polarlabs.io](https://www.polarlabs.io)
- Crate: [crates.io/kodiak-sets](https://crates.io/crates/kodiak-sets)
- API documentation: [docs.rs/kodiak-sets](https://docs.rs/kodiak-sets/)

# Contributing

See [CONTRIBUTING](CONTRIBUTING.md) for more details.

# Appendix

## Cargo Geiger Safety Report

```
Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols: 
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Dependency

0/0        0/0          0/0    0/0     0/0      :) kodiak-sets 0.1.0
0/0        0/0          0/0    0/0     0/0      ?  └── num-integer 0.1.45
0/0        6/12         0/0    0/0     0/0      !      └── num-traits 0.2.15

0/0        6/12         0/0    0/0     0/0
```

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or https://opensource.org/licenses/Apache-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Links

[https://www.figma.com/blog/realtime-editing-of-ordered-sequences/]

[https://crates.io/crates/fractional_index]

[https://cs.stackexchange.com/questions/14708/maintaining-an-efficient-ordering-where-you-can-insert-elements-in-between-any]

[https://ccssmathanswers.com/inserting-a-fraction-between-two-given-fractions/]
