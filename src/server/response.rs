use iron::Response;
use iron::modifier::Modifier;
use hyper::status::StatusCode;
use serde::Serialize;
use serde_json;

#[derive(Debug, Serialize)]
pub struct APIResponse<T: Serialize> {
    pub payload: T,
}

impl <T> APIResponse<T> where T: Serialize {
    pub fn with(code: StatusCode, response: APIResponse<T>) -> Response {
        let mut res = Response::with(code);
        response.modify(&mut res);
        res
    }
}

impl <T> Modifier<Response> for APIResponse<T> where T: Serialize {
    fn modify(self, res: &mut Response) {
        let data = serde_json::to_string(&self).unwrap();
        data.modify(res);
    }
}