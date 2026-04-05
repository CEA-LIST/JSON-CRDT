use json::package::JsonLog;

#[test]
#[ignore]
fn fuzz() {
    use moirai_fuzz::{
        config::{FuzzerConfig, RunConfig},
        fuzzer::fuzzer,
    };

    let run = RunConfig::new(0.5, 4, 1_000, None, None, false, false);
    let runs = vec![run.clone(); 10];

    let config = FuzzerConfig::<JsonLog>::new("json", runs, true, |a, b| a.json == b.json, true);

    fuzzer::<JsonLog>(config);
}
