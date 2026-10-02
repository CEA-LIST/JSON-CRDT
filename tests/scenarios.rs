/// We reproduce the conflict-scenario tests from the original JSON CRDT paper here:
///     Kleppmann, M., & Beresford, A. R. (2017).
///     "A conflict-free replicated JSON datatype."
///     IEEE Transactions on Parallel and Distributed Systems, 28(10), 2733-2746.
use json_model::{
    classifiers::{JsonKind, JsonKindChildValue, JsonKindValue},
    package::{Json, JsonLog, JsonValue},
};
use moirai_crdt::{
    counter::resettable_counter::Counter,
    flag::ew_flag::EWFlag,
    list::{eg_walker::List, nested_list::NestedList},
    map::uw_map::UWMap,
    utils::membership::twins_log,
};
use moirai_protocol::{crdt::query::Read, replica::IsReplica};
use rustc_hash::FxHashMap;

/// Test 1: Concurrent assignment to the register at doc.get(“key”) by replicas p and q.
///
/// We could not reproduce the first conflict as the generated JSON CRDT API is richer
/// than the one used in the original paper, and does not use registers for primitive values.
/// Instead, we use a counter for numbers, a list for strings, and a flag for booleans.
/// This means that the first conflict scenario is not possible in our implementation,
/// as it requires two replicas to concurrently assign different primitive values to the same key in a map.
#[test]
fn concurrent_assignment_register() {
    let (mut replica_a, mut replica_b) = twins_log::<JsonLog>();

    // * CONCURRENCY ON BOOLEAN VALUES *//

    let event_a_1 = replica_a
        .send(Json::JsonKind(JsonKind::Object(UWMap::Update(
            String::from("flag"),
            Box::new(JsonKind::Boolean(EWFlag::Enable)),
        ))))
        .unwrap();

    let event_b_1 = replica_b
        .send(Json::JsonKind(JsonKind::Object(UWMap::Update(
            String::from("flag"),
            Box::new(JsonKind::Boolean(EWFlag::Disable)),
        ))))
        .unwrap();

    // * CONCURRENCY ON STRING VALUES *//

    let event_a_2 = replica_a
        .send(Json::JsonKind(JsonKind::Object(UWMap::Update(
            String::from("string"),
            Box::new(JsonKind::String(List::insert('a', 0))),
        ))))
        .unwrap();

    let event_b_2 = replica_b
        .send(Json::JsonKind(JsonKind::Object(UWMap::Update(
            "string".to_string(),
            Box::new(JsonKind::String(List::insert('b', 0))),
        ))))
        .unwrap();

    // * CONCURRENCY ON NUMERIC VALUES *//

    let event_a_3 = replica_a
        .send(Json::JsonKind(JsonKind::Object(UWMap::Update(
            String::from("number"),
            Box::new(JsonKind::Number(Counter::Inc(1.0))),
        ))))
        .unwrap();

    let event_b_3 = replica_b
        .send(Json::JsonKind(JsonKind::Object(UWMap::Update(
            String::from("number"),
            Box::new(JsonKind::Number(Counter::Inc(2.0))),
        ))))
        .unwrap();

    //* CONVERGENCE *//

    replica_b.receive(event_a_1);
    replica_b.receive(event_a_2);
    replica_b.receive(event_a_3);
    replica_a.receive(event_b_1);
    replica_a.receive(event_b_2);
    replica_a.receive(event_b_3);

    let value_a = replica_a.query(&Read::new());
    let value_b = replica_b.query(&Read::new());

    let expected_value = JsonValue {
        json: JsonKindValue::Value(Box::new(JsonKindChildValue::Object(FxHashMap::from_iter(
            [
                (
                    String::from("flag"),
                    JsonKindValue::Value(Box::new(JsonKindChildValue::Boolean(true))),
                ),
                (
                    String::from("string"),
                    JsonKindValue::Value(Box::new(JsonKindChildValue::String(String::from("ab")))),
                ),
                (
                    String::from("number"),
                    JsonKindValue::Value(Box::new(JsonKindChildValue::Number(3.0))),
                ),
            ],
        )))),
    };

    assert_eq!(value_a.json, value_b.json);
    assert_eq!(value_a.json, expected_value.json);
}

