# ASIMOV Apify Module

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Package on Crates.io](https://img.shields.io/crates/v/asimov-apify-module.svg)](https://crates.io/crates/asimov-apify-module)
[![Package on PyPI](https://img.shields.io/pypi/v/asimov-apify-module.svg)](https://pypi.org/project/asimov-apify-module)
[![Package on RubyGems](https://img.shields.io/gem/v/asimov-apify-module.svg)](https://rubygems.org/gems/asimov-apify-module)
[![Package on NPM](https://img.shields.io/npm/v/asimov-apify-module.svg)](https://npmjs.com/package/asimov-apify-module)

[ASIMOV] module for data import powered by the [Apify] web automation platform.

## ✨ Features

- Imports structured data from Apify actors (web automation scripts).
- Collects the raw JSON data via the Apify API (requires an API token).
- Constructs a semantic knowledge graph based on the [KNOW] ontology.
- Supports plain JSON output as well as [RDF] output in the form of [JSON-LD].
- Distributed as a standalone static binary with zero runtime dependencies.

## 🛠️ Prerequisites

- [Rust] 1.85+ (2024 edition) if building from source code

## ⬇️ Installation

### Installation from PyPI

```bash
pip install -U asimov-apify-module
```

### Installation from RubyGems

```bash
gem install asimov-apify-module
```

### Installation from NPM

```bash
npm install -g asimov-apify-module
```

### Installation from Source Code

```bash
cargo install asimov-apify-module
```

## 👉 Examples

```bash
export APIFY_TOKEN="..."
```

### Fetching Google Results

```bash
asimov-apify-fetcher  https://www.google.com/search?q=Isaac+Asimov  # JSON
asimov-apify-importer https://www.google.com/search?q=Isaac+Asimov  # JSON-LD
```

### Fetching X (Twitter) Followers

```bash
asimov-apify-fetcher  https://x.com/apify/followers  # JSON
asimov-apify-importer https://x.com/apify/followers  # JSON-LD
```

### Fetching X (Twitter) Followees

```bash
asimov-apify-fetcher  https://x.com/apify/following  # JSON
asimov-apify-importer https://x.com/apify/following  # JSON-LD
```

### Fetching LinkedIn Profile

```bash
asimov-apify-fetcher  https://www.linkedin.com/in/sarptecimer  # JSON
asimov-apify-importer https://www.linkedin.com/in/sarptecimer  # JSON-LD 
```

### Fetching Instagram Profile

```bash
asimov-apify-fetcher  https://www.instagram.com/humansofny  # JSON
asimov-apify-importer https://www.instagram.com/humansofny  # JSON-LD 
```

## ⚙ Configuration

### Environment Variables

- `APIFY_TOKEN`: (required) the [Apify API token] to use

## 📚 Reference

### Installed Binaries

- `asimov-apify-fetcher`: collects JSON data from the Apify API
- `asimov-apify-importer`: collects and transforms JSON into JSON-LD

### Supported Actors

| Actor                         | URL Pattern                              |             JSON             |             RDF              |
|:------------------------------|:-----------------------------------------|:----------------------------:|:----------------------------:|
| Google Search                 | `https://www.google.com/search?q=:query` |              ✅               |              ✅               |
| X (Twitter) Followers         | `https://x.com/:account/followers`       |              ✅               |              ✅               |
| X (Twitter) Followees         | `https://x.com/:account/following`       |              ✅               |              ✅               |
| LinkedIn Profile              | `https://www.linkedin.com/in/:username`  |              ✅               |              🚧              |
| Instagram Profile             | `https://www.instagram.com/:username`    |              ✅               |              🚧              |
| <img width="100" height="1"/> | <img width="550" height="1"/>            | <img width="50" height="1"/> | <img width="50" height="1"/> |

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
[JSON-LD]: https://json-ld.org
[KNOW]: https://github.com/know-ontology
[NPM]: https://npmjs.org
[Python]: https://python.org
[RDF]: https://github.com/rust-rdf
[Ruby]: https://ruby-lang.org
[Rust]: https://rust-lang.org
