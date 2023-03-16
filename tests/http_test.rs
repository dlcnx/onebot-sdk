#[cfg(test)]
mod http {
    #[tokio::test]
    async fn async_connect() {
        let code = reqwest::get("https://baidu.com").await.unwrap().status();
        assert_eq!(200, code);
    }

    #[test]
    fn blocking_connect() {
        let code = reqwest::blocking::get("https://baidu.com")
            .unwrap()
            .status();
        assert_eq!(200, code);
    }
}
