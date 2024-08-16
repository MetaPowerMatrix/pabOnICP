#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk::update(name = "register")]
fn kol_register() -> String {
    "1232".to_string()
}
