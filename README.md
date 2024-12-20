# Cultivators Rust LTI tool

Cultivators (or "cuLTIvatoRS") is a Rust-based example LTI 1.3 tool. Its focus is on illustrating the implementation of an basic LTI tool, and ease of installation. It comes with a CLI to do some related rudimentary tasks.

[LTI](https://www.imsglobal.org/activity/learning-tools-interoperability) is a standard for integrating third-party tools with Learning Management Systems (LMSs). [LTI 1.3](https://www.imsglobal.org/spec/lti/v1p3) is the latest version of the standard, and it is based on [OAuth2 and JWTs](https://www.imsglobal.org/spec/security/v1p0), and uses a flow called [Dynamic Registration](https://www.imsglobal.org/spec/security/v1p0/#dynamic-registration) for users to initiate setup of tools within an LMS.

## Quickstart

```
git clone https://github.com/evanbattaglia/cultivators
cd cultivators
cargo run app # over HTTP, for use with locally running LMS
cargo run app --tls # over HTTPs w/ self-signed cert, for deployed LMSs
```

or with nix flakes:

```
nix run github:evanbattaglia/cultivators -- app
# or:
nix run github:evanbattaglia/cultivators -- app --tls
```

With the default [compiled-in config](default.env), also available in `cultivators show-default-env`. Cultivators will:
* listen on [http://localhost:3330](http://localhost:3330/), or [https://localhost:3330](https://localhost:3330/) if `--tls` is used
* create a `cultivators.sqlite` database in the current directory to registrations and nonces
* generate a private key and save it as `cultivators.private.pem` -- public jwks are avaliable at `/jwks`
 
The Dynamic Registration URL is served at `/install`; in many cases you can simply run the app with the defaults and paste `http://localhost:3330/install` or `https://localhost:3330/install` into the LMS's LTI Dynamic Registration page.

## HTTPS
If using deploy env, run with `--tls` or the `CULTIVATORS_USE_TLS=1` env var and use `https://localhost:3330/install` for your registration URL.

To make your browser accept the self-signed cert, you can usually do one of the following:
* Go to cultivators URL (e.g.) [https://localhost:3330/](https://localhost:3330/) in your browser directly (not in an iframe), and on the error page, click "Advanced" in the browser SSL error screen, and then click "Accept the Risk and Continue" (Firefox) or "Proceed to 127.0.0.1 (unsafe)" (Chromium).
* For some versions of Chrome, you may need to type `thisisunsafe` anywhere on the error page to bypass the warning.

TODO: generate with self-signed root cert that can be added to browser

## Tool JWKs (for LTI Advantage)
With the default config, Cultivators will look for a `cultivators.private.pem`
file; if it doesn't find that, it will generate a private key and save it to
this location. This path can be overridden with the
`CULTIVATORS_PRIVATE_KEY_PEM_FILE` env var. Once the private key is generated,
you can get the public JWKs at `/jwks` or with the the CLI's `gen jwkset`
command.

If your LMS requires a valid JWKs URI, but your LMS cannot talk to your app
(for instance, you're using a deployed or dockerized LMS but a local
Cultivators tool), you'll need to upload your *public* JWK set to public URL,
such as a github gist, and set `CULTIVATORS_JWKS_URI` accordingly. Note that
you'll need to have this set correctly _at the time of registration with the
platform_. If you change `CULTIVATORS_JWKS_URI` after going through a dynamic
registration flow with the platform, you'll need to go through that flow
again.

For example, once you have generate a key by running the app, you can get the
jwk set and create a gist with (using Github's `gh` CLI);

```
cargo run --bin cli gen jwkset | \
gh gist create -f public.jwks -p -w \
-d "Cultivators LTI tool public JWK set (development)"
```

You can then add the URL to the gist to the `CULTIVATORS_JWKS_URI` env var in
your `.env` file.

Note that for Canvas, a valid JWKs URI this is only necessary when Canvas receives signed JWTs from the tool, that is, for use with Deep Linking, AGS, and other LTI Advantage services. (As of Dec 2024, these aren't implemented in Cultivators yet.)

# Developing

I recommend using `bacon` for development, as well as the following. (These are also included in the `nix shell`):

* Installed with cargo install/bininstall:
  * bacon
  * sea-orm-cli
* Installed with rustup component:
  * clippy
  * rustfmt
  * rust-analyzer and rust-src (for LSP)

## Code structure
* Uses [SeaORM](https://www.sea-ql.org/SeaORM/) with a SQLite database -- see [models](./entity), [migrations](./migration), and [setup](src/seaorm_setup.rs).
* Uses an [axum](https://github.com/tokio-rs/axum) webserver -- see [src/app.rs](./src/app.rs) for server set up and [src/app_routes](./src/app_routes) for route handlers.
* [clap](https://github.com/clap-rs/clap)-based CLI see [`run_cli_command`](src/cli/mod.rs) for routing the the CLI commands.
* [src/lti_models](./src/lti_models) contains [Serialize](https://docs.rs/serde/latest/serde/trait.Serialize.html)-able structs defining the shape of LTI JSON request/response payloads used with [serde_json](https://docs.rs/serde_json/latest/serde_json/).
* [src/lti_app_models](./src/lti_app_models) contains some Cultivators-specific utilities/abstractions over the LTI models and/or SeaORM models.
