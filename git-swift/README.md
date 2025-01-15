# Git-Swift

Git-Swift is a Rust-based tool that automates the process of generating commit messages using the Gemini API, committing changes, and pushing them to a Git repository. This tool leverages AI to create meaningful commit messages based on the changes detected in the repository.

## Features

- Automatically detects changes in the repository
- Generates multiple AI-powered commit message options using the Gemini API
- Allows selection from multiple commit message options
- Commits the changes with the selected message
- Pushes the commit to the remote repository

## Prerequisites

- Rust (latest stable version)
- Git
- Gemini API key

## Installation

1. Clone the repository:
```sh
git clone https://github.com/its-me-ojas/git-swift.git
cd git-swift/git_swift
```

2. Build and install the project:
```sh
cargo build --release
```

3. Move the binary to a directory in your system's `PATH`:
```sh
sudo mv target/release/git-swift /usr/local/bin/
```

## Setup

Before using git-swift, you need to configure your Gemini API key:

```sh
git-swift setup YOUR_API_KEY
```

This will store your API key securely in `~/.git-swift/config`.

## Usage

Git-Swift provides two main commands:

1. Setup your API key:
```sh
git-swift setup YOUR_API_KEY
```

2. Generate commit message and push changes:
```sh
git-swift push
```

When you run `git-swift push`, the tool will:
1. Detect changes in your repository
2. Generate multiple commit message options using AI
3. Let you select your preferred message
4. Ask for confirmation before committing
5. Commit and push the changes

## Example Workflow

1. Make some changes to your repository

2. Run git-swift:
```sh
git-swift push
```

3. Select from the generated commit message options

4. Confirm the commit and push

The tool will then commit your changes with the selected message and push them to the remote repository.

## Help

To see available commands and options:
```sh
git-swift --help
```

## Configuration

The API key is stored in `~/.git-swift/config`. You can update it at any time by running the setup command again with a new API key.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## Author

Ojas - [its-me-ojas](https://github.com/its-me-ojas)
