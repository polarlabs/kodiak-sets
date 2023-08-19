Just a short checklist on merging a feature branch and publishing a release.

# Preflight check

No matter what you plan to do, make the preflight check.

* Run the benchmarks.
```
cargo criterion
```

* Check the test coverage. Make sure to use LLVM as engine.
```
cargo tarpaulin --engine llvm
```

* Run the tests.
```
cargo test
```

* Check the linter.
```
cargo clippy
```

* Fix formatting issues.
```
cargo fmt
```

# Merge a feature branch

* Fix formatting issues.

```
cargo fmt
```

* Test all examples.
* Update benchmarks.

```
cargo criterion
```

* Review benchmark results and add benchmarks to repo.

```
cp -a target/criterion benchmarks/VERSION
```

* Commit changes to feature branch.

```
git add .
git commit -m "feat!: ... ."
```

* Review CI results of to be merged feature branch.
* Merge feature branch.

```
git checkout main
git merge --squash <FEATURE_BRANCH>
```

* Change version in Cargo.toml
* Update Geiger report in README

```
cargo clean
cargo geiger --all-features --output-format GitHubMarkdown --update-readme
```

* Review README.md
* Update roadmap in README.md

```
git add .
git commit -m "feat!: ... ."
git push
```

# Publish a new version on crates.io

From the project root:

```
cargo doc
```

* Review docs locally

```
cargo publish --dry-run
cargo package --list
cargo publish
```

# Publish a new release on GitHub

On GitHub:

* Create a tag with pattern vx.y.z
* Release title: kodiak-sets-vx.y.z
* Write some release notes (mainly inspired by roadmap and git log)

# Links

[The Cargo Book - Publishing a new version of an existing crate](https://doc.rust-lang.org/cargo/reference/publishing.html#publishing-a-new-version-of-an-existing-crate)
