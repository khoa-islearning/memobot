# Memobot

New iteration of Memodrone, with backend written in Rust.

Feature:
- Task keeping with url
- Rating system help scheduling next learning period, based on difficulty
- Cross-platform with Tauri

TODO:
- installation package
- system tray
- improved scheduling algorithm

## Running the code: 
1. install npm
2. Install Rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
3. Install Tauri prereq
```
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
```
for gnome eleven on my laptop: you will need to install https://extensions.gnome.org/extension/615/appindicator-support/ for menu
