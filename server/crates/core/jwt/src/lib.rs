use chrono::Local;
use jsonwebtoken::{
    decode, encode, errors, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};

const SECRET: &str = "secret";
const ISS: &str = "silent-rain";
const KID: &str = "silent-rain";
/// Not Before 时间
const NBF: usize = 0;
/// Token 过期时间
const EXPIRE: i64 = 1000 * 60 * 60 * 24 * 30; // 30 Day

#[derive(Debug, PartialEq, thiserror::Error)]
#[repr(u16)]
pub enum Error {
    /// 令牌已经过期
    #[error("Token has expired")]
    CheckExp,
    /// Audience 验证错误
    #[error("Audience verification failed")]
    CheckAud,
    /// Issued At 时间验证错误
    #[error("Issued At time verification failed")]
    CheckIat,
    /// Not Before 时间验证错误
    #[error("Not Before time verification failed")]
    CheckNbf,
    /// Subject 验证错误
    #[error("Subject verification failed")]
    CheckSub,
    /// 发行人验证错误
    #[error("Issuer verification failed")]
    CheckIss,
    /// JWT 错误
    #[error("JWT processing error: {0}")]
    JwtError(#[from] errors::Error),
}

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Claims {
    pub user_id: i32,
    pub username: String,

    /// Required, Expiration time, 表示令牌的过期时间。如果当前时间超过此时间戳，令牌将不再有效。默认情况下，验证过程会检查这个值。
    exp: usize,
    /// Optional, Audience, 指定令牌的预期接收者。如果令牌的接收者不在这个声明中指定的列表中，令牌应该被拒绝。
    aud: Option<String>,
    /// Optional, Issued at, 记录令牌的发行时间。它可以用来确定令牌的年龄。
    iat: Option<usize>,
    /// Optional, Not Before, 指定一个时间戳，在此时间之前，令牌不应被接受处理。
    /// 延迟生效：nbf 允许您设置一个时间戳，表示令牌在此之前不应该被接受。这对于延迟生效的令牌非常有用。例如，如果您希望令牌在某个特定时间之后才能被使用，您可以设置 nbf 为该时间之后的时间戳。
    /// 防止早期使用：如果令牌在 nbf 时间之前被接收，验证过程应该拒绝该令牌。这有助于防止令牌在不合理的时间被使用。
    nbf: Option<usize>,
    /// Optional, Subject, 指定令牌的主题，通常是指用户的唯一标识符。
    sub: Option<String>,
    /// Optional, Issuer, 指定发行令牌的实体。
    iss: Option<String>,
}

impl Claims {
    /// 验证过期时间
    pub fn check_exp(&self) -> Result<&Self, Error> {
        let exp = Local::now().timestamp_millis();
        let nbf = self.nbf.unwrap_or(0);
        if exp as usize >= self.exp + nbf {
            return Err(Error::CheckExp);
        }
        Ok(self)
    }

    /// 验证 Audience
    pub fn check_aud(&self, expected_aud: &str) -> Result<&Self, Error> {
        if self.aud.is_none() {
            return Ok(self);
        }
        if self.aud != Some(expected_aud.to_owned()) {
            return Err(Error::CheckAud);
        }
        Ok(self)
    }

    /// 验证 Issued At 时间
    pub fn check_iat(&self, valid_duration: usize) -> Result<&Self, Error> {
        let inner_iat = match self.iat {
            Some(v) => v,
            None => return Ok(self),
        };
        let iat = Local::now().timestamp() as usize;
        if iat >= inner_iat + valid_duration {
            return Err(Error::CheckIat);
        }
        Ok(self)
    }

    /// 验证 Not Before 时间
    pub fn check_nbf(&self) -> Result<&Self, Error> {
        if self.nbf.is_none() {
            return Ok(self);
        }
        if self.nbf != Some(NBF) {
            return Err(Error::CheckNbf);
        }
        Ok(self)
    }

