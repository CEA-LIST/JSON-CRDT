use json::package::JsonLog;

#[test]
fn fuzz() {
    use moirai_fuzz::{
        config::{FuzzerConfig, RunConfig},
        fuzzer::fuzzer,
    };

    let run = RunConfig::new(0.4, 2, 3, None, None, true, false);
    let runs = vec![run.clone(); 10_000];

    let config = FuzzerConfig::<JsonLog>::new("json", runs, true, |a, b| a.json == b.json, false);

    fuzzer::<JsonLog>(config);
}
