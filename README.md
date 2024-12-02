# GitNav: A version-based git navigator
This package intends to facilitate navigating across a repo's history with the intent of modifying a prior release. An example of this would be backporting a release between major versions, where previously service branches would have been necessary.

_Note: this is my first time using Rust in years, and I was never particularly good. Use with caution!_

## How to use

```shell
❯ gitnav --help

Usage: gitnav [OPTIONS] -v <target version> <REPOSITORY_PATH>

Arguments:
  <REPOSITORY_PATH>
          

Options:
  -v <target version>
          Semver-compatible version pattern to find

  -c
          Create and checkout to a branch containing the last valid version

  -t <package framework>
          Package framework. Will affect which files to look at for versions.
          
          [default: npm]

          Possible values:
          - npm: For Node.js packages (default)

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```