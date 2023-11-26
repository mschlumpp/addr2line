#[cfg(not(feature = "sync"))]
pub type LazyCell<T> = core::cell::OnceCell<T>;
#[cfg(feature = "sync")]
pub type LazyCell<T> = std::sync::OnceLock<T>;

