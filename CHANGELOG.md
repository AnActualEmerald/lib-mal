# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [v0.3.1]

### Changed

-   README example
-   Module documentation for `lib-mal`

### Fixed

-   A println that was accidentally left in

## [v0.3.0]

### Added

-   Local token encryption

## Changed

-   `get_auth_parts` no longer takes a redirect uri

## Fixed

-   Auth failing for "invalid_client", seems to be an issue on MAL's side when the parameter `redirect_uri` is present. Only applications with one URI registered with the API are supported unless this issue is resolved.

## [v0.2.0]

### Added

-   Improved docs for some MALClient functions

## Changed

-   `get_auth_parts` now generates a random state
-   `get_auth_parts` now allows for a specified redirect uri
-   `auth` now takes a state and a redirect

## [v0.2.0]

### Added

-   Improved docs for some MALClient functions

## Changed

-   `get_auth_parts` now generates a random state
-   `get_auth_parts` now allows for a specified redirect uri
-   `auth` now takes a state and a redirect

## [v0.1.0]

### Added

-   Oauth2 authorization
-   `Get anime list` API funciton
-   `Get anime details` API function
-   `Get anime ranking` API function
-   `Get seasonal anime` API function
-   `Get user anime list` API funciton
-   `Delete user anime list item` API function
-   `Get forum boards` API function
-   `Get forum topic details` API function
-   `Get forum topics` API funciton
-   `Get my user information` API function
-   Enums for available fields and request options
