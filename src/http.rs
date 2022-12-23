use std::collections::HashMap;

/// HTTP methods supported. All methods except GET and HEAD will be redirected to a user-chosen
/// program via a CLI arg or return `405 Not Allowed`. The following methods are unsupported and
/// will return `405 Not Allowed`:
///
/// - CONNECT.
/// - TRACE.
#[derive(Debug)]
#[derive(PartialEq)]
pub enum HttpMethod {
    OPTIONS,
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    // TRACE,
    // CONNECT,
    PATCH,
}

impl ToString for HttpMethod {
    fn to_string(&self) -> String {
        match self {
            HttpMethod::OPTIONS => String::from("OPTIONS"),
            HttpMethod::GET     => String::from("GET"),
            HttpMethod::HEAD    => String::from("HEAD"),
            HttpMethod::POST    => String::from("POST"),
            HttpMethod::PUT     => String::from("PUT"),
            HttpMethod::DELETE  => String::from("DELETE"),
            HttpMethod::PATCH   => String::from("PATCH"),
        }
    }
}

/// HTTP reponse codes supported. The following codes shouldn't ever be sent:
///
/// - `101 Switching Protocols`: The server won't switch protocol. It's HTTP only;
/// - `103 Early Hints`: Just not implemented as it's experimental;
/// - `300 Multiple Choices`: Just not implemented because It's never used;
/// - `304 Not Modified`: The server doesn't implement cache as it's meant to serve development
/// pages that're constantly changing;
/// - `402 Payment Required`: Just not implemented as it's experimental;
/// - `406 Not Acceptable`: The server doesn't change behaviour based on `Accept`,
/// `Accept-Encoding` or `Accept-Language` values. It'll always serve the defaults;
/// - `408 Request Timeout`: The server simply shutdowns the connection without sending the code;
/// - `410 Gone`: Will always send `404 Not Found`.
/// - `425 Too Early`: Early data is not implemented.
/// - `426 Upgrade Required`: The server will not change from HTTP 1.1.
///
/// The following codes will be sent only if a user-chosen program creates a response with it, but
/// are available to create for users wishing to use the lib crate and not the binary:
/// - `100 Continue`;
/// - `201 Created`;
/// - `202 Accepted`;
/// - `203 Non Authoritative Information`;
/// - `205 Reset Content`;
/// - `303 See Other`;
/// - `407 Proxy Authentication Required`;
/// - `409 Conflict`;
/// - `411 Lenght Required`;
/// - `412 Precondition Failed`;
/// - `413 Payload Too Large`;
/// - `414 URI Too Long`;
/// - `416 Range Not Satisfiable`;
/// - `417 Expectation Failed`;
/// - `422 Unprocessable Entity`;
/// - `428 Precondition Required`;
/// - `429 Too Many Requests`;
/// - `431 Request Header Fields Too Large`;
/// - `451 Unavailable For Legal Reasons`;
/// - `500 Internal Server Error`;
/// - `502 Bad Gateway`;
/// - `503 Service Unavailable`;
/// - `504 Gateway Timeout`;
/// - `506 Variant Also Negotiates`;
/// - `507 Insufficient Storage`;
/// - `508 Loop Detected`;
/// - `510 Not Extended`;
/// - `511 Network Authentication Required`.
///
/// Note that user-defined programs can create any response, so they can create unsupported ones,
/// but shouldn't.
#[derive(Debug)]
#[derive(PartialEq)]
pub enum HttpResponseCode {
    //
    // informational
    //
    Continue100,
    // SwitchingProtocols101,
    Processing102,
    // EarlyHints103,

    //
    // success
    //
    OK200,
    Created201,
    Accepted202,
    NonAuthoritativeInformation203,
    NoContent204,
    ResetContent205,
    PartialContent206,
    MultiStatus207,
    AlreadyReported208,
    IMUsed226,

    //
    // redirection
    //
    // MultipleChoices300,
    MovedPermanently301,
    Found302,
    SeeOther303,
    NotModified304,
    UseProxy305,
    TemporaryRedirect307,
    PermanentRedirect308,

    //
    // client error
    //
    BadRequest400,
    Unauthorized401,
    // PaymentRequired402,
    Forbbiden403,
    NotFound404,
    MethodNotAllowed405,
    // NotAcceptable406,
    ProxyAuthenticationRequired407,
    // RequestTimeout408,
    Conflict409,
    // Gone410,
    LengthRequired411,
    PreconditionFailed412,
    PayloadTooLarge413,
    URITooLong414,
    UnsupportedMediaType415,
    RangeNotSatisfiable416,
    ExpectationFailed417,
    ImATeapot418,
    MisdirectedRequest421,
    UnprocessableEntity422,
    Locked423,
    FailedDependency424,
    // TooEarly425,
    // UpgradeRequired426,
    PreconditionRequired428,
    TooManyRequests429,
    RequestHeaderFieldsTooLarge431,
    UnavailableForLegalReasons451,
    NotImplemented501,
    BadGateway502,
    ServiceUnavailable503,
    GatewayTimeout504,
    HttpVersionNotSupported505,
    VariantAlsoNegotiates506,
    InsufficientStorage507,
    LoopDetected508,
    NotExtend510,
    NetworkAuthenticationRequired511,
}

