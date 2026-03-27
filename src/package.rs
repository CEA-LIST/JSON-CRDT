/// Auto-generated code by 🅰🆁🅰🅲🅷🅽🅴 - do not edit directly
mod __package {
    pub use crate::classifiers::*;
    pub use moirai_protocol::clock::version_vector::Version;
    pub use moirai_protocol::crdt::eval::EvalNested;
    pub use moirai_protocol::crdt::query::QueryOperation;
    pub use moirai_protocol::crdt::query::Read;
    pub use moirai_protocol::event::Event;
    pub use moirai_protocol::replica::ReplicaIdx;
    pub use moirai_protocol::state::log::IsLog;
    pub use moirai_protocol::utils::intern_str::Interner;
    pub use moirai_protocol::utils::translate_ids::TranslateIds;
}
#[derive(Debug, Clone)]
pub enum Json {
    Json(__package::Json),
}
#[derive(Debug, Clone, Default)]
pub struct JsonValue {
    pub json: __package::JsonValue,
}
#[derive(Debug, Clone, Default)]
pub struct JsonLog {
    json_log: __package::JsonLog,
}
impl JsonLog {
    pub fn json_log(&self) -> &__package::JsonLog {
        &self.json_log
    }
}
impl __package::IsLog for JsonLog {
    type Value = JsonValue;
    type Op = Json;
    fn is_enabled(&self, op: &Self::Op) -> bool {
        match op {
            Json::Json(o) => self.json_log.is_enabled(o),
        }
    }
    fn effect(&mut self, event: __package::Event<Self::Op>) {
        match event.op().clone() {
            Json::Json(o) => self.json_log.effect(__package::Event::unfold(event, o)),
        }
    }
    fn stabilize(&mut self, version: &__package::Version) {
        self.json_log.stabilize(version);
    }
    fn redundant_by_parent(&mut self, version: &__package::Version, conservative: bool) {
        self.json_log.redundant_by_parent(version, conservative);
    }
    fn is_default(&self) -> bool {
        self.json_log.is_default()
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
impl __package::TranslateIds for Json {
    fn translate_ids(&self, from: __package::ReplicaIdx, interner: &__package::Interner) -> Self {
        match self {
            Json::Json(op) => Json::Json(op.clone()),
        }
    }
}
