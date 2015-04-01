extern crate bytes;
extern crate capnp;

mod messages_capnp {
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/messages_capnp.rs"));
}



use bytes::RingBuf;
use capnp::serialize_packed;
use capnp::{
    MessageBuilder,
    MallocMessageBuilder,
};
use messages_capnp::rpc_request;

#[test]
fn it_panics() {
    let mut builder = MallocMessageBuilder::new_default();

    {
        let request = builder.init_root::<rpc_request::Builder>();
        let mut append = request.init_append_entries();
        append.set_term(0);
        append.set_prev_log_index(0);
        append.set_prev_log_term(0);
        append.set_leader_commit(0);
        append.init_entries(0);
    }

    let mut buf = RingBuf::new(4096);

    serialize_packed::write_packed_message_unbuffered(
                    &mut buf,
                    &mut builder
    ).unwrap();

}
