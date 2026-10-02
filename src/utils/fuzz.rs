use moirai_crdt::{
    counter::resettable_counter::Counter,
    flag::ew_flag::EWFlag,
    list::{
        eg_walker::List,
        nested_list::{NestedList, NestedListLog},
    },
    map::uw_map::{UWMap, UWMapLog},
};
#[cfg(feature = "fuzz")]
use moirai_fuzz::generator::command_generator::CommandGenerator;
#[cfg(feature = "fuzz")]
use moirai_fuzz::observers::footprint::{FootprintNode, LogFootprint};
#[cfg(feature = "fuzz")]
use moirai_protocol::state::{graph_log::GraphLog, log::BoxedLog};
use moirai_protocol::{crdt::query::Read, state::po_log::VecLog, utils::boxer::Boxer};
use rand::Rng;

#[cfg(feature = "fuzz")]
use crate::classifiers::{
    JsonKind, JsonKindChild, JsonKindChildValue, JsonKindContainer, JsonKindLog, JsonKindValue,
};
use crate::package::{Json, JsonLog};

#[cfg(feature = "fuzz")]
impl deepsize::DeepSizeOf for Json {
    fn deep_size_of_children(&self, context: &mut deepsize::Context) -> usize {
        match self {
            Json::JsonKind(op) => deepsize::DeepSizeOf::deep_size_of_children(op, context),
        }
    }
}

#[cfg(feature = "fuzz")]
impl deepsize::DeepSizeOf for JsonLog {
    fn deep_size_of_children(&self, context: &mut deepsize::Context) -> usize {
        deepsize::DeepSizeOf::deep_size_of_children(self.json_log(), context)
    }
}

#[cfg(feature = "fuzz")]
impl LogFootprint for JsonLog {
    fn footprint(&self) -> FootprintNode {
        FootprintNode::leaf(self)
    }
}

#[cfg(feature = "fuzz")]
impl CommandGenerator for JsonKindLog {
    fn generate_command(&self, rng: &mut impl Rng) -> Self::Command {
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
            JsonKind::Number(<VecLog<Counter<f64>> as CommandGenerator>::generate_command(log, rng))
        }

        fn generate_boolean(log: &VecLog<EWFlag>, rng: &mut impl Rng) -> JsonKind {
            JsonKind::Boolean(<VecLog<EWFlag> as CommandGenerator>::generate_command(
                log, rng,
            ))
        }

        fn generate_string(log: &GraphLog<List<char>>, rng: &mut impl Rng) -> JsonKind {
            JsonKind::String(<GraphLog<List<char>> as CommandGenerator>::generate_command(log, rng))
        }

        fn generate_object(log: &UWMapLog<String, JsonKindLog>, rng: &mut impl Rng) -> JsonKind {
            let op =
                <UWMapLog<String, JsonKindLog> as CommandGenerator>::generate_command(log, rng);
            JsonKind::Object(Boxer::<UWMap<String, Box<JsonKind>>>::boxer(op))
        }

        fn generate_array(
            log: &NestedListLog<BoxedLog<JsonKindLog>>,
            rng: &mut impl Rng,
        ) -> JsonKind {
            let op = <NestedListLog<BoxedLog<JsonKindLog>> as CommandGenerator>::generate_command(
                log, rng,
            );
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

        let value = self.eval(&Read::new());

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
                    Choice::String => generate_string(&GraphLog::<List<char>>::new(), rng),
                    Choice::Array => {
                        generate_array(&NestedListLog::<BoxedLog<JsonKindLog>>::new(), rng)
                    }
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

impl CommandGenerator for JsonLog {
    fn generate_command(&self, rng: &mut impl Rng) -> Self::Command {
        Json::JsonKind(self.json_log().generate_command(rng))
    }
}
