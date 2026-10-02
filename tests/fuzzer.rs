#[cfg(feature = "fuzz")]
#[test]
#[ignore]
fn fuzz() {
    use json_model::package::{JsonLog, JsonValue};
    use moirai_fuzz::{
        config::{ChurnConfig, FuzzerConfig, Predicate, RunConfig},
        fuzzer::fuzzer,
    };
    use moirai_protocol::crdt::query::Read;

    let run = RunConfig::new(ChurnConfig::new(0.4, 0.6), 4, 1_000, None, None);

    let config = FuzzerConfig::<JsonLog, Read<JsonValue>>::new(
        "json-model",
        vec![run.clone()],
        Predicate::new(Read::new(), |a, b| a.json == b.json),
    );

    fuzzer::<JsonLog, Read<JsonValue>>(config);
}
