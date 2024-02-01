# MyPass GUI

MyPass is a lightweight, cross-platform desktop application built using **Tauri** and Astro & Svelte as web technologies for the UI. It allows users to manage their passwords securely. All the credentials are stored encrypted in a MongoDB database.

The main goal of this project is to put in practice my knowledge of Rust and Tauri, and also to learn Astro and Svelte.

>[!warning]
> This is not a production ready project and im not a security expert, so use this application at your own risk.

## Features

- Create, read, update and delete credentials.
- Generate random passwords.
- Copy credentials to the clipboard.
- Dark/Light mode (depends in your system settings).

## Stack

The application is built using the following technologies:

- Tauri
- Rust
- Astro
- Sevelte
- Tailwind CSS
- Bun

## Getting Started

To run the application locally, follow these steps:

1. Clone the repository: `git clone https://github.com/gomezbc/mypass-gui.git`
2. Navigate to the project directory: `cd mypass-gui`
3. Install dependencies: `bun install`
4. Install Tauri CLI: `cargo install tauri-cli`
5. Compile the Tauri app: `cargo tauri build`
6. The window will pop up automatically.

## Showcase

![Screenshot from 2024-01-30 18-36-54](https://github.com/gomezbc/mypass-gui/assets/77118356/ac1a3439-3187-4431-ab26-b1635df3d2f0)


![Screenshot from 2024-01-30 18-38-15](https://github.com/gomezbc/mypass-gui/assets/77118356/781da1f9-feb5-42ff-b382-a1c9294c8e2b)
