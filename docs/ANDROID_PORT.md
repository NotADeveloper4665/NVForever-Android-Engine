# Android engine port

This fork is the native engine side of NVForever-Android. The Android launcher remains in
[NotADeveloper4665/NVForever-Android](https://github.com/NotADeveloper4665/NVForever-Android).

## Architecture

- `engine/nvforever` is built as both a Rust library and a desktop executable wrapper.
- Android loads `libnvforever.so` through Bevy 0.20.1's GameActivity backend.
- The Android build targets ARM64 (`aarch64-linux-android`) and API 26 initially because
  the engine's audio stack uses CPAL/Rodio.
- Game data remains user-provided and read-only. The launcher stages it in app-private
  storage; the engine must receive that directory as `NVFOREVER_DATA`/runtime config.
- ESM/NIF schemas and script-function metadata must be made available to the native
  runtime. They are not game data and must ship with the app or be passed as app-private
  paths.

## First native build

From the repository root with Android NDK installed:

```sh
rustup target add aarch64-linux-android
cargo install cargo-ndk --locked
cd engine
cargo ndk -t arm64-v8a -P 26 -o ../artifacts/jniLibs build --release --locked --lib -p nvforever
```

The CI workflow performs this build and publishes `libnvforever.so` as an artifact.

## Known port work still required

A successful cross-compile is only the native-library milestone, not a playable Android
build. The current desktop runtime still needs platform-specific work before it can boot
reliably on a phone:

1. Android lifecycle and activity suspension/resume handling, including pausing audio and
   saving safely when the activity is backgrounded.
2. A reliable bridge from the Kotlin launcher to the native runtime for the staged data,
   schema, save, and log directories. Android process arguments are not a dependable way
   to carry the SAF-selected path.
3. Android-safe config/save/crash-report paths; desktop home-directory assumptions must
   not leak into the app sandbox.
4. Android GPU validation and fallback settings. The renderer currently directly uses
   Vulkan/WGPU HAL details, desktop window settings, MSAA, and high-cost post-processing.
5. Android audio backend validation and device lifecycle handling.
6. Touch/gamepad mapping into the existing gameplay input systems, then physical-device
   testing of main menu, new game, movement, combat, dialogue, saves, and reloads.

Do not enable the launch action in the Android shell until the library is packaged into
that app, the runtime receives its paths, and a device smoke test reaches the main menu.

## Compatibility policy

Keep Bevy pinned to `=0.20.1` while bringing up Android. Avoid broad dependency upgrades
as part of the port. Keep Android-specific code behind `cfg(target_os = "android")` and
comment every device-specific workaround with the affected API/device behavior.
