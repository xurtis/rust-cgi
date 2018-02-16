//! This crate provides a simple interface for woring within a CGI environment.
//! It also provides tools to emulate that environment from the command line
//! that get built into the generated binary.

extern crate mime;
extern crate url;

mod content;
mod error;

pub use content::*;
pub use error::*;
use std::net;
use std::sync::atomic::{AtomicBool, Ordering};
use url::Url;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Method {
    Options,
    Get,
    Head,
    Post,
    Put,
    Delete,
    Trace,
    Connect,
}

/// The request data as pulled from the environment.
pub struct Request {
    /// HTTP version (`SERVER_PROTOCOL`).
    http_version: String,
    /// CGI version (`GATEWAY_INTERFACE`).
    cgi_version: String,
    /// (`REQUEST_METHOD`).
    method: Method,
    /// The complete composed URL of the request
    /// (http + `SERVER_NAME` + `SERVER_PORT` + `PATH_INFO` + `QUERY_STRING`).
    /// Path to script if running from command line.
    full_url: Url,
    /// The path after the URL to the CGI script (`PATH_INFO`).
    path_info: Option<String>,
    /// The translation of the info_path relative to the document root (`PATH_TRANSLATED`).
    path_translated: Option<String>,
    /// Query section of the URL up to and including the cgi binary (`SCRIPT_NAME`).
    script: String,
    /// Remote user authenticated by the server (`REMOTE_USER`).
    user: Option<String>,
    /// Remote identity authenticated by the server (`REMOTE_IDENT`).
    ident: Option<String>,
    /// Authentication type used (`AUTH_TYPE`).
    auth: Option<String>,
    /// Client connection information (`REMOTE_HOST`).
    client: net::Ipv4Addr,
    /// Processed content from POST or PUT,
    content: Option<Content>,
}

impl Request {
    /// Load a request from the execution environment.
    ///
    /// Note that this will consume the headers sent to the script as well as the contents of the
    /// POST or PUT requests, so this should only be used once.
    ///
    /// Will return errors if more than one attempt is made to load a request.
    pub fn load() -> Result<Request, Error> {

        /// Only load once.
        static request_loaded: AtomicBool = AtomicBool::new(false);
        if request_loaded.swap(true, Ordering::Acquire) {
            return Err(Error::MultipleLoad);
        }

        unimplemented!()
    }

    /// Get the url for a request.
    pub fn url(&self) -> &Url {
        &self.full_url
    }

    /// Update the composite path in the URL.
    fn update_path(&mut self) {
        let mut composite_path = self.script.to_string();
        if let &Some(ref path_info) = &self.path_info {
            composite_path.push_str(path_info);
        }
        self.full_url.set_path(&composite_path);
    }
}

impl Default for Request {
    fn default() -> Request {
        use std::env::args;

        Request {
            http_version: "HTTP/1.0".to_string(),
            cgi_version: "CGI/1.1".to_string(),
            method: Method::Get,
            full_url: script_url(),
            path_info: None,
            path_translated: None,
            script: args().next().expect("Could not get script name"),
            user: None,
            ident: None,
            auth: None,
            client: net::Ipv4Addr::new(127, 0, 0, 1),
            content: None,
        }
    }
}

/// Build queries from components.
///
/// Useful for creating dummy queries for tests.
pub struct Builder {
    request: Request,
}

impl Builder {

    /// Create a new request builder.
    pub fn new() -> Builder {
        Builder { request: Request::default() }
    }

    /// Build into actual request.
    pub fn build(self) -> Request {
        self.request
    }

    /// Set the HTTP protocol version.
    pub fn http_version(mut self, version: &str) -> Builder {
        self.request.http_version = version.to_string();
        self
    }

    /// Set the CGI protocol version.
    pub fn cgi_version(mut self, version: &str) -> Builder {
        self.request.cgi_version = version.to_string();
        self
    }

    /// Set the request method.
    ///
    /// Content can only be set for Put and Post requests.
    pub fn method(mut self, method: Method) -> Builder {
        self.request.method = method;
        if !(method == Method::Put || method == Method::Post) {
            self.request.content = None;
        }
        self
    }

    /// Set the request to Get.
    pub fn get(mut self) -> Builder {
        self.request.method = Method::Get;
        self
    }

    /// Set the request to Put and set the content.
    pub fn put(mut self, content: Option<Content>) -> Builder {
        self.request.method = Method::Put;
        self.request.content = content;
        self
    }

    /// Set the request to Post and set the content.
    pub fn post(mut self, content: Option<Content>) -> Builder {
        self.request.method = Method::Post;
        self.request.content = content;
        self
    }

    /// Set the server hostname.
    pub fn host(mut self, host: &str) -> Builder {
        self.request.full_url.set_host(Some(host));
        self
    }

    /// Set the server port and change the schema to http.
    pub fn port(mut self, port: u16) -> Builder {
        self.request.full_url.set_scheme("http");
        self.request.full_url.set_port(Some(port));
        self
    }

    /// Set the script path.
    pub fn script(mut self, script: &str) -> Builder {
        self.request.script = script.to_string();
        self.request.update_path();
        self
    }

    /// Set the path info.
    pub fn path_info(mut self, base: &str, info: &str) -> Builder {
        self.request.path_info = Some(info.to_string());
        let mut translated = base.to_string();
        translated.push_str(info);
        self.request.path_translated = Some(translated);
        self.request.update_path();
        self
    }

    /// Set the user and authentication method accessing the query.
    ///
    /// Removes any ident that has been set.
    pub fn user(mut self, user: &str, method: Option<&str>) -> Builder {
        self.request.user = Some(user.to_string());
        self.request.auth = method.map(str::to_string);
        self.request.ident = None;
        self
    }

    /// Set the ident and authentication method accessing the query.
    ///
    /// Removes any user that has been set.
    pub fn ident(mut self, ident: &str, method: Option<&str>) -> Builder {
        self.request.ident = Some(ident.to_string());
        self.request.auth = method.map(str::to_string);
        self.request.user = None;
        self
    }

    /// Set the client's IP address.
    pub fn client(mut self, client: net::Ipv4Addr) -> Builder {
        self.request.client = client;
        self
    }

    /// Set the query string used in the URL.
    pub fn query(mut self, query: &str) -> Builder {
        self.request.full_url.set_query(Some(query));
        self
    }
}

/// Get the file URL for the running binary.
fn script_url() -> Url {
    use std::env::args;
    use std::fs::canonicalize;

    let script = args().next().expect("Could not get script name");
    let script_path = canonicalize(&script).expect("Could not canonicalize script path");
    Url::from_file_path(script_path).expect("Could not create a url from the script name")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn build_url() {
        let request = Builder::new()
            .host("jabberwocky")
            .port(81)
            .script("/test.cgi")
            .path_info("/var/www/html", "/some/info")
            .query("choice=12")
            .build();

        assert_eq!("http://jabberwocky:81/test.cgi/some/info?choice=12", request.url().as_str());
    }
}
