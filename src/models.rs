use std::net::Ipv4Addr;
use url::Url;
use querystring as qs;

use crate::Error;
use crate::SCHEME_AFFIX;

#[derive(Debug, Default)]
pub struct UrlBuilder {
    scheme: String,
    host: String,
    port: Option<u16>,
    path: Option<String>,
    query: Option<String>,
}

pub enum Part<'a> {
    Scheme(&'a str),
    Host(&'a str),
    HostIpv4(Ipv4Addr),
    Port(u16),
    Path(&'a str),
    PathSlice(&'a [&'a str]),
    Query(&'a [(&'a str, &'a str)]),
}

impl UrlBuilder {
    pub fn try_build(self) -> Result<Url, Error> {
        let Self {
            port,
            path,
            query,
            ..
        } = self;

        let mut url = {
            let Self {
                scheme: s,
                host: h,
                ..
            } = self;
            let base_url = [s, h].join(SCHEME_AFFIX);
            Url::parse(&base_url)?
        };

        url.set_port(port).map_err(|_| Error::PortError)?;

        if let Some(path) = path.as_deref() {
            url.set_path(path);
        }

        url.set_query(query.as_deref());

        Ok(url)
    }

    pub fn set(&mut self, part: Part) -> &mut Self {
        match part {
            Part::Scheme(value) => self.scheme = value.to_owned(),
            Part::Host(value) => {
                self.host = value.to_owned();
            },
            Part::HostIpv4(value) => {
                self.host = value.to_string();
            },
            Part::Port(value) => self.port = Some(value),
            Part::Path(value) => self.path = Some(value.into()),
            Part::PathSlice(value) => {
                self.path = Some(value.join("/").into());
            }
            Part::Query(value) => {
                self.query = Some(qs::stringify(value.to_vec()));
            },
        };
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn basic() {
        let scheme = "https";
        let host = Ipv4Addr::new(192, 168, 0, 11);
        let port = 3000_u16;
        let path = &["my", "path", "here"];
        let query = &[
            ("try", "true")
        ];

        let mut builder = UrlBuilder::default();

        builder
            .set(Part::Scheme(scheme))
            .set(Part::HostIpv4(host))
            .set(Part::Port(port))
            .set(Part::PathSlice(path))
            .set(Part::Query(query));

        let result = builder.try_build().unwrap();

        assert_eq!(result.scheme(), scheme);
        assert_eq!(result.host().to_owned().unwrap(), url::Host::parse(&host.to_string()).unwrap());
        assert_eq!(result.port().unwrap(), port);
        assert_eq!(result.path(), "/my/path/here");
        assert_eq!(result.query().unwrap(), "try=true&");
    }
}
