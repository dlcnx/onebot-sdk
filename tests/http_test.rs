#[cfg(test)]
mod http {
    #[tokio::test]
    async fn test_connect() {
        let code = reqwest::get("https://baidu.com").await.unwrap().status();
        assert_eq!(200, code);
    }
}
