# Git-Swift Server

This server is built to accept requests from the git-swift cli and respond with generated commit messages

## Features

- Generate commit messages based on commit differences

## Prerequisites

- Rust (latest stable version)
- Gemini API key
- GCC (latest stable version)

## Installation

1. Clone the repository: 
```sh
git clone https://github.com/its-me-ojas/git-swift.git
cd git-swift/server
```
2. Run the server:
```sh
cargo run
```

## Setup

Before running the server, you need to setup the Gemini API key:
1. Copy the contents of sample.env to .env in the same directory
2. Enter your Gemini API key

You can change the port that the server runs on by change the following field in the .env:
```sh
PORT=YOUR_PORT_HERE
```

## Author

Tarush Mohindru - [tarushmohindru](http://github.com/tarushmohindru)