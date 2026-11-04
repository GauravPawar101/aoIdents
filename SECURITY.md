# Security policy

## Reporting a vulnerability

**Do not open a public issue for a vulnerability.** Report it privately through
[GitHub Security Advisories](https://github.com/aoheyra/aoIdents/security/advisories/new)
("Report a vulnerability") on this repository. The maintainers will acknowledge
the report, investigate and coordinate a fix and disclosure with you.

Please include:

- the affected area (identity derivation, signatures, sealed messages,
  rotating tags, replay guard, backup, …) and the `IDN-xx` issue you believe
  is involved, if known;
- a description of the impact and, if possible, a minimal reproducer or
  known-answer input;
- whether the issue is already public or under embargo.

This is the most security-critical crate in the project: reports here are
treated as top priority.

## Scope

In scope are anything that lets an attacker forge, replay or tamper with
signatures or sealed messages, extract or misuse keys, link rotating tags to
a `NodeId`, bypass verification or replay checks, or recover secret material
(including through `Debug` output or error messages).

Not in scope: issues that require a compromised local device or a malicious
build of the app itself; vulnerabilities in sibling crates (`aoMesh`,
`aoNetwork`, `aoheyra`), which are reported against their own repos.

## Supported versions

Pre-`v0.1.0` there are no supported releases: the repository `master` branch
is the only supported target and fixes land there. Once `v0.1.0` is tagged,
security fixes are backported to the latest minor release line.
