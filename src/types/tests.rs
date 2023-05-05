use super::*;

#[test]
fn serialize_init_msg() {
    let msg = Message {
        src: "n1".into(),
        dst: "n2".into(),
        body: Body::Init {
            msg_id: 1,
            node_id: "n2".into(),
            node_ids: vec!["n2".into(), "n3".into()],
        },
    };

    let json = serde_json::to_string(&msg).unwrap();
    let assert_with = r#"{"src":"n1","dest":"n2","body":{"type":"init","msg_id":1,"node_id":"n2","node_ids":["n2","n3"]}}"#;

    assert_eq!(&json, assert_with);
}

#[test]
fn serialize_init_ok_msg() {
    let msg = Message {
        src: "n2".into(),
        dst: "n1".into(),
        body: Body::InitOk { in_reply_to: 1 },
    };

    let json = serde_json::to_string(&msg).unwrap();
    let assert_with = r#"{"src":"n2","dest":"n1","body":{"type":"init_ok","in_reply_to":1}}"#;
    assert_eq!(&json, assert_with);
}
