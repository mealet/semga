[Rust]: https://www.rust-lang.org/
[Tauri]: https://tauri.app/v1/guides/getting-started/prerequisites

<div align="center">
  <img src="https://github.com/user-attachments/assets/a9110a84-ad7d-4651-804c-5f24f474ebb3" />
</div>

## 👀 Desciprtion
**SEMGA (String Encryption Method Gui Avaible)** - the GUI representation of my `sem-rs` repository. <br>
Algorithm uses token string as alphabet to crypt given input. <br>
All logics and backend written in Rust 🦀 <br>
All UI and fronted created by using Svelte Framework with Vite 💜

## 😵‍💫 Installation
1. Download executable file from [latest release](https://github.com/mealet/semga/releases/latest)
2. Move it to any place on your computer
3. Launch it and enjoy!

## 🦛 Usage
**Encryption:**
1. Type you'r message into the first field.
2. Insert custom token, or generate it by button.
3. Click on `Encrypt` button and copy output.

**Decryption:**
1. Insert you'r encrypted string into the first field.
2. Type custom token to second field.
3. Click on `Decrypt` button and copy output.

## 🦈 Building from source
1. Download [Rust] language from official site.
2. Follow [Tauri] guide for installing dependencies.
3. Clone this repository to you'r local machine:
```
git clone https://github.com/mealet/semga
```
4. Install `tauri-cli` util:
```
cargo install tauri-cli
```
5. Build the app:
```
cargo tauri build
```

## 💀 License
Project licensed under the MIT License.
See more in License file.
