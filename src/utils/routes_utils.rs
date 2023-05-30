use actix_web::HttpRequest;

use crate::services::auth_services::validate_and_get_id_from_token;

pub fn extract_token_from_headers(req: &HttpRequest) -> Option<String> {
    match req.headers().get("authorization") {
        None => None,
        Some(auth_header) => {
            let auth_header = String::from(auth_header.to_str().unwrap());
            let token = auth_header.split(" ").collect::<Vec<&str>>()[1].to_string();
            Some(token)
        }
    }
}

pub fn extract_user_id_from_headers(req: &HttpRequest) -> Option<String> {
    let token = match req.headers().get("authorization") {
        None => return None,
        Some(auth_header) => {
            let auth_header = String::from(auth_header.to_str().unwrap());
            let token = auth_header.split(" ").collect::<Vec<&str>>()[1].to_string();
            token
        }
    };

    let user_id = match validate_and_get_id_from_token(&token) {
        Err(_) => return None,
        Ok(id) => id,
    };

    Some(user_id)
}
