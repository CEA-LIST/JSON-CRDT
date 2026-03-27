use moirai_crdt::{
    counter::resettable_counter::Counter,
    flag::ew_flag::EWFlag,
    list::{
        eg_walker::List,
        nested_list::{NestedList, NestedListLog},
    },
    map::uw_map::{UWMap, UWMapLog},
};
use moirai_fuzz::{
    metrics::{FuzzMetrics, StructureMetrics},
    op_generator::OpGeneratorNested,
};
use moirai_protocol::{
    crdt::query::Read,
    state::{event_graph::EventGraph, po_log::VecLog},
    utils::boxer::Boxer,
};
use rand::Rng;

use crate::{
    classifiers::{
        Json as InnerJson, JsonChild, JsonChildValue, JsonContainer, JsonLog as InnerJsonLog,
        JsonValue as InnerJsonValue,
    },
    package::{Json, JsonLog},
};

#[cfg(feature = "fuzz")]
impl OpGeneratorNested for InnerJsonLog {
    fn generate(&self, rng: &mut impl Rng) -> Self::Op {
        use moirai_protocol::state::log::IsLog;
        use rand::distr::{Distribution, weighted::WeightedIndex};

        enum Choice {
            Number,
            Boolean,
            String,
            Object,
            Array,
        }
        let dist = WeightedIndex::new([2, 2, 2, 3, 3]).unwrap();

        fn generate_number(log: &VecLog<Counter<f64>>, rng: &mut impl Rng) -> InnerJson {
            InnerJson::Number(<VecLog<Counter<f64>> as OpGeneratorNested>::generate(
                log, rng,
            ))
        }

        fn generate_boolean(log: &VecLog<EWFlag>, rng: &mut impl Rng) -> InnerJson {
            InnerJson::Boolean(<VecLog<EWFlag> as OpGeneratorNested>::generate(log, rng))
        }

        fn generate_string(log: &EventGraph<List<char>>, rng: &mut impl Rng) -> InnerJson {
            InnerJson::String(<EventGraph<List<char>> as OpGeneratorNested>::generate(
                log, rng,
            ))
        }

        fn generate_object(log: &UWMapLog<String, InnerJsonLog>, rng: &mut impl Rng) -> InnerJson {
            let op = <UWMapLog<String, InnerJsonLog> as OpGeneratorNested>::generate(log, rng);
            InnerJson::Object(Boxer::<UWMap<String, Box<InnerJson>>>::boxer(op))
        }

        fn generate_array(log: &NestedListLog<InnerJsonLog>, rng: &mut impl Rng) -> InnerJson {
            let op = <NestedListLog<InnerJsonLog> as OpGeneratorNested>::generate(log, rng);
            InnerJson::Array(Boxer::<NestedList<Box<InnerJson>>>::boxer(op))
        }

        fn generate_value(val: &JsonChildValue, log: &JsonChild, rng: &mut impl Rng) -> InnerJson {
            match (val, log) {
                (JsonChildValue::Number(_), JsonChild::Number(l)) => generate_number(l, rng),
                (JsonChildValue::Boolean(_), JsonChild::Boolean(l)) => generate_boolean(l, rng),
                (JsonChildValue::String(_), JsonChild::String(l)) => generate_string(l, rng),
                (JsonChildValue::Object(_), JsonChild::Object(l)) => generate_object(l, rng),
                (JsonChildValue::Array(_), JsonChild::Array(l)) => generate_array(l, rng),
                _ => unreachable!(),
            }
        }

        let value = self.eval(Read::new());

        match value {
            InnerJsonValue::Unset => {
                use moirai_protocol::state::log::IsLog;

                let available_choices: Vec<Choice> = match &self.child {
                    JsonContainer::Unset => vec![
                        Choice::Number,
                        Choice::String,
                        Choice::Boolean,
                        Choice::Object,
                        Choice::Array,
                    ],
                    JsonContainer::Value(child) => match child.as_ref() {
                        JsonChild::Number(_) => vec![Choice::Number],
                        JsonChild::Boolean(_) => vec![Choice::Boolean],
                        JsonChild::String(_) => vec![Choice::String],
                        JsonChild::Object(_) => vec![Choice::Object],
                        JsonChild::Array(_) => vec![Choice::Array],
                    },
                    JsonContainer::Conflicts(children) => children
                        .iter()
                        .map(|child| match child {
                            JsonChild::Number(_) => Choice::Number,
                            JsonChild::Boolean(_) => Choice::Boolean,
                            JsonChild::String(_) => Choice::String,
                            JsonChild::Object(_) => Choice::Object,
                            JsonChild::Array(_) => Choice::Array,
                        })
                        .collect(),
                };

                let choice = if available_choices.len() == 5 {
                    &available_choices[dist.sample(rng)]
                } else {
                    rand::seq::IteratorRandom::choose(available_choices.iter(), rng).unwrap()
                };
                match choice {
                    Choice::Number => generate_number(&VecLog::<Counter<f64>>::new(), rng),
                    Choice::Boolean => generate_boolean(&VecLog::<EWFlag>::new(), rng),
                    Choice::Object => {
                        generate_object(&UWMapLog::<String, InnerJsonLog>::new(), rng)
                    }
                    Choice::String => generate_string(&EventGraph::<List<char>>::new(), rng),
                    Choice::Array => generate_array(&NestedListLog::<InnerJsonLog>::new(), rng),
                }
            }
            InnerJsonValue::Value(v) => match &self.child {
                JsonContainer::Value(child) => generate_value(&v, child.as_ref(), rng),
                JsonContainer::Conflicts(child_logs) => {
                    let log = child_logs
                        .iter()
                        .find(|log| {
                            matches!(
                                (v.as_ref(), log),
                                (JsonChildValue::Number(_), JsonChild::Number(_))
                                    | (JsonChildValue::Boolean(_), JsonChild::Boolean(_))
                                    | (JsonChildValue::Object(_), JsonChild::Object(_))
                                    | (JsonChildValue::String(_), JsonChild::String(_))
                                    | (JsonChildValue::Array(_), JsonChild::Array(_))
                            )
                        })
                        .unwrap();
                    generate_value(&v, log, rng)
                }
                JsonContainer::Unset => unreachable!(),
            },
            InnerJsonValue::Conflict(json_child_values) => match &self.child {
                JsonContainer::Conflicts(child_logs) => {
                    let choice =
                        rand::seq::IteratorRandom::choose(json_child_values.iter(), rng).unwrap();
                    let log = child_logs
                        .iter()
                        .find(|log| {
                            matches!(
                                (choice, log),
                                (JsonChildValue::Number(_), JsonChild::Number(_))
                                    | (JsonChildValue::Boolean(_), JsonChild::Boolean(_))
                                    | (JsonChildValue::Object(_), JsonChild::Object(_))
                                    | (JsonChildValue::String(_), JsonChild::String(_))
                                    | (JsonChildValue::Array(_), JsonChild::Array(_))
                            )
                        })
                        .unwrap();
                    generate_value(choice, log, rng)
                }
                _ => unreachable!(),
            },
        }
    }
}

impl OpGeneratorNested for JsonLog {
    fn generate(&self, rng: &mut impl Rng) -> Self::Op {
        Json::Json(self.json_log().generate(rng))
    }
}

impl FuzzMetrics for JsonLog {
    fn structure_metrics(&self) -> StructureMetrics {
        self.json_log().structure_metrics()
    }
}

#[cfg(test)]
mod tests {
    use moirai_fuzz::{
        config::{FuzzerConfig, RunConfig},
        fuzzer::fuzzer,
    };

    use crate::package::JsonLog;

    #[test]
    fn fuzz_json() {
        let run = RunConfig::new(0.1, 4, 1_000, None, None, false, false);
        let runs = vec![run; 1];

        let config =
            FuzzerConfig::<JsonLog>::new("json", runs, true, |a, b| a.json == b.json, false);

        fuzzer::<JsonLog>(config);
    }
}
