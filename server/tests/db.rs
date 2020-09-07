#[test]
fn get_credentials_for_db() {
    let username = std::env::var("DB_ADMIN");
    let password = std::env::var("DB_PASSWORD");
    let url = std::env::var("DB_URL");

    assert_eq!(username.is_ok(), true);
    assert_eq!(password.is_ok(), true);
    assert_eq!(url.is_ok(), true);
}
