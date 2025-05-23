# ASIMOV Apify Module

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.85%2B-blue)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)
[![Package](https://img.shields.io/crates/v/asimov-apify-module)](https://crates.io/crates/asimov-apify-module)

[ASIMOV] module for data import powered by the [Apify] web automation platform.

## ✨ Features

- Imports structured data from Apify actors.
- Collects the raw JSON data via the Apify API (requires an API token).
- Constructs a semantic knowledge graph based on the [KNOW] ontology.
- Supports plain JSON output as well as [RDF] output formats such as JSON-LD,
  Turtle, and N-Triples.

## 🛠️ Prerequisites

- [Rust](https://rust-lang.org) 1.85+ (2024 edition)

## ⬇️ Installation

### Installation from Source Code

```bash
cargo install asimov-apify-module
```

## 👉 Examples

```bash
export APIFY_TOKEN="..."
```

### Fetching LinkedIn Profiles

```bash
asimov-apify-fetcher https://www.linkedin.com/in/arto/
```

## ⚙ Configuration

### Environment Variables

- `APIFY_TOKEN`: (required) the [Apify API token] to use

## 📚 Reference

### Installed Binaries

- `asimov-apify-fetcher`: collects JSON data from the Apify API
  _(not implemented yet)_

### Supported Actors

Actor   | URL Prefix | JSON | RDF
:------ | :--------- | :--: | :--:
<img width="100" height="1"/> | <img width="550" height="1"/> | <img width="50" height="1"/> | <img width="50" height="1"/>

## 👨‍💻 Development

```bash
git clone https://github.com/asimov-modules/asimov-apify-module.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/asimov-modules/asimov-apify-module&text=asimov-apify-module)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/asimov-modules/asimov-apify-module&title=asimov-apify-module)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/asimov-modules/asimov-apify-module&t=asimov-apify-module)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/asimov-modules/asimov-apify-module)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/asimov-modules/asimov-apify-module)

[ASIMOV]: https://github.com/asimov-platform
[Apify]: https://apify.com
[Apify API token]: https://docs.apify.com/platform/integrations/api
[KNOW]: https://github.com/know-ontology
[RDF]: https://github.com/rust-rdf
