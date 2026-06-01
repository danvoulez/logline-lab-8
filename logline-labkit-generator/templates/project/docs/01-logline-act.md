# LogLine Act

A canonical Act has exactly nine semantic slots: `who`, `did`, `this`, `when`, `confirmed_by`, `if_ok`, `if_doubt`, `if_not`, and `status`.

The Act validator parses JSON and rejects malformed syntax before checking shape. It accepts ugly Candidates when all nine top-level slots are present and no extra top-level slots are present; it does not judge the internal contents of `this`, `confirmed_by`, `if_ok`, `if_doubt`, or `if_not`.

Top-level fields outside the nine slots are invalid in the canonical Act validator. `selected_branch` belongs outside the Act as metadata/projection/practice output, and `runtime_envelope` belongs outside the Act as provenance metadata or pack/profile practice.

`status` is one of the nine slots, but its vocabulary is pack/profile/conformance/practice-definable rather than a universal closed ontology in this kit.

Current parser support is JSON. YAML parsing remains a Ghost until implemented by a later scoped change.
