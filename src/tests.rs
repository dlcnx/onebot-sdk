use crate::HttpHandler;

#[test]
fn get_login_info() {
    let mut handler = HttpHandler::new();
    handler.token("WERTYUIO");
    handler.get_login_info().unwrap();
}

#[test]
fn get_group_msg_history() {
    let mut handler = HttpHandler::new();
    handler.token("WERTYUIO");
    handler.get_group_msg_history(531241108).unwrap();
}

#[ignore]
#[test]
fn send_group_msg() {
    let mut handler = HttpHandler::new();
    handler.token("WERTYUIO");
    handler.send_group_msg(531241108, "啊这").unwrap();
}
