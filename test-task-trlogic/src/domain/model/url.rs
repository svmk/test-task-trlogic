use std::str::FromStr;
use failure::{Error, err_msg};
use std::convert::{Into, AsRef};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Url(url::Url);

impl Url {
    pub fn as_str(&self) -> &str {
        return self.0.as_str();
    }

    pub fn as_ref(&self) -> &Url {
        return &self;
    }

    pub fn join_path(mut self, path_segment: impl AsRef<str>) -> Result<Url, Error> {
        {
            let mut path_segments = self.0.path_segments_mut()
                .or_else(|_| {
                    return Err(err_msg("Unable to get path segments"));
                })?;
            path_segments.push(path_segment.as_ref());
        }
        return Ok(self);
    }

    pub fn get_domain(&self) -> Option<&str> {
        return self.0.domain();
    }

    pub fn get_path(&self) -> &str {
        return self.0.path();
    }
}

impl From<url::Url> for Url {
    fn from(value: url::Url) -> Self {
        return Url(value);
    }
}

impl <'a>Into<&'a url::Url> for &'a Url {
    fn into(self) -> &'a url::Url {
        return &self.0;
    }
}

impl Into<url::Url> for Url {
    fn into(self) -> url::Url {
        return self.0;
    }
}

impl FromStr for Url {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url = url::Url::from_str(s).map_err(err_msg)?;
        return Ok(Url(url));
    }
}

impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.0)
    }
}