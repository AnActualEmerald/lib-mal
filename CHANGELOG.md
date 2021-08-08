# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]


## [v0.4.0]

### Added

- Documentation for bitflags usage
- `get_access_token` method for `MALClient`
- Tests for basic anime list functions
- Doc tests to ensure working documentation
- Method to create an `MALClient` with just an access token

### Changed

- Available fields for API functions are now represented using bitflags (Thanks dblanovschi)
- Functions now return a `MALError` struct rather than plain strings when returning an error
- `MALClient::new` to `MALClient::init`

### Removed

- `MALClient::get_my_user_info` no longer takes a fields argument as it seems there's only one field option

## [v0.3.2]

### Fixed

- Another extraneous println
- Fields for the User struct being private
- Season field for AnimeList struct being private
- Typo in `MALClient::new` doc
- Typo in `MALClient::get_anime_details` doc

## [v0.3.1]

### Changed

- README example
- Module documentation for `lib-mal`

### Fixed

- A println that was accidentally left in

## [v0.3.0]

### Added

- Local token encryption

## Changed

- `get_auth_parts` no longer takes a redirect uri

## Fixed

- Auth failing for "invalid_client", seems to be an issue on MAL's side when the parameter `redirect_uri` is present. Only applications with one URI registered with the API are supported unless this issue is resolved.

## [v0.2.0]

### Added

- Improved docs for some MALClient functions

## Changed

- `get_auth_parts` now generates a random state
- `get_auth_parts` now allows for a specified redirect uri
- `auth` now takes a state and a redirect

## [v0.2.0]

### Added

- Improved docs for some MALClient functions

## Changed

- `get_auth_parts` now generates a random state
- `get_auth_parts` now allows for a specified redirect uri
- `auth` now takes a state and a redirect

## [v0.1.0]

### Added

- Oauth2 authorization
- `Get anime list` API function
- `Get anime details` API function
- `Get anime ranking` API function
- `Get seasonal anime` API function
- `Get user anime list` API function
- `Delete user anime list item` API function
- `Get forum boards` API function
- `Get forum topic details` API function
- `Get forum topics` API function
- `Get my user information` API function
- Enums for available fields and request options
