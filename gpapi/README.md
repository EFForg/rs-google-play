<!--- `README.md` is automatically generated from the rustdoc using [`cargo-readme`](https://crates.io/crates/cargo-readme). -->
# `gpapi` - Google Play API for Rust

[![crates.io](https://img.shields.io/crates/v/gpapi.svg)](https://crates.io/crates/gpapi)
[![Documentation](https://docs.rs/gpapi/badge.svg)](https://docs.rs/gpapi)
[![MIT licensed](https://img.shields.io/crates/l/gpapi.svg)](./LICENSE)
[![CI](https://github.com/EFForg/rs-google-play/actions/workflows/ci.yml/badge.svg)](https://github.com/EFForg/rs-google-play/actions/workflows/ci.yml)

A library for interacting with the Google Play API.

## Getting Started

There are two ways to authenticate with the Google Play API:

### Option 1: Using OAuth Token (recommended for personal accounts)

First, obtain an OAuth token by visiting the Google
[embedded setup page](https://accounts.google.com/EmbeddedSetup)
and opening the browser debugging console, logging in, and looking for the `oauth_token` cookie
being set on your browser. It will be present in the last requests being made and start with
"oauth2_4/". Copy this value. It can only be used once, in order to obtain the `aas_token`,
which can be used subsequently. To obtain this token:

```rust
use gpapi::Gpapi;

#[tokio::main]
async fn main() {
    let mut api = Gpapi::new("ad_g3_pro", &email);
    api.request_aas_token(oauth_token).await.unwrap();
    println!("{:?}", api.get_aas_token());
}
```

Now, you can begin interacting with the API by setting the `aas_token` and logging in.

```rust
use gpapi::Gpapi;

#[tokio::main]
async fn main() {
    let mut api = Gpapi::new("px_7a", &email);
    api.set_aas_token(aas_token);
    api.login().await.unwrap();
    // do something
}
```

### Option 2: Using AUTH Token directly (for token dispensers)

If you have an AUTH token (e.g., from Aurora Store's token dispenser), you can use it directly
without going through the OAuth/AAS token flow:

```rust
use gpapi::Gpapi;

#[tokio::main]
async fn main() {
    let mut api = Gpapi::new("px_7a", &email);
    api.set_auth_token(auth_token); // AUTH tokens typically start with "ya29."
    api.login().await.unwrap();
    // do something
}
```

## Downloading Apps

From here, you can get package details, get the info to download a package, or use the library to download it.

```rust
let details = api.details("com.instagram.android").await;
println!("{:?}", details);

let download_info = api.get_download_info("com.instagram.android", None).await;
println!("{:?}", download_info);

api.download("com.instagram.android", None, true, true, true, &Path::new("/tmp/testing"), None).await;
```

## Docs

Documentation for this crate can be found on [docs.rs](https://docs.rs/gpapi/).

## Todo

This inludes some subset, but not all, of the Google Play API library. Some of the functionality  is missing, such as browsing and searching for packages.

## Credits

This library was originally created by David Weinstein, and is currently maintained by Bill Budington.

It follows some of the conventions set by Aurora's [gplayapi java library](https://gitlab.com/AuroraOSS/gplayapi/).

License: MIT
