
use std::mem;

extern crate httparse;
use httparse::EMPTY_HEADER;
use httparse::Request;
pub use httparse::Header;

///Errors that can occur while parsing
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub enum HttpFault {
    InvalidVersion,
    InvalidMethod,
    InvalidPath,
    HeaderName,
    HeaderValue,
    NewLine,
    Status,
    Token,
    TooManyHeaders,
    Version
}
impl From<httparse::Error> for HttpFault {
    fn from(x: httparse::Error) -> Self {
        match x {
            httparse::Error::HeaderName => HttpFault::HeaderName,
            httparse::Error::HeaderValue => HttpFault::HeaderValue,
            httparse::Error::NewLine => HttpFault::NewLine,
            httparse::Error::Status => HttpFault::Status,
            httparse::Error::Token => HttpFault::Token,
            httparse::Error::TooManyHeaders => HttpFault::TooManyHeaders,
            httparse::Error::Version => HttpFault::Version
        }
    }
}

///The HTTP verb being used
#[derive(Debug,Clone,PartialEq,Eq)]
pub enum Method<'a> {
    Options,
    Get,
    Post,
    Put,
    Delete,
    Head,
    Trace,
    Connect,
    Patch,
    Extension(&'a str)
}
impl<'a> From<&'a str> for Method<'a> {
    fn from(x: &'a str) -> Self {
        match x.trim() {
            "Get"|"GET"|"get" => Method::Get,
            "Options"|"OPTIONS"|"options" => Method::Options,
            "Post"|"POST"|"post" => Method::Post,
            "Put"|"PUT"|"put" => Method::Put,
            "Delete"|"DELETE"|"delete" => Method::Delete,
            "Head"|"HEAD"|"head" => Method::Head,
            "Trace"|"TRACE"|"trace" => Method::Trace,
            "Connect"|"CONNECT"|"connect" => Method::Connect,
            "Patch"|"PATCH"|"patch" => Method::Patch,
            y => Method::Extension(y)
        }
    }
}

///Represents a request
pub struct Http<'a> {
    data: Request<'a,'a>,
    body: &'a [u8]
}
impl<'a> Http<'a> {

    ///Parse a new request.
    pub fn new(data: &'a [u8], headers: &'a mut [Header<'a>])
    -> Result<Option<Http<'a>>,HttpFault> {
        let mut ret_val = Http {
            data: unsafe{mem::zeroed()},
            body: unsafe{mem::zeroed()}
        };
        let mut rq = Request::new(headers);
        let res = match rq.parse(data) {
            Ok(x) => match x {
                httparse::Status::Complete(val) => val,
                httparse::Status::Partial => 0
            },
            Err(e) => return Err(HttpFault::from(e))
        };
        if rq.method.is_none() || rq.path.is_none() || rq.version.is_none(){
            return Ok(None);
        }
        ret_val.data = rq;
        let (_,body) = data.split_at(res);
        ret_val.body = body;
        Ok(Some(ret_val))
    }

    ///Get HTTP Method
    pub fn method(&self) -> Method<'a> {
        match self.data.method {
            Option::None => unreachable!(),
            Option::Some(x) => Method::from(x)
        }
    }

    ///Get Path
    pub fn path(&self) -> &'a str {
        match self.data.path {
            Option::None => unreachable!(),
            Option::Some(x) => x
        }
    }
    
    ///Get Version
    pub fn version(&self) -> u8 {
        match self.data.version {
            Option::None => unreachable!(),
            Option::Some(x) => x
        }
    }

    ///Get headers
    pub fn headers(&'a self) -> &'a [Header<'a>] {
        self.data.headers
    }
}

#[test]
fn test_request() {
    let test_data = b"PUT / HTTP/1.1
Host: mysite.com
Accept: */*
Content-Type: text/html
Content-Length: 16";
    let mut headers = [EMPTY_HEADER;25];
    let item = Http::new(test_data,&mut headers).unwrap().unwrap();

    assert_eq!(Method::Put, item.method() );
    assert_eq!("/", item.path());
    assert_eq!(1, item.version());
    assert_eq!("mysite.com",String::from_utf8_lossy(item.headers()[0].value)); 
}