    /// 验证 Subject
    pub fn check_sub(&self, expected_sub: &str) -> Result<&Self, Error> {
        if self.sub.is_none() {
            return Ok(self);
        }
        if self.sub != Some(expected_sub.to_owned()) {
            return Err(Error::CheckSub);
        }
        Ok(self)
    }

    /// 验证发行人
    pub fn check_iss(&self) -> Result<&Self, Error> {
        if self.iss.is_none() {
            return Ok(self);
        }
        if self.iss != Some(ISS.to_owned()) {
            return Err(Error::CheckIss);
        }
        Ok(self)
    }

    /// 验证所有的 Claims
    pub fn verify(&self) -> Result<(), Error> {
        self.check_iss()?.check_nbf()?.check_exp()?;

        Ok(())
    }
}

/// 编码
pub fn encode_token(user_id: i32, username: String) -> Result<String, Error> {
    let exp = Local::now().timestamp_millis() + EXPIRE;
    let claims = Claims {
        user_id,
        username,
        exp: exp as usize,
        nbf: Some(NBF),
        iss: Some(ISS.to_owned()),
        ..Default::default()
    };
    let mut header = Header::new(Algorithm::HS256);
    header.kid = Some(KID.to_owned());
    let token = encode(&header, &claims, &EncodingKey::from_secret(SECRET.as_ref()))
        .map_err(Error::JwtError)?;
    Ok(token)
}

/// 解码
pub fn decode_token(token: &str) -> Result<Claims, Error> {
    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET.as_ref()),
        &Validation::default(),
    )
    .map_err(Error::JwtError)?
    .claims;

    Ok(claims)
}

/// 解码 Token 并进行验证
pub fn decode_token_with_verify(token: &str) -> Result<Claims, Error> {
    let claims = decode_token(token)?;
    claims.verify()?;

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
    fn it_encode_token() -> Result<(), Error> {
        let token = encode_token(1, "user_name".to_owned())?;
        println!("token: {:?}", token);
        assert!(!token.is_empty());

        Ok(())
    }

    #[test]
    fn it_decode_token() -> Result<(), Error> {
        let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiIsImtpZCI6InNpbGVudC1yYWluIn0.eyJ1c2VyX2lkIjoxLCJ1c2VybmFtZSI6InVzZXJfbmFtZSIsImV4cCI6MTcxNTUyNTYzNDQ2NywiYXVkIjpudWxsLCJpYXQiOm51bGwsIm5iZiI6MCwic3ViIjpudWxsLCJpc3MiOiJzaWxlbnQtcmFpbiJ9.g4A48G5PE0vAaiqYzXEe_Xb7AtLo9h5B3Z3hOOYgDyU";
        let result = decode_token(token)?;
        println!("result: {:?}", result);

        let expected = Claims {
            user_id: 1,
            username: "user_name".to_owned(),
            exp: 1715525634467,
            nbf: Some(NBF),
            iss: Some(ISS.to_owned()),
            ..Default::default()
        };

        assert!(result == expected);
        Ok(())
    }

    #[test]
    fn it_decode_token_verify() -> Result<(), Error> {
        let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiIsImtpZCI6InNpbGVudC1yYWluIn0.eyJ1c2VyX2lkIjoxLCJ1c2VybmFtZSI6InVzZXJfbmFtZSIsImV4cCI6MTcxNTUyNTc1NjM1NCwiYXVkIjpudWxsLCJpYXQiOm51bGwsIm5iZiI6MCwic3ViIjpudWxsLCJpc3MiOiJzaWxlbnQtcmFpbiJ9.HtSnf34Ybz-O8cfKMADJ_lvKK8LdRxKcPhb4yynNn-o";
        let result = decode_token(token)?;
        println!("result: {:?}", result);

        let expected = Claims {
            user_id: 1,
            username: "user_name".to_owned(),
            exp: 1715525756354,
            aud: None,
            iat: None,
            nbf: Some(0),
            sub: None,
            iss: Some("silent-rain".to_owned()),
        };
        assert!(result == expected);

        if let Err(e) = result.verify() {
            println!("err: {:?}", e);
            assert!(e == Error::CheckExp);
        }

        Ok(())
    }
}
