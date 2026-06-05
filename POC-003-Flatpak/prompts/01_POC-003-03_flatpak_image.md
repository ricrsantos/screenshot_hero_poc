# Prompt for Codex 5.3 - POC-003-03 Completion

## Context

We are developing a Rust-based POC application (`screenshot-poc`) that interacts with the GNOME screenshot portal via `ashpd`. The previous POCs have validated the following:

- **POC-003-01 & POC-003-02:** Rust environment, cargo build, Flatpak build process, and simple binary installation were successful.
- **POC-003-02:** Running `cargo run` outside Flatpak successfully captured screenshots, returned a `file://` URI, and printed metadata (size, width, height). The `image` crate was used to validate the screenshot.
- **POC-003-03:** Running inside Flatpak works, but URIs returned by the portal are sandboxed under `/run/user/1000/doc/...`, which causes `std::fs::metadata` and `image::open` to fail because the Flatpak sandbox does not allow access to that path. We need to fix sandbox permissions in the Flatpak YAML and adjust the Rust code to handle portal URIs properly.

The Flatpak YAML must include:
- `--share=ipc` and `--socket=session-bus` for DBus communication with the portal.
- `--talk-name=org.freedesktop.portal.Desktop` to access the screenshot portal.
- `--filesystem=home` to allow saving/reading files.
- `--socket=x11` and `--socket=wayland` to allow screen capture.

## Task

Update the `screenshot-poc` Rust application to correctly handle screenshot URIs returned by the GNOME portal when running inside Flatpak:

1. Request a screenshot interactively and modally using `ashpd::desktop::screenshot::Screenshot`.
2. Retrieve the URI from the portal response.
3. Check if the file exists under the path obtained from the URI. If it does not exist, log a warning and exit gracefully instead of panicking.
4. If the file exists, open it using the `image` crate.
5. Print the following information:
   - URI
   - Converted filesystem path (if possible)
   - File existence (true/false)
   - File size in bytes
   - Image width
   - Image height
6. Ensure all unwraps are removed; handle errors using proper `Result` handling or logging.
7. Maintain asynchronous execution using `#[tokio::main]`.

### Additional notes

- Assume the Rust environment is already set up and the Flatpak SDK includes `rust-stable`.
- Preserve previous functionality: interactive screenshot, printing URI, metadata, and image dimensions.
- The application should not crash if the screenshot path is not accessible from inside the Flatpak sandbox.
- Make sure the final code compiles and runs both outside and inside Flatpak (with appropriate sandbox permissions).

## Deliverable

Generate the **complete Rust source code** for `src/main.rs` for POC-003-03 implementing all the requirements above. The code must be ready to compile with `cargo build` and run with `cargo run`.

## Guidelines
    - If you make more than 5 attempts, stop and ask me if you should continue.
    - If necessary, consult the documentation in context7. Use the corresponding installed skill.
    - Do not commit, our push in the repository.
    - In case of doubt, ask me.

