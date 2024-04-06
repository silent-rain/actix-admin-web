use chrono::Local;
use jsonwebtoken::{
    decode, encode, errors, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};

const SECRET: &str = "secret";
const ISS: &str = "silent-rain";
const KID: &str = "silent-rain";
const NBF: usize = 1000 * 60 * 60 * 24; // 1 day
/// Token 过期时间
const EXPIRE: i64 = 1000 * 60 * 60 * 24; // 1 Day

#[derive(Debug, PartialEq)]
pub enum Error {
    /// 发行人
    CheckIss,
    /// 不在此之前（作为UTC时间戳）
    CheckNbf,
    /// 过期时间
    CheckExp,
}

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Claims {
    pub user_id: i32,
    pub username: String,

    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    // aud: String, // Optional. Audience
    // iat: usize, // Optional. Issued at (as UTC timestamp)
    nbf: usize, // Optional. Not Before (as UTC timestamp)
    // sub: String, // Optional. Subject (whom token refers to)
    iss: String, // Optional. Issuer
}

impl Claims {
    fn check_iss(&self) -> Result<&Self, Error> {
        if self.iss != ISS {
            return Err(Error::CheckIss);
        }
        Ok(self)
    }

    fn check_nbf(&self) -> Result<&Self, Error> {
        if self.nbf != NBF {
            return Err(Error::CheckNbf);
        }
        Ok(self)
    }
    fn check_exp(&self) -> Result<&Self, Error> {
        let exp = Local::now().timestamp_millis();
        if self.exp + self.nbf >= exp as usize {
            return Err(Error::CheckExp);
        }
        Ok(self)
    }
    pub fn verify(&self) -> Result<(), Error> {
        let _ = self.check_iss()?.check_nbf()?.check_exp();

        Ok(())
    }
}

/// 编码
pub fn encode_token(user_id: i32, username: String) -> Result<String, errors::Error> {
    let exp = Local::now().timestamp_millis() + EXPIRE;
    let claims = Claims {
        user_id,
        username,
        exp: exp as usize,
        nbf: NBF,
        iss: ISS.to_owned(),
    };
    let mut header = Header::new(Algorithm::HS256);
    header.kid = Some(KID.to_owned());
    let token = encode(&header, &claims, &EncodingKey::from_secret(SECRET.as_ref()))?;
    Ok(token)
}

/// 解码
pub fn decode_token(token: &str) -> Result<Claims, errors::Error> {
    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET.as_ref()),
        &Validation::default(),
    )?
    .claims;

    Ok(claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_utc() {
        let local = Local::now();
        println!("local: {}", local);

        let timestamp = local.timestamp_millis();
        // 1709819375028
        println!("timestamp: {:?}", timestamp);
        assert!(timestamp > 0);
    }

    #[test]
    fn it_encode_token() {
        let token = encode_token(1, "user_name".to_owned());
        println!("token: {:?}", token);
        let expected = Ok("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiIsImtpZCI6InNpbGVudC1yYWluIn0.eyJ1c2VyX2lkIjoxLCJ1c2VybmFtZSI6InVzZXJfbmFtZSIsImV4cCI6MTcwOTgyMjY1ODIwNiwibmJmIjo4NjQwMDAwMCwiaXNzIjoic2lsZW50LXJhaW4ifQ.1DfBZg5p5CVnRrpZ3WWpBNUdh2v7rGB_wPsGy2NSTSU".to_owned());

        assert!(token != expected);
    }

    #[test]
    fn it_decode_token() {
        let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiIsImtpZCI6InNpbGVudC1yYWluIn0.eyJ1c2VyX2lkIjoxLCJ1c2VybmFtZSI6InVzZXJfbmFtZSIsImV4cCI6MTcwOTgyMjY1ODIwNiwibmJmIjo4NjQwMDAwMCwiaXNzIjoic2lsZW50LXJhaW4ifQ.1DfBZg5p5CVnRrpZ3WWpBNUdh2v7rGB_wPsGy2NSTSU";
        let result = decode_token(token).unwrap();
        println!("result: {:?}", result);
        let expected = Claims {
            user_id: 1,
            username: "user_name".to_owned(),
            exp: 1709822658206,
            nbf: NBF,
            iss: ISS.to_owned(),
        };

        assert!(result == expected);
    }

    #[test]
    fn it_decode_token_verify() {
        let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiIsImtpZCI6InNpbGVudC1yYWluIn0.eyJ1c2VyX2lkIjoxLCJ1c2VybmFtZSI6InVzZXJfbmFtZSIsImV4cCI6MTcwOTgyMjY1ODIwNiwibmJmIjo4NjQwMDAwMCwiaXNzIjoic2lsZW50LXJhaW4ifQ.1DfBZg5p5CVnRrpZ3WWpBNUdh2v7rGB_wPsGy2NSTSU";
        let result = decode_token(token).unwrap();
        println!("result: {:?}", result);
        let expected = Claims {
            user_id: 1,
            username: "user_name".to_owned(),
            exp: 1709822658206,
            nbf: NBF,
            iss: ISS.to_owned(),
        };

        assert!(result == expected);
        if let Err(e) = result.verify() {
            println!("err: {:?}", e);
            assert!(e == Error::CheckExp);
        }
    }
}
