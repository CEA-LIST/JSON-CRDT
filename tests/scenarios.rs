use json::{
    classifiers::{
        JsonKind,
        JsonKindVariant::{Boolean, Number},
    },
    package::{Json, JsonLog, ReadAsEcore},
};
use moirai_crdt::{
    counter::resettable_counter::Counter, flag::ew_flag::EWFlag, utils::membership::twins_log,
};
use moirai_protocol::replica::IsReplica;

#[test]
fn example_of_execution() {
    // Two replicasof the same JSON CRDT document
    let (mut replica_a, mut replica_b) = twins_log::<JsonLog>();

    // A and B make concurrent changes to the document

    let event_a1 = replica_a
        .send(Json::JsonKind(JsonKind::Number(Counter::Inc(5.0))))
        .unwrap();
    let event_b1 = replica_b
        .send(Json::JsonKind(JsonKind::Boolean(EWFlag::Enable)))
        .unwrap();

    // They receive each other's changes

    replica_b.receive(event_a1.clone());
    replica_a.receive(event_b1.clone());

    // They make more changes

    let event_a2 = replica_a
        .send(Json::JsonKind(JsonKind::Choose(Number)))
        .unwrap();
    let event_b2 = replica_b
        .send(Json::JsonKind(JsonKind::Choose(Boolean)))
        .unwrap();

    // They receive each other's changes again

    replica_b.receive(event_a2);
    replica_a.receive(event_b2);

    // They are in the same state and see the same document
    // They can query the document as an Ecore model

    println!("{}", replica_a.query(ReadAsEcore));

    assert_eq!(replica_a.query(ReadAsEcore), replica_b.query(ReadAsEcore));
}
