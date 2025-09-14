// native/src/lib.rs
use flutter_rust_bridge::frb;
use pichamu_core::*; // 调用核心库

#[frb(sync)]
pub fn add(a: i32, b: i32) -> i32 {
    api::add(a, b)
}

#[frb]
pub async fn fetch_message() -> String {
    api::fetch_message().await
}

#[frb(mirror(MyData))]
pub struct MyData {
    pub id: i32,
    pub name: String,
}

#[frb(sync)]
pub fn get_data() -> MyData {
    let d = api::get_data();
    MyData { id: d.id, name: d.name }
}

#[frb(sync)]
pub fn might_fail(flag: bool) -> Result<String, String> {
    api::might_fail(flag).map_err(|e| e.to_string())
}