/// Test 2: Modifying the contents of a nested map while concurrently the entire map is overwritten.
#[test]
fn modification_nested_map_concurrently_overwritten() {
    let (mut replica_a, mut replica_b) = twins_log::<JsonLog>();

    //* INITIAL STATE *//

    let blue = "#0000ff";

    for (i, c) in blue.chars().enumerate() {
        let event_a = replica_a
            .send(Json::JsonKind(JsonKind::Object(UWMap::Update(
                String::from("colors"),
                Box::new(JsonKind::Object(UWMap::Update(
                    String::from("blue"),
                    Box::new(JsonKind::String(List::insert(c, i))),
                ))),
            ))))
            .unwrap();
        replica_b.receive(event_a);
    }

    let value_a = replica_a.query(&Read::new());
    let value_b = replica_b.query(&Read::new());
    assert_eq!(value_a.json, value_b.json);

    //* CONCURRENCY *//

    let red = "#ff0000";
    let mut events_a = vec![];

    for (i, c) in red.chars().enumerate() {
        let event_a = replica_a
            .send(Json::JsonKind(JsonKind::Object(UWMap::Update(
                String::from("colors"),
                Box::new(JsonKind::Object(UWMap::Update(
                    String::from("red"),
                    Box::new(JsonKind::String(List::insert(c, i))),
                ))),
            ))))
            .unwrap();
        events_a.push(event_a);
    }

    let event_b = replica_b
        .send(Json::JsonKind(JsonKind::Object(UWMap::Clear)))
        .unwrap();

    let green = "#00ff00";
    let mut events_b = vec![];

    for (i, c) in green.chars().enumerate() {
        let event_b = replica_b
            .send(Json::JsonKind(JsonKind::Object(UWMap::Update(
                String::from("colors"),
                Box::new(JsonKind::Object(UWMap::Update(
                    String::from("green"),
                    Box::new(JsonKind::String(List::insert(c, i))),
                ))),
            ))))
            .unwrap();
        events_b.push(event_b);
    }

    //* CONVERGENCE *//

    for event_a in events_a {
        replica_b.receive(event_a);
    }

    replica_a.receive(event_b);

    for event_b in events_b {
        replica_a.receive(event_b);
    }

    let value_a = replica_a.query(&Read::new());
    let value_b = replica_b.query(&Read::new());

    let expected_value = JsonValue {
        json: JsonKindValue::Value(Box::new(JsonKindChildValue::Object(FxHashMap::from_iter(
            [(
                String::from("colors"),
                JsonKindValue::Value(Box::new(JsonKindChildValue::Object(FxHashMap::from_iter(
                    [
                        (
                            String::from("blue"),
                            JsonKindValue::Value(Box::new(JsonKindChildValue::String(
                                String::new(),
                            ))),
                        ),
                        (
                            String::from("green"),
                            JsonKindValue::Value(Box::new(JsonKindChildValue::String(
                                String::from("#00ff00"),
                            ))),
                        ),
                        (
                            String::from("red"),
                            JsonKindValue::Value(Box::new(JsonKindChildValue::String(
                                String::from("#ff0000"),
                            ))),
                        ),
                    ],
                )))),
            )],
        )))),
    };

    assert_eq!(value_a.json, value_b.json);
    assert_eq!(value_a.json, expected_value.json);
}

/// Test 3: Two replicas concurrently create ordered lists under the same map key.
#[test]
fn concurrently_create_ordered_lists_under_same_map_key() {
    let (mut replica_a, mut replica_b) = twins_log::<JsonLog>();

    //* CONCURRENCY *//

    let mut events_a = vec![];

    let eggs = "eggs";
    let ham = "ham";

    for (i, l) in [eggs, ham].iter().enumerate() {
        for (k, c) in l.chars().enumerate() {
            let array_op = if k == 0 {
                NestedList::insert(i, Box::new(JsonKind::String(List::insert(c, k))))
            } else {
                NestedList::update(i, Box::new(JsonKind::String(List::insert(c, k))))
            };
            let event_a = replica_a
                .send(Json::JsonKind(JsonKind::Object(UWMap::Update(
                    String::from("grocery"),
                    Box::new(JsonKind::Array(array_op)),
                ))))
                .unwrap();
            events_a.push(event_a);
        }
    }

    let mut events_b = vec![];

    let milk = "milk";
    let flour = "flour";

    for (i, l) in [milk, flour].iter().enumerate() {
        for (k, c) in l.chars().enumerate() {
            let array_op = if k == 0 {
                NestedList::insert(i, Box::new(JsonKind::String(List::insert(c, k))))
            } else {
                NestedList::update(i, Box::new(JsonKind::String(List::insert(c, k))))
            };
            let event_a = replica_b
                .send(Json::JsonKind(JsonKind::Object(UWMap::Update(
                    String::from("grocery"),
                    Box::new(JsonKind::Array(array_op)),
                ))))
                .unwrap();
            events_b.push(event_a);
        }
    }

    //* CONVERGENCE *//

    for event_a in events_a {
        replica_b.receive(event_a);
    }

    for event_b in events_b {
        replica_a.receive(event_b);
    }

    let value_a = replica_a.query(&Read::new());
    let value_b = replica_b.query(&Read::new());

    let expected_value = JsonValue {
        json: JsonKindValue::Value(Box::new(JsonKindChildValue::Object(FxHashMap::from_iter(
            [(
                String::from("grocery"),
                JsonKindValue::Value(Box::new(JsonKindChildValue::Array(vec![
                    Box::new(JsonKindValue::Value(Box::new(JsonKindChildValue::String(
                        String::from("eggs"),
                    )))),
                    Box::new(JsonKindValue::Value(Box::new(JsonKindChildValue::String(
                        String::from("ham"),
                    )))),
                    Box::new(JsonKindValue::Value(Box::new(JsonKindChildValue::String(
                        String::from("milk"),
                    )))),
                    Box::new(JsonKindValue::Value(Box::new(JsonKindChildValue::String(
                        String::from("flour"),
                    )))),
                ]))),
            )],
        )))),
    };

    assert_eq!(value_a.json, value_b.json);
    assert_eq!(value_a.json, expected_value.json);
}

