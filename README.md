# Git-Swift

Git-Swift is a Rust-based tool that automates the process of generating commit messages using the Gemini API, committing changes, and pushing them to a Git repository. This tool leverages AI to create meaningful commit messages based on the changes detected in the repository.

## Features

- Automatically detects changes in the repository.
- Generates commit messages using the Gemini API.
- Commits the changes with the generated commit message.
- Pushes the commit to the remote repository.

## Prerequisites

- Rust (latest stable version)
- Git
- Gemini API key

## Installation

1. Clone the repository:

```sh
git clone https://github.com/its-me-ojas/git-swift.git
cd git-swift
```

2. Set up your environment variables by creating a `.env` file in the project directory with the following content:

```
GEMINI_API_KEY=your-gemini-api-key
```

3. Install the required Rust dependencies and build the project:

```sh
cargo build --release
```

## Usage

### Using the Binary Globally

To use the binary globally in any repository, follow these steps:

1. Build the project to create the binary:

```sh
cargo build --release
```

2. Move the binary to a directory that is in your system's `PATH`. For example, you can move it to `/usr/local/bin`:

```sh
sudo mv target/release/git-swift /usr/local/bin/
```

3. Now you can use the `git-swift` command in any initialized Git repository. Navigate to the root directory of your Git repository and run:

```sh
git-swift
```

The tool will:

- Detect changes in the repository.
- Generate a commit message using the Gemini API.
- Commit the changes with the generated commit message.
- Push the commit to the remote repository.

## Example

Here's an example of how to use Git-Swift:

1. Make some changes to your repository.

2. Run the tool:

```sh
git-swift
```

3. The tool will output the generated commit message and push the changes to the remote repository.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request if you have any improvements or bug fixes.
