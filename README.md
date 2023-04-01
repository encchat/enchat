# Enchat

The end to end encrypted chat, made with Tauri and Svelte.
Your messages are safe thanks to Signal's Double Rachet algorithm.

Homepage (Polish): [enchat](https://enchat.kawuka.xyz)

## Download
Go to the releases tab.

### Build
#### Prerequisites
- NodeJS v18+
- Rust
- OpenSSL
#### Building
Once you checkout the repository, install packages via ```pnpm install``` and run ```pnpm run tauri build```.
For development purposes, use ```pnpm run tauri dev```
#### Windows
On windows, you may encounter problems with linking OpenSSL library. You can try downloading the OpenSSL library with choco, but the easier way is to compile openssl with this app. To do this, add ```--features bundled-openssl``` to the tauri command.
## Note
This app is just a learning project and is by no means production ready. If you need an encrypted chat, use Signal or Matrix, but not this app!


#### Icons
Used Icons come from the heroicon pack.