/// Test 4: Concurrent editing of an ordered list of characters (i.e., a text document).
#[test]
fn concurrent_editing_ordered_list() {
    let (mut replica_a, mut replica_b) = twins_log::<JsonLog>();

    //* INITIAL STATE *//

    for (i, c) in "abc".chars().enumerate() {
        let event_a = replica_a
            .send(Json::JsonKind(JsonKind::Array(NestedList::insert(
                i,
                Box::new(JsonKind::String(List::insert(c, 0))),
            ))))
            .unwrap();
        replica_b.receive(event_a);
    }

    //* CONCURRENCY *//

    let event_a_1 = replica_a
        .send(Json::JsonKind(JsonKind::Array(NestedList::delete(1))))
        .unwrap();

    let event_a_2 = replica_a
        .send(Json::JsonKind(JsonKind::Array(NestedList::insert(
            1,
            Box::new(JsonKind::String(List::insert('x', 0))),
        ))))
        .unwrap();

    let event_b_1 = replica_b
        .send(Json::JsonKind(JsonKind::Array(NestedList::insert(
            0,
            Box::new(JsonKind::String(List::insert('y', 0))),
        ))))
        .unwrap();

    let event_b_2 = replica_b
        .send(Json::JsonKind(JsonKind::Array(NestedList::insert(
            2,
            Box::new(JsonKind::String(List::insert('z', 0))),
        ))))
        .unwrap();

    //* CONVERGENCE *//

    replica_b.receive(event_a_1);
    replica_b.receive(event_a_2);

    replica_a.receive(event_b_1);
    replica_a.receive(event_b_2);

    let value_a = replica_a.query(&Read::new());
    let value_b = replica_b.query(&Read::new());

    let expected_value = JsonValue {
        json: JsonKindValue::Value(Box::new(JsonKindChildValue::Array(vec![
            Box::new(JsonKindValue::Value(Box::new(JsonKindChildValue::String(
                String::from("y"),
            )))),
            Box::new(JsonKindValue::Value(Box::new(JsonKindChildValue::String(
                String::from("a"),
            )))),
            Box::new(JsonKindValue::Value(Box::new(JsonKindChildValue::String(
                String::from("x"),
            )))),
            Box::new(JsonKindValue::Value(Box::new(JsonKindChildValue::String(
                String::from("z"),
            )))),
            Box::new(JsonKindValue::Value(Box::new(JsonKindChildValue::String(
                String::from("c"),
            )))),
        ]))),
    };

    assert_eq!(value_a.json, value_b.json);
    assert_eq!(value_a.json, expected_value.json);
}

