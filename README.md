# git purge

Git extension to purge stale local branches deleted from remote.

## Installation

### Manual (macOS/Linux)

Note: Requires `rust`, `cargo`, `make` and `git` to be installed.

```sh
git clone https://github.com/mariocampacci/git-purge.git
cd git-purge
sudo make install
```

## Usage

```sh
git purge
``` 

### Options

- `-f, --force` - Force delete branches
- `-n, --dry-run` - Show what would be deleted without actually deleting
- `-v, --verbose` - Show detailed output
- `-V, --version` - Show version
- `-h` - Show help message

## Uninstall

```sh
brew uninstall git-purge  # if installed via brew
# or
sudo make uninstall       # if installed manually
```

## Contributing
Contributions are welcome! Please open issues and pull requests on GitHub.

### Setting up development environment
You need to have `rust`, `cargo`, and `make` installed.

After this, your environment will be ready for development.

## License
MIT License. See `LICENSE` file for details.