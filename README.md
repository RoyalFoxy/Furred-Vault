# Furred Vault

- [Furred Vault](#furred-vault)
  - [Basic concept](#basic-concept)
  - [Developing it](#developing-it)
  - [Features](#features)
  - [Technologies](#technologies)
  - [Development](#development)
    - [Prerequisites](#prerequisites)
    - [Installing dependencies](#installing-dependencies)
    - [Running the dev version](#running-the-dev-version)
  - [Build](#build)
    - [App Icon](#app-icon)
    - [Splash screen](#splash-screen)

## Basic concept

Security is the focus of this password manager. Even tho the encrypted password entries are stored as is on the disk of the user the are still encrypted with Aes256Gcm and unless Aes256Gcm gets broken this combined with a secure enough master password should prevent hacking stored password entries.

The application itself is also secured by using Tauri which ensures that no unwanted calls to system-apis are allowed from the frontend including xss attacks.

The performance of the entire application is kept to a maximum by using Rust with Tauri and TypeScript with SvelteKit.

The default configuration does **not** store the password entries anywhere besides the local machine.

The interface was designed with ease of use in mind. The design is kept minimalistic on purpose.

The concept of multiple password storages, here called vaults, exists and requires a password for each separate storage.

This password manager also provides a possibility of creating secure and random passwords for each entry.

The binary encrypted files are stored under the config folders of each major os

| OS      | Path                                                         |
| ------- | ------------------------------------------------------------ |
| Windows | {FOLDERID_RoamingAppData}\Furred Vault\config                |
| Macos   | $HOME/Library/Application Support/Furred-Vault               |
| Linux   | $XDG_CONFIG_HOME/furredvault or    $HOME/.config/furredvault |

## Developing it

I wasn't really unfamiliar with either Tauri or svelte as I've done few projects that use these technologies but I haven't sofar combined the two to make a single project. I never faced any challenges as it was pretty straight forward on how I should approach a problem when developing this app.

## Features

**As of the 12th June 2024 this project is still WIP and will be continued being worked on.**

The feature list below isn't fully complete and will grow over time and requests.

| Features                          | Progress       |
| --------------------------------- | -------------- |
| Password saving                   | ✅ Obviously :) |
| Multiple Vaults                   | ✅              |
| Flash screen                      | ✅              |
| Custom *slightly annoying* cursor | ✅              |
| Toasts!                           | ✅              |
| Vault renaming                    | ✅              |
| Vault deletion*                   | ✅              |
| Change vault password             | ✅              |
| Entry notes for details           | ✅              |
| Multiple Vaults                   | ✅              |
| Password generator                | ✅              |
| Multiple Vaults                   | ✅              |
| History via git                   | ❌              |
| Backup via Github etc.            | ❌              |
| Device sync                       | ❌              |
| Advanced password search          | 🛠️              |
| App integration                   | ❌              |
| Automatic password saving         | ❌              |
| Design/Animation improvements     | 🛠️              |

| Emoji | Meaning         |
| ----- | --------------- |
| ❌     | Not done        |
| 🛠️     | Being worked on |
| ✅     | Done            |

\* Vault deletion is technically possible without a password as well as it only requires deleting the respective binary file but I highly discourage this.

## Technologies

This project uses Tauri for the app stack and SvelteKit for the frontend as well as Tailwind for the styling. Eslint with a few specific plugins is used to keep everything tidy.

## Development

The steps to get this working are as follows

You need the bun runtime for any of the setups below. You can get an install script from their [website](https://bun.sh/).

### Prerequisites

Visit and follow the prerequisites page on Tauris documentation matching your operating system. [Windows](https://tauri.app/v1/guides/getting-started/prerequisites#setting-up-windows), [MacOS](https://tauri.app/v1/guides/getting-started/prerequisites#setting-up-macos), [Linux](https://tauri.app/v1/guides/getting-started/prerequisites#setting-up-linux)

If you want to manage the rust installation it's the same on all operation systems and you can find out more on Tauris page about [managing the rust installation](https://tauri.app/v1/guides/getting-started/prerequisites#managing-the-rust-installation)

### Installing dependencies

You have to only manually download the JavaScript dependencies with `bun i`

### Running the dev version

Execute this command to get it to run `bun tauri dev`

## Build

If you want to build the executable yourself you should follow the steps in [Development](#development) but instead of running `bun tauri dev` at the end you simply do `bun tauri build`. After it's done compiling and packing everything it will prompt you to install it

### App Icon

The app icon is generated via Tauris builtin icon builder. There is a script in the `package.json` `bun build:icon` which takes a "raw image" and transforms it with sharp to look more like a suitable icon it then also executes the before mentioned Tauri command to create all the platform specific icon formats and writes them to the correct folder

### Splash screen

The splash screen is built once before the dev or build command are executed. It's a really basic sub project that uses kitajs to create html from some jsx and also builds the tailwind classes with it.
