# ASIMOV Apify Module

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.85%2B-blue)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)
[![Package](https://img.shields.io/crates/v/asimov-apify-module)](https://crates.io/crates/asimov-apify-module)

[ASIMOV] module for data import powered by the [Apify] web automation platform.

## 🛠️ Prerequisites

- [Rust](https://rust-lang.org) 1.85+ (2024 edition)

## ⬇️ Installation

### Installation from Source Code

```bash
cargo install asimov-apify-module
```

## 👉 Examples

```console
$ export APIFY_TOKEN="..."

$ asimov-apify-importer https://www.linkedin.com/in/arto/
```

## ⚙ Configuration

### Environment Variables

- `APIFY_TOKEN`: (required) the [Apify API token] to use

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
