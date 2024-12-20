use crate::{app_routes, crypto, env, seaorm_setup};
use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use rcgen::generate_simple_self_signed;
use sea_orm::DatabaseConnection;
use sea_orm::NotSet;
use sea_orm::Set;
use std::net::SocketAddr;
use tracing::info;
use tracing_subscriber::reload::Handle;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::Registry;

struct CertKey {
    cert_pem: Vec<u8>,
    key_pem: Vec<u8>,
}

fn get_tls_certkey_from_files(cert_file: &str, key_file: &str) -> CertKey {
    let cert_pem = std::fs::read(cert_file).unwrap_or_else(|e| {
        panic!("Couldn't open CULTIVATORS_TLS_CERT_PEM_FILE={cert_file:?}: {e:?}")
    });
    let key_pem = std::fs::read(key_file).unwrap_or_else(|e| {
        panic!("Couldn't open CULTIVATORS_TLS_KEY_PEM_FILE={key_file:?}: {e:?}")
    });
    info!("Cultivators TLS: Using cert file {cert_file:?}, key file {key_file:?}");
    CertKey { cert_pem, key_pem }
}

async fn existing_tls_certkey(domain: &str, db: &DatabaseConnection) -> Option<CertKey> {
    use entity::tls_cert::{Column::Domain, Entity as TlsCert};
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

    let cert = TlsCert::find()
        .filter(Domain.eq(domain))
        .one(db)
        .await
        .unwrap()?;
    info!("Using existing TLS cert pem + key pem in database for domain {domain:?}");
    Some(CertKey {
        cert_pem: cert.cert_pem.into_bytes(),
        key_pem: cert.key_pem.into_bytes(),
    })
}

async fn find_or_generate_tls_certkey() -> CertKey {
    // TODO we do this again when setting up the app...
    let db = seaorm_setup::create_db_connection().await.unwrap();
    let domain = std::env::var("CULTIVATORS_TLS_GENERATED_CERT_DOMAIN")
        .unwrap_or_else(|_| crate::env::base_url_domain());

    match existing_tls_certkey(&domain, &db).await {
        Some(certkey) => certkey,
        None => generate_new_tls_certkey(domain, &db).await,
    }
}

async fn generate_new_tls_certkey(domain: String, db: &DatabaseConnection) -> CertKey {
    let cert_hosts = vec![domain.clone()];
    let cert_with_key = generate_simple_self_signed(cert_hosts).unwrap();
    let cert_pem = cert_with_key.cert.pem();
    let key_pem = cert_with_key.key_pair.serialize_pem();
    info!("Cultivators TLS: Using newly generated cert pem + key pem for domain {domain:?}");

    use entity::{tls_cert, tls_cert::Entity as TlsCert};
    use sea_orm::EntityTrait;

    let new_res = tls_cert::ActiveModel {
        id: NotSet,
        domain: Set(domain),
        created_at: Set(chrono::Utc::now()),
        cert_pem: Set(cert_pem.clone()),
        key_pem: Set(key_pem.clone()),
    };
    TlsCert::insert(new_res).exec(db).await.unwrap();

    CertKey {
        cert_pem: cert_pem.into_bytes(),
        key_pem: key_pem.into_bytes(),
    }
}

async fn get_tls_certkey() -> CertKey {
    let cert_env = std::env::var("CULTIVATORS_TLS_CERT_PEM_FILE")
        .ok()
        .filter(|s| !s.is_empty());
    let key_env = std::env::var("CULTIVATORS_TLS_KEY_PEM_FILE")
        .ok()
        .filter(|s| !s.is_empty());
    match (cert_env.as_deref(), key_env.as_deref()) {
        (None, None) => find_or_generate_tls_certkey().await,
        (Some(c), Some(k)) => get_tls_certkey_from_files(c, k),
        _ => panic!("CULTIVATORS_TLS_CERT_PEM_FILE and CULTIVATORS_TLS_KEY_PEM_FILE must either both be set or both not be set")
    }
}

fn bind_addr() -> SocketAddr {
    let addr = std::env::var("CULTIVATORS_BIND_ADDRESS").expect("missing CULTIVATORS_BIND_ADDRESS");
    addr.parse()
        .unwrap_or_else(|e| panic!("error parsing CULTIVATORS_BIND_ADDRESS={addr:?}: {e:?}"))
}

async fn serve_ssl(app: Router, setup_app_logging: impl FnOnce()) {
    rustls::crypto::ring::default_provider()
        .install_default()
        .unwrap();
    let CertKey { cert_pem, key_pem } = get_tls_certkey().await;
    let config = RustlsConfig::from_pem(cert_pem, key_pem).await.unwrap();
    let addr = bind_addr();
    info!("Cultivators running with TLS on https://{addr:?}");
    info!(
        "Dynamic registration URL: {}",
        crate::app_uris::dyn_reg_initiation_uri()
    );
    let server = axum_server::bind_rustls(bind_addr(), config).serve(app.into_make_service());
    setup_app_logging();
    server.await.unwrap();
}

async fn serve_http(app: Router, setup_app_logging: impl FnOnce()) {
    let addr = bind_addr();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    info!("Cultivators running HTTP (without TLS) at http://{addr:?}");
    info!(
        "Dynamic registration URL: {}",
        crate::app_uris::dyn_reg_initiation_uri()
    );
    info!("(re-run with CULTIVATORS_USE_TLS=true or --tls to enable TLS)");
    let server = axum::serve(listener, app);
    setup_app_logging();
    server.await.unwrap();
}

fn env_filter(env_var: &str, default: &str) -> EnvFilter {
    EnvFilter::try_from_env(env_var).unwrap_or_else(|_| EnvFilter::try_new(default).unwrap())
}

fn init_tracing() -> Handle<EnvFilter, Registry> {
    use tracing_subscriber::{fmt, prelude::*, reload};

    // On start up, only show info-level) (mute sql logs for creation of certs etc.)
    let env_filter = env_filter("CULTIVATORS_INIT_LOG", "info");
    let (filter, reload_handle) = reload::Layer::new(env_filter);
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::Layer::default())
        .init();
    reload_handle
}

pub async fn run() {
    env::init_env();
    let reload_handle = init_tracing();
    crypto::files::ensure_tool_keys_loaded();
    let app = app_routes::app().await;

    let setup_app_logging = move || {
        reload_handle
            .modify(|filter| *filter = env_filter("CULTIVATORS_LOG", "info"))
            .unwrap()
    };

    if env::bool_from_env("CULTIVATORS_USE_TLS") {
        serve_ssl(app, setup_app_logging).await;
    } else {
        serve_http(app, setup_app_logging).await;
    }
}
