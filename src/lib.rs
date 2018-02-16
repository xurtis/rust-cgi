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
    /// The complete composed URL of the request (http + `SERVER_NAME` + `SERVER_PORT` + `QUERY_STRING`).
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
    auth_type: String,
    /// Client connection information (`REMOTE_HOST`).
    client: net::Ipv4Addr,
    /// Processed content from POST or PUT,
    content: Option<Content>,
    /// Query parameters from GET request (`QUERY_STRING`).
    query: String,
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
}
