/// Auto-generated code by 🅰🆁🅰🅲🅷🅽🅴 - do not edit directly
mod __package {
    pub use crate::classifiers::*;
    pub use moirai_protocol::clock::version_vector::Version;
    pub use moirai_protocol::crdt::eval::EvalNested;
    pub use moirai_protocol::crdt::query::QueryOperation;
    pub use moirai_protocol::crdt::query::Read;
    pub use moirai_protocol::event::Event;
    pub use moirai_protocol::state::effect_context::EffectContext;
    pub use moirai_protocol::state::log::IsLog;
    pub use moirai_protocol::state::object_path::ObjectPath;
    pub use moirai_protocol::state::po_log::POLog;
    pub use moirai_protocol::state::sink::SinkCollector;
    pub use moirai_protocol::state::sink::SinkEffect;
    pub use moirai_protocol::utils::intern_str::InternalizeOp;
    pub use moirai_protocol::utils::intern_str::Interner;
}
#[derive(Debug, Clone)]
pub enum Json {
    JsonKind(__package::JsonKind),
}
#[derive(Debug)]
pub enum JsonRejection {
    JsonKind(<__package::JsonKindLog as __package::IsLog>::Rejection),
}
impl std::fmt::Display for JsonRejection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::JsonKind(error) => write!(f, "{}: {}", "JsonKind", error),
        }
    }
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct JsonValue {
    pub json: __package::JsonKindValue,
}
#[derive(Debug, Clone, Default)]
pub struct JsonLog {
    json_log: __package::JsonKindLog,
}
impl JsonLog {
    pub fn json_log(&self) -> &__package::JsonKindLog {
        &self.json_log
    }
}
impl __package::IsLog for JsonLog {
    type Value = JsonValue;
    type Op = Json;
    type Rejection = JsonRejection;
    fn is_enabled(&self, op: &Self::Op) -> Result<(), Self::Rejection> {
        match op {
            Json::JsonKind(o) => self.json_log.is_enabled(o).map_err(JsonRejection::JsonKind),
        }
    }
    fn effect(
        &mut self,
        event: __package::Event<Self::Op>,
        _ctx: &mut __package::EffectContext<'_>,
    ) {
        let mut ctx = __package::EffectContext::root("json", None);
        match event.op().clone() {
            Json::JsonKind(o) => {
                let child_event = __package::Event::unfold(event.clone(), o);
                ctx.with_field("json", |ctx| {
                    self.json_log.effect(child_event, ctx);
                });
            }
        }
    }
    fn stabilize(&mut self, version: &__package::Version) {
        self.json_log.stabilize(version);
    }
    fn redundant_by_parent(&mut self, version: &__package::Version, conservative: bool) {
        self.json_log.redundant_by_parent(version, conservative);
    }
    fn is_default(&self) -> bool {
        true && self.json_log.is_default()
    }
}
impl __package::EvalNested<__package::Read<<Self as __package::IsLog>::Value>> for JsonLog {
    fn execute_query(
        &self,
        _q: __package::Read<<Self as __package::IsLog>::Value>,
    ) -> <__package::Read<<Self as __package::IsLog>::Value> as __package::QueryOperation>::Response
    {
        JsonValue {
            json: self.json_log.execute_query(__package::Read::new()),
        }
    }
}
impl __package::InternalizeOp for Json {
    fn internalize(self, interner: &__package::Interner) -> Self {
        match self {
            Json::JsonKind(op) => Json::JsonKind(op.clone()),
        }
    }
}
