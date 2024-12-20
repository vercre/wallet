# Vercre Wallet

The Vercre Wallet is a simple example of a wallet that can be used to receive, store and present Verifiable Credentials. It is a demonstration of the interactions between a wallet and issuance service that conforms to [OpenID for Verifiable Credential Issuance](https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0.html) and a verification service that conforms to [OpenID for Verifiable Presentations](https://openid.net/specs/openid-4-verifiable-presentations-1_0.html).

A conformant issuer service can be constructed using the Vercre Issuer crate, and a conformant verification service can be constructed using the Vercre Verifier crate. This wallet is built using the Vercre Holder crate which provides a set of convenient data types and functions for constructing a wallet. The Holder crate and this Wallet, while adhering to the OpenID standards for issuance and presentation flows do not conform internally to any standards but simply provide a "for-instance" example you may wish to use to influence your own wallet project for use with standards-compliant issuers and verifiers. See the open source [Vercre](https://github.com/vercre/vercre) repository for details.

## Multiplatform with Crux

The Vercre Wallet is built using the [Crux](https://github.com/redbadger/crux) framework which allows for targeting multiple platforms with a single codebase. The Vercre Wallet is currently built for the web, desktop and iOS platforms.

## Getting Started

### Prerequisites

#### Rust targets

See the `rust-toolchain.toml` file for cross-platform targets that should be installed. This is done
by running the following command:

```shell
rustup target list --installed
```

#### pnpm

The version of Crux used in this project uses the `pnpm` package manager for generating TypeScript.
There are [various ways to install `pnpm`](https://pnpm.io/installation).

#### Xcode

For building the iOS application, you will need Xcode installed with an iOS simulator. (This alpha code has only been tested on the iOS 18 simulator.)

You will also need to install the [command line tools for Xcode](https://developer.apple.com/download/all/).

### Rust core application

1. Make sure the core builds (located in the `wallet` directory):

```shell
cargo build --package vercre-wallet
```

2. Generate shared types

```shell
cargo build --package shared
```

### Mobile application - iOS

```shell
cd iOS
open VercreWallet.xcodeproj
```

## Sample Issuance and Verification

To demonstrate the wallet you can use the services and web applications provided in the `vcservice` and `vcweb` folders.

You can run these applications somewhere accessible to a mobile device or use the following steps to build and run locally using a Docker runtime and [ngrok](https://ngrok.com/) to expose localhost services to the internet.

By default the web application will be available at http://localhost:3000 and the service at http://localhost:8080.

In order to serve the API over the internet so your mobile device can interact with it (for issuance or verification), point an ngrok domain to http://localhost:8080.

Set up an [ngrok](https://ngrok.com/) account if you don't already have one then [install and configure the cli](https://dashboard.ngrok.com/get-started/setup/macos)

Create a `.env` file in the root folder of this workspace and add the following:

```shell
RUST_LOG=debug # or exclude to have info as default
VERCRE_HTTP_ADDR=<your ngrok url>
```

Build and run containers

```shell
docker compose build
docker compose up
```

Point ngrok to your instance of the API:

```shell
ngrok config add-authtoken <your authtoken>
ngrok http --url=<your url> 8080
```

Test ngrok is working by navigating to the root of that URL (browser/curl/Postman, etc) on a separate device and you should receive a valid response.


## Additional

[![ci](https://github.com/vercre/wallet/actions/workflows/ci.yaml/badge.svg)](https://github.com/vercre/wallet/actions/workflows/ci.yaml)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE-MIT)
[![License](https://img.shields.io/badge/license-Apache-blue.svg)](./LICENSE-APACHE)

More information about [contributing][CONTRIBUTING]. Please respect we maintain this project on a
part-time basis. While we welcome suggestions and technical input, it may take time to respond.

The artefacts in this repository are dual licensed under either:

- MIT license ([LICENSE-MIT] or <http://opensource.org/licenses/MIT>)
- Apache License, Version 2.0 ([LICENSE-APACHE] or <http://www.apache.org/licenses/LICENSE-2.0>)

The license applies to all parts of the source code, its documentation and supplementary files
unless otherwise indicated.

[OpenID for Verifiable Credential Issuance]: https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0.html
[OpenID for Verifiable Presentations]: https://openid.net/specs/openid-4-verifiable-presentations-1_0.html
[CONTRIBUTING]: CONTRIBUTING.md
[LICENSE-MIT]: LICENSE-MIT
[LICENSE-APACHE]: LICENSE-APACHE
