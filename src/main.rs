use wsm::setup;
use serde_json::json;
use serde_json::Value;

mod message;

fn pretty_print(json_str: &str) {
    // 1. parse 回 Value
    let v: Value = serde_json::from_str(json_str)
        .expect("返回的不是合法 JSON");
    // 2. pretty-print
    let s = serde_json::to_string_pretty(&v)
        .expect("无法序列化为 pretty JSON");
    println!("{}", s);
}

fn main() {
    setup();
    // 构造一个示例参数对象
    let params: Value = json!({
        "user": "alice",
        "action": "ping"
    });

    // 1) 创建并打印一个 request
    let request_json = message::create_request(
        "client",      // from
        "server",      // to
        "msg-1001",      // msg_id
        "do_something",  // method
        params.clone(),  // params
    );
    pretty_print(&request_json);

    // 2) 创建并打印一个 event
    let event_json = message::create_event(
        "evt-2002",      // msg_id
        "request",  // method
        params.clone(),  // params
    );
    pretty_print(&event_json);
    

    // 3) 创建并打印一个 success response
    let success_json = message::create_success_response(
        "server",      // from
        "1234s",      // to
        "msg-1001",      // msg_id
        "rcpt-3003",     // receipt
    );
    pretty_print(&success_json);

    // 4) 创建并打印一个 fail response
    let fail_json = message::create_fail_response(
        "193sj",     // from
        "client",     // to
        "msg-1001",     // msg_id
        "ERR_NOT_OK",   // err_code
    );
    pretty_print(&fail_json);
}