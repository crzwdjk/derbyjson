# DerbyJSON: a standard for roller derby data interchange

The DerbyJSON standard is a standard for encoding all kinds of information
about roller derby in a uniform format. It can be used to store information
about teams and rosters, as well as to store game stats in a format that
is not Excel. It is, as far as I know, maintained by WFTDA. Version 0.2
of the standard is documented at 
https://github.com/WFTDA/derbystatter/raw/master/docs/DerbyJSONSpec.pdf

This package provides a set of Rust structs and enums that are used in
conjunction with [serde](https://serde.rs) to represent parsed DerbyJSON
structures in Rust. This representation aims to parse all DerbyJSON that
is compliant with the standard, and produce standards-compliant DerbyJSON.

### Compatibility note

The DerbyJSON spec is only at version 0.2, and is somewhat incomplete.
There is a high chance that the spec will change in the future. There are
also currently, to the best of my knowledge, no actual implementations of
the spec, thus there is no good way to test interoperability with anything
else.



