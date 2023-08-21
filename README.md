`typven` is a command-line interface for vendoring local Typst packages. Its core 
job is to take local packages that are not shared through [Typst's package 
management] and place them in the designated local Typst package 
directory to make them available system-wide.

For more information see the Typst [packages] repository.

## Installation

Typven's CLI is only available from the [releases page] where you can get 
sources and pre-built binaries for the latest release of typven.

## Usage

Installs packages in the [local system directory](#package-directory). It will
either install a top-level package or recursively search the next two 
subdirectories for packages and install each valid one.
```sh
# Install package(s) from the current working directory
typven install

# Install package(s) from a given directory
typven install A:/GitHub/my-packages

# Install package(s) from a repository
typven install --git https://github.com/jimvdl/typven.git
```

Viewing the installed packages can be done by running `ls`, this will output 
every package with all of their versions.
```sh
# List installed packages in table format
typven ls
```

If you want to clean packages from your system the `clean` subcommand can either 
clean the whole directory entirely or target a specific package by either name 
or a combination of a name and version.
```sh
# Clean all packages
typven clean

# Clean a specific package
typven clean mypkg

# Clean a spefic version for a given package
typven clean mypkg 0.2.5
```

## Package directory
Packages are stored in `{data-dir}/typst/packages/{namespace}/{name}/{version}` 
to make them available locally on your system. Here, `{data-dir}` is:
- `$XDG_DATA_HOME` or `~/.local/share` on Linux.
- `~/Library/Application Support` on macOS.
- `%APPDATA%` on Windows.

Packages in the data directory have precedence over ones in the cache directory. 
While you can create arbitrary namespaces with folders, the namespace typven 
uses is `local`:
- Stores a package in `~/.local/share/typst/packages/local/mypkg/1.0.0`
- Import from it with `#import "@local/mypkg:1.0.0": *`

## License

Licensed under either of
  * Apache License, Version 2.0 
    ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
  * MIT license 
    ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted 
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be 
dual licensed as above, without any additional terms or conditions.

[packages]: https://github.com/typst/packages
[Typst's package management]: https://typst.app/docs/packages/
[releases page]: https://github.com/jimvdl/typven/releases