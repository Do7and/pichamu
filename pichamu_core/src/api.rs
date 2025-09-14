// pichamu_core/src/api.rs
use anyhow::Result;
use serde::{Serialize, Deserialize};
use tokio::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyData {
    pub id: i32,
    pub name: String,
}

/// 简单同步函数
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 异步函数（模拟 IO）
pub async fn fetch_message() -> String {
    tokio::time::sleep(Duration::from_millis(500)).await;
    "Hello from Rust (async)".to_string()
}

/// 返回自定义结构体
pub fn get_data() -> MyData {
    MyData {
        id: 42,
        name: "Hamster".to_string(),
    }
}

/// 错误处理示例
pub fn might_fail(flag: bool) -> Result<String> {
    if flag {
        Ok("Everything OK".to_string())
    } else {
        anyhow::bail!("Something went wrong!")
    }
}
