//! 浏览器

use tokio::task;
use uap_rust::parser::Parser;

/// 从User-Agent解析浏览器系统等信息
pub async fn parse_user_agent_async(
    user_agent: String,
) -> Result<(String, String, String), String> {
    let user_agent = user_agent.clone();
    task::spawn_blocking(move || {
        let p = Parser::new().map_err(|err| err.to_string())?;
        let client = p.parse(user_agent);
        let device = client.device.family;
        let system = client.os.family;
        let browser = client.user_agent.family;
        Ok((device, system, browser))
    })
    .await
    .map_err(|err| err.to_string())?
}
