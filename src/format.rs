// Copyright 2013-2014 Simon Sapin.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Formatting utilities for URLs.
//!
//! These formatters can be used to coerce various URL parts into strings.
//!
//! You can use `<formatter>.to_string()`, as the formatters implement `Show`.

use std::fmt::{self, Show, Formatter};
use super::Url;

/// Formatter and serializer for URL path data.
pub struct PathFormatter<'a, T:'a> {
    /// The path as a slice of string-like objects (String or &str).
    pub path: &'a [T]
}

impl<'a, T: Str + Show> Show for PathFormatter<'a, T> {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        if self.path.is_empty() {
            formatter.write_str("/")
        } else {
            for path_part in self.path.iter() {
                try!("/".fmt(formatter));
                try!(path_part.fmt(formatter));
            }
            Ok(())
        }
    }
}


/// Formatter and serializer for URL username and password data.
pub struct UserInfoFormatter<'a> {
    /// URL username as a string slice.
    pub username: &'a str,

    /// URL password as an optional string slice.
    ///
    /// You can convert an `Option<String>` with `.as_ref().map(|s| s.as_slice())`.
    pub password: Option<&'a str>
}

impl<'a> Show for UserInfoFormatter<'a> {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        if !self.username.is_empty() || self.password.is_some() {
            try!(formatter.write_str(self.username));
            match self.password {
                None => (),
                Some(password) => {
                    try!(formatter.write_str(":"));
                    try!(formatter.write_str(password));
                }
            }
            try!(formatter.write_str("@"));
        }
        Ok(())
    }
}


/// Formatter for URLs which ignores the fragment field.
pub struct UrlNoFragmentFormatter<'a> {
    pub url: &'a Url
}

impl<'a> Show for UrlNoFragmentFormatter<'a> {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        try!(formatter.write_str(self.url.scheme.as_slice()));
        try!(formatter.write_str(":"));
        try!(self.url.scheme_data.fmt(formatter));
        match self.url.query {
            None => (),
            Some(ref query) => {
                try!(formatter.write_str("?"));
                try!(formatter.write_str(query.as_slice()));
            }
        }
        Ok(())
    }
}


/// Formatting Tests
#[cfg(test)]
mod tests {
    use super::super::Url;
    use super::{PathFormatter, UserInfoFormatter};

    #[test]
    fn path_formatting() {
        let data = [
            (vec![], "/"),
            (vec![""], "/"),
            (vec!["test", "path"], "/test/path"),
            (vec!["test", "path", ""], "/test/path/")
        ];
        for &(ref path, result) in data.iter() {
            assert_eq!(PathFormatter {
                path: path.as_slice()
            }.to_string(), result.to_string());
        }
    }

    #[test]
    fn userinfo_formatting() {
        // Test data as (username, password, result) tuples.
        let data = [
            ("", None, ""),
            ("", Some(""), ":@"),
            ("", Some("password"), ":password@"),
            ("username", None, "username@"),
            ("username", Some(""), "username:@"),
            ("username", Some("password"), "username:password@")
        ];
        for &(username, password, result) in data.iter() {
            assert_eq!(UserInfoFormatter {
                username: username,
                password: password
            }.to_string(), result.to_string());
        }
    }

    #[test]
    fn relative_scheme_url_formatting() {
        let data = [
            ("http://example.com/", "http://example.com/"),
            ("http://addslash.com", "http://addslash.com/"),
            ("http://@emptyuser.com/", "http://emptyuser.com/"),
            ("http://:@emptypass.com/", "http://:@emptypass.com/"),
            ("http://user@user.com/", "http://user@user.com/"),
            ("http://user:pass@userpass.com/", "http://user:pass@userpass.com/"),
            ("http://slashquery.com/path/?q=something", "http://slashquery.com/path/?q=something"),
            ("http://noslashquery.com/path?q=something", "http://noslashquery.com/path?q=something")
        ];
        for &(input, result) in data.iter() {
            let url = Url::parse(input).unwrap();
            assert_eq!(url.to_string(), result.to_string());
        }
    }
}
