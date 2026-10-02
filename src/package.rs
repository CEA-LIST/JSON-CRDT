/// Auto-generated code by 🅰🆁🅰🅲🅷🅽🅴 - do not edit directly
mod __package {
    pub use moirai_protocol::{
        clock::{causal_frontier::CausalFrontier, version_vector::Version},
        crdt::{eval::EvalNested, query::Read},
        event::Event as ProtocolEvent,
        state::{effect_context::EffectContext, log::IsLog},
    };
}
#[derive(Debug, Clone)]
pub enum Json {
    JsonKind(crate::classifiers::JsonKind),
}
#[derive(Debug)]
pub enum JsonRejection {
    JsonKind(<crate::classifiers::JsonKindLog as __package::IsLog>::Rejection),
}
impl std::fmt::Display for JsonRejection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::JsonKind(error) => write!(f, "JsonKind: {}", error),
        }
    }
}
#[derive(Debug, Clone, Default)]
pub struct JsonValue {
    pub json: crate::classifiers::JsonKindValue,
}
#[derive(Debug, Clone, Default)]
pub struct JsonLog {
    json_log: crate::classifiers::JsonKindLog,
}
impl JsonLog {
    pub fn json_log(&self) -> &crate::classifiers::JsonKindLog {
        &self.json_log
    }
}
impl __package::IsLog for JsonLog {
    type Command = Json;
    type Op = Json;
    type Rejection = JsonRejection;
    fn prepare(&self, command: Self::Command) -> Self::Op {
        command
    }
    fn is_enabled(&self, op: &Self::Op) -> Result<(), Self::Rejection> {
        match op {
            Json::JsonKind(o) => self.json_log.is_enabled(o).map_err(JsonRejection::JsonKind),
        }
    }
    fn effect(
        &mut self,
        event: __package::ProtocolEvent<Self::Op>,
        _ctx: &mut __package::EffectContext<'_>,
    ) {
        let mut ctx = __package::EffectContext::root("json", None);
        match event.op().clone() {
            Json::JsonKind(o) => {
                let child_event = __package::ProtocolEvent::unfold(event.clone(), o);
                ctx.with_field("json", |ctx| {
                    self.json_log.effect(child_event, ctx);
                });
            }
        }
    }
    fn stabilize(&mut self, frontier: &__package::CausalFrontier) {
        self.json_log.stabilize(frontier);
    }
    fn redundant_by_parent(&mut self, version: &__package::Version, conservative: bool) {
        self.json_log.redundant_by_parent(version, conservative);
    }
    fn is_default(&self) -> bool {
        true && self.json_log.is_default()
    }
}
impl __package::EvalNested<__package::Read<JsonValue>> for JsonLog {
    fn execute_query(&self, _q: &__package::Read<JsonValue>) -> JsonValue {
        JsonValue {
            json: self
                .json_log
                .execute_query(&__package::Read::<crate::classifiers::JsonKindValue>::new()),
        }
    }
}
