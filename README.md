# Git Commit Message Generator

A terminal-based Rust application that analyzes your git diff and generates commit messages in various styles (corporate, pirate, shakespeare, haiku). Perfect for teams who want consistent, fun, or creative commit messages!

## Features
- Analyzes staged changes using `git diff`
- Generates commit messages in multiple personalities:
  - Corporate
  - Pirate
  - Shakespeare
  - Haiku
- Optionally shows the diff before generating the message
- Save your favorite messages for reuse

## Installation

1. **Clone the repository:**
   ```sh
   git clone https://github.com/harvathsteven/rust-commit-experiment.git
   cd rust-commit-experiment
   ```
2. **Build the project:**
   ```sh
   cargo build --release
   ```

## Usage

1. **Stage your changes:**
   ```sh
   git add .
   ```
2. **Run the generator:**
   ```sh
   cargo run -- --personality <style> [-d] [-f]
   ```
   - `--personality <style>`: Choose from `corporate`, `pirate`, `shakespeare`, `haiku` (default: corporate)
   - `-d`, `--show-diff`: Show the git diff before generating the message
   - `-f`, `--save`: Save the generated message to your favorites

**Example:**
```sh
cargo run -- --personality pirate -d
```

## Example Output
```
Git diff:
diff --git a/test.rs b/test.rs
index ...
+ // New test content

Generated commit message:
Arr, ye scurvy code be fixed now! (+1 -0)
```

## Contributing
Pull requests are welcome! For major changes, please open an issue first to discuss what you would like to change.

## License
MIT 