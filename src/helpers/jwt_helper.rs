use std::env;
use jsonwebtoken::{DecodingKey, Validation, Algorithm, decode, errors::ErrorKind};

use crate::models::dtos::Claims;

pub struct JwtHelper;

impl JwtHelper {
    pub fn decode_jwt(token: String) -> Result<Claims, ErrorKind> {
        let magic: String = env::var("JWT_SECRET").expect("JWT_SECRET is empty.");
    
        let token = token.trim_start_matches("Bearer").trim();
    
        match decode::<Claims>(
            &token,
            &DecodingKey::from_secret(magic.as_bytes()),
            &Validation::new(Algorithm::HS256),
        ) {
            Ok(token) => Ok(token.claims),
            Err(err) => Err(err.kind().to_owned()),
        }
    }
}