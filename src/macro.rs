/// Extremely simple macro that should make it easier to construct urls. 
///
/// ```rust,no_run
/// use url_builder::{
///     Url,
///     Error,
///     Part,
///     UrlBuilder,
///     url_builder,
/// };
///
/// fn main() {
///     let url: Result<Url, Error> = url_builder! {
///         Part::Scheme("http");
///         Part::Host("192.168.0.1");
///         Part::Port(3000);
///     };
/// }
/// ```
#[macro_export]
macro_rules! url_builder {
    ($($var:expr);*$(;)?) => {
        {
            let mut builder = $crate::UrlBuilder::default();
            $(
                builder.set($var);
            )*
            builder.try_build()
        }
    };
}

pub use url_builder;

#[cfg(feature="macros")]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Part;

    #[test]
    fn r#macro() {
        let url = url_builder! {
            Part::Scheme("http");
            Part::Host("192.168.0.1");
            Part::Port(3000);
        };

        assert!(url.is_ok());
        assert_eq!(url.as_ref().unwrap().to_string(), "http://192.168.0.1:3000/");
    }
}

