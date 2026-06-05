# Prompt for Codex 5.3 - POC-003-04

## Context

We are developing Screenshot Hero, a Linux desktop application written in Rust.

The application will run primarily on GNOME and be distributed as a Flatpak.

Previous POCs have already validated the following:

### POC-002

Successfully invoked the GNOME Screenshot Portal using `ashpd`.

The application can:

* Open the GNOME screenshot UI
* Let the user select an area
* Capture a screenshot
* Receive the resulting URI

### POC-003-01

Successfully built a Flatpak package.

### POC-003-02

Successfully executed the screenshot workflow from inside a Flatpak application.

Important discovery:

Running via:

```bash
flatpak-builder --run
```

does not provide the same runtime behavior as an installed Flatpak.

The correct validation flow is:

```bash
flatpak-builder --user --install --force-clean build-dir io.github.screenshothero.Poc003.yml

flatpak run io.github.screenshothero.Poc003
```

### POC-003-03

Successfully:

* received the screenshot URI
* converted URI to local path
* opened the image
* read image metadata
* read image dimensions

This works correctly inside an installed Flatpak.

## Objective of POC-003-04

Validate that a screenshot captured through the GNOME Screenshot Portal can be rendered inside a GTK4 application.

This is the final technical validation before implementing annotation tools.

## Requirements

Create a minimal GTK4 + Libadwaita application.

The application must contain:

### Main Window

* Libadwaita ApplicationWindow
* HeaderBar
* Button labeled:

```text
Take Screenshot
```

### Screenshot Flow

When the button is clicked:

1. Invoke the GNOME Screenshot Portal using `ashpd`
2. Open the interactive screenshot UI
3. Wait for user selection
4. Receive screenshot URI
5. Convert URI to a filesystem path
6. Load the image

### Rendering

Display the captured image inside the main window.

Requirements:

* Image must become visible immediately after capture
* Scale image to fit available space
* Preserve aspect ratio
* No scrolling required
* No zoom
* No annotations yet

### Error Handling

If:

* screenshot is canceled
* file cannot be opened
* URI conversion fails

Display a user-friendly message in the UI.

Do not panic.

Do not use unwrap().

## Technical Constraints

Use:

* Rust stable
* GTK4
* Libadwaita
* ashpd
* image crate if necessary

Prefer GTK-native image rendering APIs.

Avoid custom OpenGL rendering.

Keep implementation simple and focused on validating image display.

## Deliverables

Generate:

1. Complete Cargo.toml dependencies section
2. Complete src/main.rs
3. Any additional helper modules if required
4. Explanation of architecture
5. Instructions to build and run

The final result should be a working GTK4 Flatpak application that:

* captures a screenshot
* receives the resulting image
* renders the image inside the application window

This POC is considered successful when the captured screenshot becomes visible inside the application after the capture process completes.


## Guidelines
    - If you make more than 5 attempts, stop and ask me if you should continue.
    - If necessary, consult the documentation in context7. Use the corresponding installed skill.
    - Do not commit, our push in the repository.
    - In case of doubt, ask me.

