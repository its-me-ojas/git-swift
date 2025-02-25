# Git-Swift Server

This server is built to accept requests from the git-swift cli and respond with generated commit messages

## Features

- Generate commit messages based on commit differences using Groq's AI models

## Prerequisites

- Rust (latest stable version)
- Groq API key
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

Before running the server, you need to setup the Groq API key:
1. Copy the contents of sample.env to .env in the same directory
2. Enter your Groq API key

You can change the port that the server runs on by changing the following field in the .env:
```sh
PORT=YOUR_PORT_HERE
```

## Usage

- Make POST request to /commit_message with the following json body -
```sh
{
    "diff": "Commit difference here"
}
```
- The response is a json object with the following structure -
```sh
{
    "messages": ["The 3 commit messages"]
}
```

## Authors

Tarush Mohindru - [tarushmohindru](http://github.com/tarushmohindru)
Ojas - [its-me-ojas](http://github.com/its-me-ojas)
