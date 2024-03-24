//! 验证码

use captcha_rs::CaptchaBuilder;

/// 生成验证码
pub fn generate_captcha() -> (String, String) {
    let captcha = CaptchaBuilder::new()
        .length(5)
        .width(130)
        .height(40)
        .dark_mode(false)
        .complexity(1) // min: 1, max: 10
        .compression(40) // min: 1, max: 99
        .build();

    let text = captcha.text.clone();
    let base_img = captcha.to_base64();
    (text, base_img)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_captcha() {
        let (text, base_img) = generate_captcha();
        println!("text: {}", text);
        // println!("base_img: {}", base_img);
        assert_eq!(text.len(), 5);
        assert!(base_img.starts_with("data:image/jpeg;base64,"));
    }
}
