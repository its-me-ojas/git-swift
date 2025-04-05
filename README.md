# Git-Swift

Git-Swift is a solution powering developers to don't worry at all about version control as our advanced AI solution is the one handling all that work.

## Features

- Automatically detects repository changes
- Generates multiple commit message options using AI
- Follows conventional commit message format
- Interactive selection of preferred message
- Handles commit and push operations

## Prerequisites

- Rust (latest stable)
- Git
- Groq API key

## Installation

1. Clone the repository:
```sh
git clone https://github.com/its-me-ojas/git-swift.git
cd git-swift
```

2. Start the server:
```sh
cd server
docker compose up -d
```

3. Build and install the CLI:
```sh
cd ../git-swift
cargo install --path .
```

## Usage

1. Make changes to your repository
2. Run:
```sh
git-swift push
```
3. Select preferred commit message
4. Changes will be committed and pushed

## Environment Variables

Create a `.env` file in the server directory:
```sh
GROQ_API_KEY=your_api_key_here
PORT=5000
```

## Authors

- Ojas - [@its-me-ojas](https://github.com/its-me-ojas)
- Tarush Mohindru - [@tarushmohindru](https://github.com/tarushmohindru)
