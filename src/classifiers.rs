/// Auto-generated code by 🅰🆁🅰🅲🅷🅽🅴 - do not edit directly
mod __classifiers {
    pub use moirai_crdt::counter::resettable_counter::Counter;
    pub use moirai_crdt::flag::ew_flag::EWFlag;
    pub use moirai_crdt::list::eg_walker::List;
    pub use moirai_crdt::list::nested_list::NestedList;
    pub use moirai_crdt::list::nested_list::NestedListLog;
    pub use moirai_crdt::map::uw_map::UWMap;
    pub use moirai_crdt::map::uw_map::UWMapLog;
    pub use moirai_macros::union;
    pub use moirai_protocol::state::event_graph::EventGraph;
    pub use moirai_protocol::state::po_log::VecLog;
}
type JsonArrayValue = __classifiers::NestedList<Box<Json>>;
type JsonArrayLog = __classifiers::NestedListLog<JsonLog>;
type JsonObjectValue = __classifiers::UWMap<std::string::String, Box<Json>>;
type JsonObjectLog = __classifiers::UWMapLog<std::string::String, JsonLog>;
type JsonStringValue = __classifiers::List<char>;
type JsonStringLog = __classifiers::EventGraph<__classifiers::List<char>>;
type JsonNumberValue = __classifiers::Counter<f64>;
type JsonNumberLog = __classifiers::VecLog<__classifiers::Counter<f64>>;
type JsonBooleanValue = __classifiers::EWFlag;
type JsonBooleanLog = __classifiers::VecLog<__classifiers::EWFlag>;
__classifiers::union!(
    Json = Array(JsonArrayValue, JsonArrayLog)
        | Object(JsonObjectValue, JsonObjectLog)
        | String(JsonStringValue, JsonStringLog)
        | Number(JsonNumberValue, JsonNumberLog)
        | Boolean(JsonBooleanValue, JsonBooleanLog)
);
