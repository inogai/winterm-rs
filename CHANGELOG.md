# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2025-11-04

### Added

- Command-line arguments for configuring animation duration and spawn interval
- Configurable snowball spawning chance and cluster size via CLI options

## [0.1.0] - 2025-11-04

### Added

- Initial release of `winterm-rs`, a terminal-based snowfall animation program.
- Implemented `Entity` trait for managing game objects with update, render, and
  position functionality.
- Added `Snowball` entity that simulates falling snowflakes with random
  horizontal position and falling speed.
- Basic animation loop with frame-based updates, clearing the screen, and
  rendering objects.
- Random spawning of snowball clusters with configurable chance and cluster
  size.
- Terminal size detection and cursor management using `crossterm`.
- Automatic program exit after a set number of frames for demonstration
  purposes.
