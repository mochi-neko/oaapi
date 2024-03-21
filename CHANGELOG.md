# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Add `embeddings` APIs.
- Add `fine-tuning` APIs.
- Add `files` APIs.
- Add `images` APIs.
- Add `models` APIs.
- Add `moderations` APIs.
- Add `assistants` APIs.
- Add `threads` APIs.
- Add `messages` APIs.
- Add `runs` APIs.

## [0.2.0] - 2024-03-20

### Added

- Add third party re-exports: `serde_json` and `subtp` in APIs.
- Add details for documents and examples.

### Changed

- Replace `Channel` with `Stream` in the `audio/speech` API.
- Replace `Channel` with `Stream` in the `chat/completions` streaming API.
- Improve third party dependencies with improving `oaapi::audio::File`.

## [0.1.1] - 2024-03-09

### Added

- Add `gpt-3.5-turbo-0125` model.

### Fixed

- Fix serialization error at request body of `audio/speech`: `oaapi::audio::SpeechRequestBody`.
- Fix panic at error handling of `chat/completions` streaming: `oaapi::chat::stream_line_reader::StreamLineReader`.
- Fix serialization error of the chat stream option: `oaapi::chat::StreamOption`.

## [0.1.0] - 2024-03-07

### Added

- Add `audio` APIs.
- Add `chat` APIs.

[unreleased]: https://github.com/mochi-neko/oaapi/compare/v0.2.0...HEAD

[0.2.0]: https://github.com/mochi-neko/oaapi/compare/v0.1.1...v0.2.0

[0.1.1]: https://github.com/mochi-neko/oaapi/compare/v0.1.0...v0.1.1

[0.1.0]: https://github.com/mochi-neko/oaapi/releases/tag/v0.1.0
