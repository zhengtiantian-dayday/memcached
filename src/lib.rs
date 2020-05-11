//!

#![deny(
    // missing_docs,
    bare_trait_objects,
    missing_copy_implementations,
    single_use_lifetimes,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    variant_size_differences,
    unsafe_code,
    trivial_casts,
    // missing_debug_implementations,
//     // 把所有warnings级别的改为deny
    warnings,
    clippy::all,
    clippy::correctness,
    clippy::restriction,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::cargo,
    // clippy::nursery ,
    clippy::pedantic
)]
#![allow(
    clippy::missing_inline_in_public_items,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::missing_docs_in_private_items,
    clippy::implicit_return,
    clippy::as_conversions,
    clippy::dbg_macro,
    clippy::pub_enum_variant_names,
    clippy::multiple_crate_versions,
    clippy::doc_markdown,
    clippy::cast_possible_truncation, //
    clippy::integer_arithmetic, //
)]

mod client;
mod connection;
pub mod error;
mod parse;
mod protocol;
mod stream;

pub type Result<T> = std::result::Result<T, error::MemcachedError>;
pub use client::{connectable::Connectable, Client};

/// Create a memcached client instance and connect to memcached server.
/// The default connection pool has only one connection.
///
/// ## Example
///
/// ```rust
/// let client = memcached::connect("memcache://127.0.0.1:12345").unwrap();
/// ```
pub fn connect(url: &str) -> Result<Client> {
    Client::connect(url)
}

/// Create a client, you can specify multiple url, connection pool size, key hash connection pool function.
///
/// ## Example
///
/// ```rust
/// let client = memcached::Client::connects_with(vec!["memcache://127.0.0.1:12345".to_owned()], 2, |s|1).unwrap();
/// ```
pub fn connects_withconnects_with(
    urls: Vec<String>,
    pool_size: u64,
    hash_function: fn(&str) -> usize,
) -> Result<Client> {
    Client::connects_with(urls, pool_size, hash_function)
}

#[allow(clippy::result_unwrap_used)]
#[cfg(test)]
mod tests {
    #[async_std::test]
    async fn it_works() {
        async_std::task::block_on(async {
            async fn foo() -> crate::Result<()> {
                let _client = crate::connect("memcache://127.0.0.1:12345")?; //.unwrap();
                                                                             // client.set("increment_test", 100, 100).await?;
                                                                             // let t = client.increment("increment_test", 10).await?;
                                                                             // assert_eq!(120, client.increment("increment_test", 10).await?);
                                                                             // let t: Option<u64> = client.get("increment_test").await?;
                                                                             // assert_eq!(t, Some(120));
                Ok(())
            }
            dbg!(foo().await.unwrap());
        });
    }
}
