/// Auto-generated code by 🅰🆁🅰🅲🅷🅽🅴 - do not edit directly
mod __classifiers {
    pub use moirai_crdt::{
        counter::resettable_counter::Counter,
        flag::ew_flag::EWFlag,
        list::{
            eg_walker::List,
            nested_list::{NestedList, NestedListLog},
        },
        map::uw_map::{UWMap, UWMapLog},
    };
    pub use moirai_macros::union;
    pub use moirai_protocol::state::{graph_log::GraphLog, log::BoxedLog, po_log::VecLog};
}
type JsonArray = __classifiers::NestedList<Box<JsonKind>>;
type JsonArrayLog = __classifiers::NestedListLog<__classifiers::BoxedLog<JsonKindLog>>;
type JsonObject = __classifiers::UWMap<std::string::String, Box<JsonKind>>;
type JsonObjectLog = __classifiers::UWMapLog<std::string::String, JsonKindLog>;
type JsonString = __classifiers::List<char>;
type JsonStringLog = __classifiers::GraphLog<__classifiers::List<char>>;
type JsonNumber = __classifiers::Counter<f64>;
type JsonNumberLog = __classifiers::VecLog<__classifiers::Counter<f64>>;
type JsonBoolean = __classifiers::EWFlag;
type JsonBooleanLog = __classifiers::VecLog<__classifiers::EWFlag>;
__classifiers::union!(
    JsonKind = Array(JsonArray, JsonArrayLog => Vec < Box < JsonKindValue > >) |
    Object(JsonObject, JsonObjectLog => rustc_hash::FxHashMap < std::string::String,
    JsonKindValue >) | String(JsonString, JsonStringLog => String) | Number(JsonNumber,
    JsonNumberLog => f64) | Boolean(JsonBoolean, JsonBooleanLog => bool)
);
