use crate::models::User::User;

/// The ID extracted from the request string.
pub fn get_id(request: &str) -> &str {
    request.split("/").nth(2).unwrap_or_default().split_whitespace().next().unwrap_or_default()
}

// Deserialize JSON (User) from request body 
pub fn get_user_request_body(request: &str) -> Result<User, serde_json::Error> {
   serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}