/// Test 5: Concurrently assigning values of different types to the same map key.
#[test]
fn concurrent_assignement_values_different_types_to_same_key_map() {
    let (mut replica_a, mut replica_b) = twins_log::<JsonLog>();

    //* CONCURRENCY *//

    let event_a = replica_a
        .send(Json::JsonKind(JsonKind::Object(UWMap::Update(
            String::from("a"),
            Box::new(JsonKind::Object(UWMap::Update(
                String::from("x"),
                Box::new(JsonKind::String(List::insert('y', 0))),
            ))),
        ))))
        .unwrap();

    let event_b = replica_b
        .send(Json::JsonKind(JsonKind::Object(UWMap::Update(
            String::from("a"),
            Box::new(JsonKind::Array(NestedList::insert(
                0,
                Box::new(JsonKind::String(List::insert('z', 0))),
            ))),
        ))))
        .unwrap();

    //* CONVERGENCE *//

    replica_b.receive(event_a);
    replica_a.receive(event_b);

    let value_a = replica_a.query(&Read::new());
    let value_b = replica_b.query(&Read::new());

    let expected_value = JsonValue {
        json: JsonKindValue::Value(Box::new(JsonKindChildValue::Object(FxHashMap::from_iter(
            [(
                String::from("a"),
                JsonKindValue::Conflict(vec![
                    JsonKindChildValue::Array(vec![Box::new(JsonKindValue::Value(Box::new(
                        JsonKindChildValue::String(String::from("z")),
                    )))]),
                    JsonKindChildValue::Object(FxHashMap::from_iter([(
                        String::from("x"),
                        JsonKindValue::Value(Box::new(JsonKindChildValue::String(String::from(
                            "y",
                        )))),
                    )])),
                ]),
            )],
        )))),
    };

    assert_eq!(value_a.json, value_b.json);
    assert_eq!(value_a.json, expected_value.json);
}

/// Test 6: One replica removes a list element, while another concurrently updates its contents.
#[test]
fn concurrent_remove_update_list_element() {
    let (mut replica_a, mut replica_b) = twins_log::<JsonLog>();

    //* INITIAL STATE *//

    // {"todo": [{"title": "buy milk", "done": false}]}
    for (index, character) in "buy milk".chars().enumerate() {
        let title_op = JsonKind::Object(UWMap::Update(
            String::from("title"),
            Box::new(JsonKind::String(List::insert(character, index))),
        ));
        let array_op = if index == 0 {
            NestedList::insert(0, Box::new(title_op))
        } else {
            NestedList::update(0, Box::new(title_op))
        };
        let event_a = replica_a
            .send(Json::JsonKind(JsonKind::Object(UWMap::Update(
                String::from("todo"),
                Box::new(JsonKind::Array(array_op)),
            ))))
            .unwrap();
        replica_b.receive(event_a);
    }

    let event_a = replica_a
        .send(Json::JsonKind(JsonKind::Object(UWMap::Update(
            String::from("todo"),
            Box::new(JsonKind::Array(NestedList::update(
                0,
                Box::new(JsonKind::Object(UWMap::Update(
                    String::from("done"),
                    Box::new(JsonKind::Boolean(EWFlag::Disable)),
                ))),
            ))),
        ))))
        .unwrap();
    replica_b.receive(event_a);

    //* CONCURRENCY *//

    // {“todo”: []}
    let event_a = replica_a
        .send(Json::JsonKind(JsonKind::Object(UWMap::Update(
            String::from("todo"),
            Box::new(JsonKind::Array(NestedList::delete(0))),
        ))))
        .unwrap();

    // {“todo”: [{“title”: “buy milk”,“done”: true}]}
    let event_b = replica_b
        .send(Json::JsonKind(JsonKind::Object(UWMap::Update(
            String::from("todo"),
            Box::new(JsonKind::Array(NestedList::update(
                0,
                Box::new(JsonKind::Object(UWMap::Update(
                    String::from("done"),
                    Box::new(JsonKind::Boolean(EWFlag::Enable)),
                ))),
            ))),
        ))))
        .unwrap();

    //* CONVERGENCE *//

    replica_b.receive(event_a);
    replica_a.receive(event_b);

    // {“todo”: [{“done”: true}]}
    let expected_value = JsonValue {
        json: JsonKindValue::Value(Box::new(JsonKindChildValue::Object(FxHashMap::from_iter(
            [(
                String::from("todo"),
                JsonKindValue::Value(Box::new(JsonKindChildValue::Array(vec![Box::new(
                    JsonKindValue::Value(Box::new(JsonKindChildValue::Object(
                        FxHashMap::from_iter([
                            (
                                String::from("title"),
                                JsonKindValue::Value(Box::new(JsonKindChildValue::String(
                                    String::from(""),
                                ))),
                            ),
                            (
                                String::from("done"),
                                JsonKindValue::Value(Box::new(JsonKindChildValue::Boolean(true))),
                            ),
                        ]),
                    ))),
                )]))),
            )],
        )))),
    };
    assert_eq!(replica_a.query(&Read::new()).json, expected_value.json);
    assert_eq!(replica_b.query(&Read::new()).json, expected_value.json);
}
