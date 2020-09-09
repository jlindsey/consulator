Consulator
==========

[![No Maintenance Intended](http://unmaintained.tech/badge.svg)](http://unmaintained.tech/)

[![GitHub release](https://img.shields.io/github/release/jlindsey/consulator.svg)](https://github.com/jlindsey/consulator/releases/)

![Build](https://github.com/jlindsey/consulator/workflows/push/badge.svg)

Simple CLI program to take a JSON input and map it into a Consul KV store such that a
[consul-template][1] file or [Nomad template stanza][2] can access it via `tree`.

This is a project for personal convenience and as an excuse to learn about Rust
cross-compilation with GitHub actions, so its usefulness to you may be limited and it
will not be actively maintained.

[1]: https://github.com/hashicorp/consul-template
[2]: https://www.nomadproject.io/docs/job-specification/template.html
