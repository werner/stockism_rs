use iron::status;
use iron::prelude::*;
use serde::ser::Serialize;
use serde_json;

use iron::mime::Mime;

fn json_content_type() -> Mime {
    "application/json; charset=utf-8".parse::<Mime>().unwrap()
}

macro_rules! create_http_response {
	($name:ident, $status:expr, "to_json_success") => {
		#[allow(dead_code)]
		pub fn $name() -> IronResult<Response> {
			return Ok(Response::with((
                json_content_type(),
				$status, 
				json!({"success": true}).to_string()
			)));
		}
	};
	($name:ident, $status:expr, "to_json_error") => {
		#[allow(dead_code)]
		pub fn $name<S: Into<String>>(text: S) -> IronResult<Response> {
			return Ok(Response::with((
                json_content_type(),
				$status, 
				json!({"success": false, "message": text.into()}).to_string()
			)));
		}
	};
	($name:ident, $status:expr, "to_json") => {
		#[allow(dead_code)]
		pub fn $name<S: Serialize>(response: &S) -> IronResult<Response> {
			let json_text = serde_json::to_string(response).unwrap();
			return Ok(Response::with((
                json_content_type(),
				$status, 
				json_text
			)));
		}
	};
	($name:ident, $status:expr, "text") => {
		#[allow(dead_code)]
		pub fn $name<S: Into<String>>(text: S) -> IronResult<Response> {
			return Ok(Response::with((
                json_content_type(),
				$status, 
				text.into()
			)));
		}
	};
}

macro_rules! get_body {
	($req:expr, $error_fn:ident) => {
        {
            let mut payload = String::new();

            if let Err(_) = $req.body.read_to_string(&mut payload) {
                return $error_fn("Request body not found")
            }

            payload
        }
	}
}

create_http_response!(response_ok, status::Ok, "to_json");
create_http_response!(response_ok_text, status::Ok, "text");

create_http_response!(response_ok_success, status::Ok, "to_json_success");
create_http_response!(response_not_found, status::NotFound, "to_json_error");
create_http_response!(response_bad_request, status::BadRequest, "to_json_error");
create_http_response!(response_internal_server_error, status::InternalServerError, "to_json_error");