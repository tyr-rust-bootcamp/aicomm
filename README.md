# aicomm

A modern, real-time chat application built with Vue.js and Tauri.

## Features

- Real-time messaging
- User-friendly interface
- work across web and desktop
- Lightweight and fast performance

## Prerequisites

Before you begin, ensure you have met the following requirements:

- Node.js (v18 or later)
- Rust (latest stable version)
- Tauri CLI

## Setup

1. Clone the repository:
   ```
   git clone https://github.com/tyr-rust-bootcamp/aicomm.git
   cd aicomm
   ```

2. Install dependencies:
   ```
   cd chatapp
   yarn
   ```


## Running the App

First, run the server:
```
cd chat/chat_server
cargo run

cd chat/notify_server
cargo run
```

To run the desktop app, you could use:
```
cd chatapp
cargo tauri dev
```

To run the web app, you could use:
```
cd chatapp
yarn dev
```
