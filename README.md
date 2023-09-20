Both a module and a simple CLI to bump NPM and Cargo versions (Go, iOS, and Android support coming soon).

Install with "cargo install cnctd_bump"

Use by navigating to the root directory of your NPM or Cargo project and running "cnctd-bump [version-part]"

Acceptible version parts are "major", "minor", and "patch". If no version part is specified, it defaults to "patch"