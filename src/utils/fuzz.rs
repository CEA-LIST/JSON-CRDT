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

#[cfg(feature = "fuzz")]
use crate::classifiers::{
    JsonKind, JsonKindChild, JsonKindChildValue, JsonKindContainer, JsonKindLog, JsonKindValue,
};
use crate::package::{Json, JsonLog};

#[cfg(feature = "fuzz")]
impl OpGeneratorNested for JsonKindLog {
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

        fn generate_number(log: &VecLog<Counter<f64>>, rng: &mut impl Rng) -> JsonKind {
            JsonKind::Number(<VecLog<Counter<f64>> as OpGeneratorNested>::generate(
                log, rng,
            ))
        }

        fn generate_boolean(log: &VecLog<EWFlag>, rng: &mut impl Rng) -> JsonKind {
            JsonKind::Boolean(<VecLog<EWFlag> as OpGeneratorNested>::generate(log, rng))
        }

        fn generate_string(log: &EventGraph<List<char>>, rng: &mut impl Rng) -> JsonKind {
            JsonKind::String(<EventGraph<List<char>> as OpGeneratorNested>::generate(
                log, rng,
            ))
        }

        fn generate_object(log: &UWMapLog<String, JsonKindLog>, rng: &mut impl Rng) -> JsonKind {
            let op = <UWMapLog<String, JsonKindLog> as OpGeneratorNested>::generate(log, rng);
            JsonKind::Object(Boxer::<UWMap<String, Box<JsonKind>>>::boxer(op))
        }

        fn generate_array(log: &NestedListLog<JsonKindLog>, rng: &mut impl Rng) -> JsonKind {
            let op = <NestedListLog<JsonKindLog> as OpGeneratorNested>::generate(log, rng);
            JsonKind::Array(Boxer::<NestedList<Box<JsonKind>>>::boxer(op))
        }

        fn generate_value(
            val: &JsonKindChildValue,
            log: &JsonKindChild,
            rng: &mut impl Rng,
        ) -> JsonKind {
            match (val, log) {
                (JsonKindChildValue::Number(_), JsonKindChild::Number(l)) => {
                    generate_number(l, rng)
                }
                (JsonKindChildValue::Boolean(_), JsonKindChild::Boolean(l)) => {
                    generate_boolean(l, rng)
                }
                (JsonKindChildValue::String(_), JsonKindChild::String(l)) => {
                    generate_string(l, rng)
                }
                (JsonKindChildValue::Object(_), JsonKindChild::Object(l)) => {
                    generate_object(l, rng)
                }
                (JsonKindChildValue::Array(_), JsonKindChild::Array(l)) => generate_array(l, rng),
                _ => unreachable!(),
            }
        }

        let value = self.eval(Read::new());

        match value {
            JsonKindValue::Unset => {
                use moirai_protocol::state::log::IsLog;

                let available_choices: Vec<Choice> = match &self.child {
                    JsonKindContainer::Unset => vec![
                        Choice::Number,
                        Choice::String,
                        Choice::Boolean,
                        Choice::Object,
                        Choice::Array,
                    ],
                    JsonKindContainer::Value(child) => match child.as_ref() {
                        JsonKindChild::Number(_) => vec![Choice::Number],
                        JsonKindChild::Boolean(_) => vec![Choice::Boolean],
                        JsonKindChild::String(_) => vec![Choice::String],
                        JsonKindChild::Object(_) => vec![Choice::Object],
                        JsonKindChild::Array(_) => vec![Choice::Array],
                    },
                    JsonKindContainer::Conflicts(children) => children
                        .iter()
                        .map(|child| match child {
                            JsonKindChild::Number(_) => Choice::Number,
                            JsonKindChild::Boolean(_) => Choice::Boolean,
                            JsonKindChild::String(_) => Choice::String,
                            JsonKindChild::Object(_) => Choice::Object,
                            JsonKindChild::Array(_) => Choice::Array,
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
                    Choice::Object => generate_object(&UWMapLog::<String, JsonKindLog>::new(), rng),
                    Choice::String => generate_string(&EventGraph::<List<char>>::new(), rng),
                    Choice::Array => generate_array(&NestedListLog::<JsonKindLog>::new(), rng),
                }
            }
            JsonKindValue::Value(v) => match &self.child {
                JsonKindContainer::Value(child) => generate_value(&v, child.as_ref(), rng),
                JsonKindContainer::Conflicts(child_logs) => {
                    let log = child_logs
                        .iter()
                        .find(|log| {
                            matches!(
                                (v.as_ref(), log),
                                (JsonKindChildValue::Number(_), JsonKindChild::Number(_))
                                    | (JsonKindChildValue::Boolean(_), JsonKindChild::Boolean(_))
                                    | (JsonKindChildValue::Object(_), JsonKindChild::Object(_))
                                    | (JsonKindChildValue::String(_), JsonKindChild::String(_))
                                    | (JsonKindChildValue::Array(_), JsonKindChild::Array(_))
                            )
                        })
                        .unwrap();
                    generate_value(&v, log, rng)
                }
                JsonKindContainer::Unset => unreachable!(),
            },
            JsonKindValue::Conflict(json_child_values) => match &self.child {
                JsonKindContainer::Conflicts(child_logs) => {
                    let choice =
                        rand::seq::IteratorRandom::choose(json_child_values.iter(), rng).unwrap();
                    let log = child_logs
                        .iter()
                        .find(|log| {
                            matches!(
                                (choice, log),
                                (JsonKindChildValue::Number(_), JsonKindChild::Number(_))
                                    | (JsonKindChildValue::Boolean(_), JsonKindChild::Boolean(_))
                                    | (JsonKindChildValue::Object(_), JsonKindChild::Object(_))
                                    | (JsonKindChildValue::String(_), JsonKindChild::String(_))
                                    | (JsonKindChildValue::Array(_), JsonKindChild::Array(_))
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
        Json::JsonKind(self.json_log().generate(rng))
    }
}

impl FuzzMetrics for JsonLog {
    fn structure_metrics(&self) -> StructureMetrics {
        self.json_log().structure_metrics()
    }
}
