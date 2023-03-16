#[cfg(test)]
mod http {
    #[tokio::test]
    async fn async_connect() {
        let res = reqwest::get("https://baidu.com").await.unwrap();
        assert_eq!(200, res.status());
    }

    #[test]
    fn blocking_get() {
        let res = reqwest::blocking::get("https://baidu.com").unwrap();
        assert_eq!(200, res.status());
    }

    #[test]
    fn blocking_post() {
        let client = reqwest::blocking::Client::new();
        let res = client
            .post("http://httpbin.org/post")
            .body("the exact body that is sent")
            .send()
            .unwrap();
        assert_eq!(200, res.status());
    }
}