impl ToString for HttpResponseCode {
    fn to_string(&self) -> String {
        match self {
            HttpResponseCode::Continue100                      => String::from("100 Continue"),
            HttpResponseCode::Processing102                    => String::from("102 Processing"),
            HttpResponseCode::OK200                            => String::from("200 Ok"),
            HttpResponseCode::Created201                       => String::from("201 Created"),
            HttpResponseCode::Accepted202                      => String::from("202 Accepted"),
            HttpResponseCode::NonAuthoritativeInformation203   => String::from("203 Non Authoritative Information"),
            HttpResponseCode::NoContent204                     => String::from("204 No Content"),
            HttpResponseCode::ResetContent205                  => String::from("205 Reset Content"),
            HttpResponseCode::PartialContent206                => String::from("206 Partial Content"),
            HttpResponseCode::MultiStatus207                   => String::from("207 Multi Status"),
            HttpResponseCode::AlreadyReported208               => String::from("208 Already Reported"),
            HttpResponseCode::IMUsed226                        => String::from("226 I'm Used"),
            HttpResponseCode::MovedPermanently301              => String::from("301 Moved Permanently"),
            HttpResponseCode::Found302                         => String::from("302 Found"),
            HttpResponseCode::SeeOther303                      => String::from("303 See Other"),
            HttpResponseCode::NotModified304                   => String::from("304 Not Modified"),
            HttpResponseCode::UseProxy305                      => String::from("305 Use Proxy"),
            HttpResponseCode::TemporaryRedirect307             => String::from("307 Temporary Redirect"),
            HttpResponseCode::PermanentRedirect308             => String::from("308 Permanent Redirect"),
            HttpResponseCode::BadRequest400                    => String::from("400 Bad Request"),
            HttpResponseCode::Unauthorized401                  => String::from("401 Unauthorized"),
            HttpResponseCode::Forbbiden403                     => String::from("403 Forbidden"),
            HttpResponseCode::NotFound404                      => String::from("404 Not Found"),
            HttpResponseCode::MethodNotAllowed405              => String::from("405 Method Not Allowed"),
            HttpResponseCode::ProxyAuthenticationRequired407   => String::from("407 Proxy Authentication Required"),
            HttpResponseCode::Conflict409                      => String::from("409 Conflict"),
            HttpResponseCode::LengthRequired411                => String::from("411 Lenght Required"),
            HttpResponseCode::PreconditionFailed412            => String::from("412 Precondition Failed"),
            HttpResponseCode::PayloadTooLarge413               => String::from("413 Payload Too Large"),
            HttpResponseCode::URITooLong414                    => String::from("414 URI Too Long"),
            HttpResponseCode::UnsupportedMediaType415          => String::from("415 Unsupported Media Type"),
            HttpResponseCode::RangeNotSatisfiable416           => String::from("416 Range Not Satisfiable"),
            HttpResponseCode::ExpectationFailed417             => String::from("417 Expectation Failed"),
            HttpResponseCode::ImATeapot418                     => String::from("418 I'm A Teapot"),
            HttpResponseCode::MisdirectedRequest421            => String::from("421 Misdirected Request"),
            HttpResponseCode::UnprocessableEntity422           => String::from("422 Unprocessable Entity"),
            HttpResponseCode::Locked423                        => String::from("423 Locked"),
            HttpResponseCode::FailedDependency424              => String::from("424 Failed Dependency"),
            HttpResponseCode::PreconditionRequired428          => String::from("428 Precondition Required"),
            HttpResponseCode::TooManyRequests429               => String::from("429 Too Many Requests"),
            HttpResponseCode::RequestHeaderFieldsTooLarge431   => String::from("431 Request Header Fields Too Large"),
            HttpResponseCode::UnavailableForLegalReasons451    => String::from("451 Unavailable For Legal Reasons"),
            HttpResponseCode::NotImplemented501                => String::from("501 Not Implemented"),
            HttpResponseCode::BadGateway502                    => String::from("502 Bad Gateway"),
            HttpResponseCode::ServiceUnavailable503            => String::from("503 Service Unavailable"),
            HttpResponseCode::GatewayTimeout504                => String::from("504 Gateway Timeout"),
            HttpResponseCode::HttpVersionNotSupported505       => String::from("505 HTTP Version Not Supported"),
            HttpResponseCode::VariantAlsoNegotiates506         => String::from("506 Variant Also Negotiates"),
            HttpResponseCode::InsufficientStorage507           => String::from("507 Insufficient Storage"),
            HttpResponseCode::LoopDetected508                  => String::from("508 Loop Detected"),
            HttpResponseCode::NotExtend510                     => String::from("510 Not Extend"),
            HttpResponseCode::NetworkAuthenticationRequired511 => String::from("511 Network Authentication Required"),
        }
    }
}

pub struct HttpRequest {
    pub content: String,
    pub method: HttpMethod,
    pub fetch: String,
    pub headers: HashMap<String, String>,
}

pub struct HttpResponse {
    pub content: String,
    pub code: HttpResponseCode,
    pub headers: HashMap<String, String>,
}
