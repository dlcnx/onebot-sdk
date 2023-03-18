use onebot_sdk::HttpHandler;

fn main() {
    let mut handler = HttpHandler::new();
    handler.host("127.0.0.1").port(5700).token("123456");
    let bot_info = handler.get_login_info().unwrap();
    println!(
        "bot_name: {}, bot_id: {}",
        bot_info.nickname, bot_info.user_id
    );
}
