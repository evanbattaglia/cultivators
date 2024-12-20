use clap::Parser;
#[derive(Parser, Debug)]
#[clap(
    name = "Cultivators CLI",
    version = env!("CARGO_PKG_VERSION"),
    author = "Evan Battaglia",
    about =
        "Cultivators LTI 1.3 test tool with swiss-army-knife CLI\n\n\
        Cultivators is an LTI 1.3 test tool written in Rust. It aims to be the easiest LTI 1.3 tool to set up, with reasonable default settings, as well as providing a debugging tools useful in LTI tool development and running the app.\n\n\
        Run `cultivators app` to start the app.
        Run `cultivators show-default-env` to show the default settings compiled into the cultivators binary. You can override these in a .env file or by setting environment variables."
)]
pub struct Args {
    #[clap(subcommand)]
    pub(super) command: Commands,
}

#[derive(Parser, Debug)]
pub(super) enum Commands {
    #[clap(name = "app", about = "Run the Cultivators web app")]
    App(App),
    #[clap(name = "regs", about = "Manage LTI tool registrations")]
    Reg {
        #[clap(subcommand)]
        subcommand: Reg,
    },
    #[clap(
        name = "gen",
        about = "Generate various artifacts / JSON blobs relatedto LTI tools or the Cultivators LTI tool in particular"
    )]
    Gen {
        #[clap(subcommand)]
        subcommand: Gen,
    },
    #[clap(
        name = "req",
        about = "Simulate requests to the tool, passing in the same arguments a user agent would"
    )]
    Req {
        #[clap(subcommand)]
        subcommand: Req,
    },
    #[clap(name = "jwt", about = "Jwt related commands")]
    Jwt {
        #[clap(subcommand)]
        subcommand: CliJwt,
    },
    #[clap(
        name = "show-default-env",
        about = "Show default env compiled into Cultivators binary (which is overridable by using a .env file)"
    )]
    ShowDefaultEnv,
}

#[derive(Parser, Debug)]
pub struct App {
    #[clap(
        long = "tls",
        help = "Use TLS (also settable with CULTIVATORS_USE_TLS=true env var)"
    )]
    pub tls: bool,
}

#[derive(Parser, Debug)]
pub(super) enum Reg {
    #[clap(name = "list", about = "List all registrations")]
    List,
    #[clap(name = "delete", about = "Delete a registration")]
    Delete(RegDelete),
}

#[derive(Parser, Debug)]
pub(super) struct RegDelete {
    #[clap(help = "UUID of registration to delete")]
    pub(super) uuid: String,
}

#[derive(Parser, Debug)]
pub(super) enum Gen {
    #[clap(
        name = "key",
        about = "Generate RSA private key -- for use as private.pem file for tool"
    )]
    Key,
    #[clap(
        name = "jwkset",
        about = "Generate JWK from RSA private key -- for use as public key in LTI tool"
    )]
    JwkSet(GenJwkSet),
    #[clap(name = "dynreg", about = "Generate Tool Dynamic Registration")]
    DynReg(GenDynReg),
}

#[derive(Parser, Debug)]
pub(super) enum CliJwt {
    #[clap(name = "val", about = "Validate a jwt")]
    Val(CliJwtVal),
}

#[derive(Parser, Debug)]
pub(super) struct CliJwtVal {
    #[clap(
        long = "key-file",
        help = "Use a private or public key other than our own"
    )]
    pub(super) key_file: Option<String>,
    #[clap(long = "jwt-file", help = "Load JWT data from file rather than STDIN")]
    pub(super) jwt_file: Option<String>,
    #[clap(
        long = "audience",
        help = "aud value -- pass blank string to skip audience checking"
    )]
    pub(super) audience: String,
    #[clap(long = "ignore-expiry", help = "don't validate expiry")]
    pub(super) ignore_expiry: bool,
}

#[derive(Parser, Debug)]
pub(super) struct GenJwkSet {
    #[clap(
        short,
        long,
        help = "Path to private key file -- defaults to the one used by LTI tool (CULTIVATORS_PRIVATE_KEY_PEM_FILE or compiled-in default.private.pem)"
    )]
    pub(super) private_key: Option<String>,
}

#[derive(Parser, Debug)]
pub(super) struct GenDynReg {}

#[derive(Parser, Debug)]
pub(super) enum Req {
    #[clap(name = "login", about = "Login to a registration")]
    Login(ReqLogin),
}

#[derive(Parser, Debug)]
pub(super) struct ReqLogin {
    #[clap(help = "UUID of registration (see 'regs list' command)")]
    pub(super) registration_uuid: String,
    #[clap(
        help = "Client ID of registration (see 'regs list' command), must match registration's client ID"
    )]
    pub(super) client_id: String,
}
