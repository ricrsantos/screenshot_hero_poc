# Screenshot Hero - Technical Feasibility Validation Report

Version: 1.0  
Date: June 2026

---

# Executive Summary

This document consolidates all Proofs of Concept (PoCs) executed during the technical validation phase of the Screenshot Hero project.

The objective was to validate the complete technical foundation required to build a Linux-native screenshot annotation application using:

- Rust
- GTK4
- Libadwaita
- XDG Desktop Portals
- ashpd
- Flatpak

The validation process focused on the highest-risk technical assumptions before implementation of the actual product.

---

# Final Conclusion

All critical technical assumptions were successfully validated.

The proposed architecture is technically viable and suitable for implementation.

The complete workflow has been proven:

```text
User requests screenshot
        ↓
GNOME Screenshot Portal opens
        ↓
User selects screen region
        ↓
Portal returns image URI
        ↓
Application accesses image file
        ↓
Application reads image metadata
        ↓
Application renders image inside GTK window
        ↓
Ready for annotation tools
```

No architectural blockers were found.

---

# Technology Stack Validated

| Component | Status |
|------------|----------|
| Rust | ✅ Validated |
| GTK4 | ✅ Validated |
| Libadwaita | ✅ Validated |
| XDG Desktop Portal | ✅ Validated |
| ashpd | ✅ Validated |
| Flatpak | ✅ Validated |
| Screenshot Portal | ✅ Validated |
| Screenshot File Access | ✅ Validated |
| Image Rendering | ✅ Validated |

---

# POC-001 - GNOME Screenshot Portal Validation

## Objective

Validate that a Rust application can invoke the GNOME Screenshot Portal using ashpd and receive a screenshot URI.

## Environment

- Rust
- ashpd
- GNOME Desktop
- XDG Desktop Portal

## Implementation

Used:

```rust
ashpd::desktop::screenshot::Screenshot
```

with:

```rust
Screenshot::request()
    .interactive(true)
    .modal(true)
```

## Results

### Screenshot Request

✅ Screenshot portal opened successfully

✅ User could select an area

✅ Screenshot was captured successfully

### Portal Response

✅ Portal returned a response object

✅ Portal returned a URI

Example:

```text
file:///home/user/Pictures/Screenshots/Screenshot From ...
```

### Findings

✅ GNOME Screenshot Portal works correctly

✅ ashpd integration works correctly

✅ Screenshot capture can be initiated from Rust

## Conclusion

POC-001 successful.

The application can interact with the GNOME Screenshot Portal and receive screenshot URIs.

---

# POC-002 - Screenshot Portal Response Validation

## Objective

Validate the structure of the response returned by ashpd.

## Results

### Portal Response

Initial output:

```text
Request(
"/org/freedesktop/portal/desktop/request/..."
)
```

After adjusting implementation:

```text
URI: file:///home/user/Pictures/Screenshots/...
```

### Findings

✅ Portal response contains screenshot URI

✅ URI is returned as Url

✅ URI can be extracted successfully

## Conclusion

POC-002 successful.

Screenshot URI retrieval is fully functional.

---

# POC-003-01 - Flatpak Build Validation

## Objective

Validate Flatpak build process.

## Environment

Runtime:

```text
org.freedesktop.Platform 25.08
```

SDK:

```text
org.freedesktop.Sdk 25.08
```

## Results

### Flatpak Builder

✅ flatpak-builder installed successfully

### Manifest

✅ Manifest parsed correctly

### Packaging

✅ Application packaged successfully

## Findings

### Rust Toolchain

Initial issue:

```text
cargo: command not found
```

Resolved by:

- Installing Rust SDK extension
- Adjusting build strategy

## Conclusion

POC-003-01 successful.

Flatpak packaging process validated.

---

# POC-003-02 - Screenshot Portal Inside Flatpak

## Objective

Validate screenshot capture from inside a Flatpak sandbox.

## Results

### Screenshot Request

✅ Screenshot portal opened

✅ User selected area

✅ Screenshot captured

### URI Returned

Example:

```text
file:///run/user/1000/doc/...
```

and

```text
file:///home/user/Pictures/Screenshots/...
```

depending on runtime context.

## Findings

✅ Screenshot portal works from Flatpak

✅ Portal communication works through sandbox

