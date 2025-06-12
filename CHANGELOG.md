# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.2.0] - 2024-01-XX

### Added
- New `CurrentWork` GraphQL endpoint
- Custom ObjectId deserialization for MongoDB integration
- Support for fetching current work experience data
- Enhanced error handling and debugging capabilities

### Technical Details
- Added `CurrentWork` struct with proper serde annotations
- Implemented custom deserializer for MongoDB ObjectId format
- Added `current_work` resolver to GraphQL schema
- Maintains backward compatibility with existing endpoints

## [1.1.0] - Previous Release
- Initial release with basic GraphQL endpoints