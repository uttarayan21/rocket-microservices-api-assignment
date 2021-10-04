use rocket::Request;
use rocket::http::Status;

#[catch(500)]
pub fn internal_error() -> &'static str {
    "Internal Server Error"
}

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("I couldn't find '{}'. Try something else?", req.uri())
}

#[catch(default)]
pub fn default(status: Status, req: &Request) -> String {
    format!("{} ({})", status, req.uri())
}