## Conclusion

POC-003-02 successful.

Screenshot capture works correctly inside Flatpak.

---

# POC-003-03 - Flatpak Screenshot File Access Validation

## Objective

Validate that a Flatpak application can access the screenshot file returned by the portal.

## Results

### URI Processing

✅ URI parsing successful

✅ URI → filesystem path conversion successful

### File Access

Outside Flatpak:

✅ File accessible

Inside installed Flatpak:

✅ File accessible

### Metadata

Successfully extracted:

✅ File size

✅ Image width

✅ Image height

Example:

```text
File size (bytes): 21588
Image width: 480
Image height: 270
```

### Runtime Hardening

Implemented:

✅ No unwrap() in runtime path

✅ Graceful error handling

✅ Safe URI parsing

✅ Safe image loading

## Important Discovery

### Builder Runtime vs Installed Runtime

Running:

```bash
flatpak-builder --run ...
```

does NOT behave exactly like:

```bash
flatpak run <app-id>
```

Builder runtime may not expose the same portal/file access behavior.

### Correct Validation Method

Authoritative validation path:

```bash
flatpak-builder --user --install ...
flatpak run <app-id>
```

## Flatpak Permissions Validated

```yaml
finish-args:
  - --share=ipc
  - --socket=session-bus
  - --talk-name=org.freedesktop.portal.Desktop
  - --filesystem=home
  - --socket=x11
  - --socket=wayland
```

## Findings

✅ Screenshot file accessible inside installed Flatpak

✅ Image metadata accessible

✅ Image dimensions accessible

✅ Runtime stable

## Conclusion

POC-003-03 successful.

The application can access and process screenshot files from inside Flatpak.

---

# POC-003-04 - Image Rendering Validation

## Objective

Validate that the captured screenshot can be rendered inside a GTK4/Libadwaita application.

## UI Architecture

Application contains:

- GTK4 Application
- Libadwaita ApplicationWindow
- HeaderBar
- "Take Screenshot" button
- Image display area

## Workflow

```text
Button Click
      ↓
Portal Opens
      ↓
Screenshot Captured
      ↓
URI Returned
      ↓
File Loaded
      ↓
Image Rendered
```

## Results

### Screenshot Flow

✅ Screenshot request successful

✅ Screenshot URI received

✅ File loaded successfully

### Rendering

✅ Image rendered inside GTK window

✅ Aspect ratio preserved

✅ Image displayed immediately after capture

### Flatpak Validation

Executed through:

```bash
flatpak run io.github.screenshothero.Poc003
```

Result:

✅ Rendering successful inside Flatpak

## Findings

✅ GTK4 image rendering works

✅ Screenshot file can be displayed directly

✅ Architecture suitable for annotation layer

## Conclusion

POC-003-04 successful.

The complete screenshot acquisition and display pipeline is operational.

---

# Overall Validation Matrix

## Screenshot Capture

✅ GNOME Screenshot Portal opens

✅ User can select region

✅ Screenshot generated

## Portal Integration

✅ ashpd works correctly

✅ Portal communication works

✅ URI returned successfully

## File Access

✅ URI parsing works

✅ URI to path conversion works

✅ File accessible

✅ Metadata readable

## Flatpak

✅ Build process validated

✅ Packaging validated

✅ Installed runtime validated

✅ Required permissions identified

## User Interface

✅ GTK4 application works

✅ Libadwaita integration works

✅ Screenshot displayed inside application

## Stability

✅ No runtime crashes

✅ Graceful error handling implemented

---

# Architecture Decision

The following architecture is approved for implementation:

```text
Rust
 ├─ GTK4
 ├─ Libadwaita
 ├─ ashpd
 ├─ XDG Desktop Portals
 ├─ image crate
 └─ Flatpak Distribution
```

---

# Implementation Readiness

The project is ready to move from feasibility validation to product implementation.

Validated capabilities:

✅ Screenshot acquisition

✅ Portal integration

✅ Flatpak distribution

✅ File access

✅ Image processing

✅ GTK rendering

No additional feasibility PoCs are required before implementation.

---

# Final Status

PROJECT STATUS: READY FOR IMPLEMENTATION

All critical technical assumptions have been validated successfully.

No blocking technical risks remain for the core Screenshot Hero workflow.