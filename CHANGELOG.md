# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2025-01-11

### Added
- Added support for field-level getter and setter attributes
- Added `#[builder(getter)]` attribute to generate getter methods
- Added `#[builder(setter)]` attribute to generate setter methods
- Added test cases for getter and setter functionality

### Changed
- Changed attribute parsing to use field-level attributes instead of struct-level attributes
- Improved error handling in attribute parsing
- Updated builder macro to handle visibility correctly

### Fixed
- Fixed attribute parsing to correctly handle multiple attributes on fields
- Fixed visibility of generated methods to match struct visibility
- Fixed pointer comparison in tests to use proper Arc methods

## [0.1.2] - 2025-01-09

### Added
- Added comprehensive test suite
- Added support for unit structs
- Added support for empty structs

## [0.1.1] - Initial Release

### Added
- Initial release of service-builder
- Basic builder pattern implementation
- Procedural macro support
