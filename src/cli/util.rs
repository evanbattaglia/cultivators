use color_eyre::eyre;

pub(super) fn run_async_block<T>(
    closure: impl std::future::Future<Output = eyre::Result<T>>,
) -> eyre::Result<T> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(closure)
}
