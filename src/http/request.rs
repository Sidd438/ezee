use super::method::METHOD;

pub struct Request {
    method: METHOD,
    path: String,
    body: String,
    query_params: Option<String>
}