/// Auto-generated code by 🅰🆁🅰🅲🅷🅽🅴 - do not edit directly
mod __package {
    pub use moirai_protocol::crdt::query::Read;
    pub use moirai_protocol::crdt::eval::EvalNested;
    pub use moirai_protocol::state::log::IsLog;
    pub use moirai_protocol::clock::version_vector::Version;
    pub use moirai_protocol::event::Event;
    pub use moirai_protocol::crdt::query::QueryOperation;
    pub use moirai_protocol::state::sink::SinkEffect;
    pub use moirai_protocol::state::effect_context::EffectContext;
    pub use moirai_protocol::utils::intern_str::Interner;
    pub use moirai_protocol::utils::intern_str::InternalizeOp;
    pub use moirai_protocol::state::sink::SinkCollector;
    pub use moirai_protocol::state::object_path::ObjectPath;
    pub use moirai_protocol::state::po_log::POLog;
    pub use crate::classifiers::*;
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
#[derive(Debug, Clone, Default)]
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
            Json::JsonKind(o) => {
                self.json_log.is_enabled(o).map_err(JsonRejection::JsonKind)
            }
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
                ctx.with_field(
                    "json",
                    |ctx| {
                        self.json_log.effect(child_event, ctx);
                    },
                );
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
impl __package::EvalNested<__package::Read<<Self as __package::IsLog>::Value>>
for JsonLog {
    fn execute_query(
        &self,
        _q: __package::Read<<Self as __package::IsLog>::Value>,
    ) -> <__package::Read<
        <Self as __package::IsLog>::Value,
    > as __package::QueryOperation>::Response {
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
#[derive(Debug, Clone, Copy, Default)]
pub struct ReadAsEcore;
impl __package::QueryOperation for ReadAsEcore {
    type Response = String;
}
fn __xmi_read<L>(log: &L) -> <L as __package::IsLog>::Value
where
    L: __package::IsLog
        + __package::EvalNested<__package::Read<<L as __package::IsLog>::Value>>,
{
    log.execute_query(__package::Read::<<L as __package::IsLog>::Value>::new())
}
fn __xmi_escape(value: &str) -> String {
    let mut escaped = String::new();
    for ch in value.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&apos;"),
            _ => escaped.push(ch),
        }
    }
    escaped
}
fn __xmi_indent(out: &mut String, indent: usize) {
    for _ in 0..indent {
        out.push_str("  ");
    }
}
fn __xmi_path_id(path: &__package::ObjectPath) -> String {
    let raw = path.to_string();
    let mut id = String::with_capacity(raw.len() + 3);
    id.push_str("id");
    for ch in raw.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
            id.push(ch);
        } else {
            id.push('_');
        }
    }
    id
}
fn __xmi_push_attr(
    attrs: &mut Vec<(&'static str, String)>,
    name: &'static str,
    mut values: Vec<String>,
    force: bool,
) {
    values.retain(|value| force || !value.is_empty());
    if values.is_empty() && !force {
        return;
    }
    attrs.push((name, values.join(" ")));
}
fn __xmi_write_open(
    out: &mut String,
    indent: usize,
    name: &str,
    attrs: &[(&str, String)],
    empty: bool,
) {
    __xmi_indent(out, indent);
    out.push('<');
    out.push_str(name);
    for (key, value) in attrs {
        out.push(' ');
        out.push_str(key);
        out.push_str("=\"");
        out.push_str(&__xmi_escape(value));
        out.push('"');
    }
    if empty {
        out.push_str("/>\n");
    } else {
        out.push_str(">\n");
    }
}
fn __xmi_write_close(out: &mut String, indent: usize, name: &str) {
    __xmi_indent(out, indent);
    out.push_str("</");
    out.push_str(name);
    out.push_str(">\n");
}
#[allow(dead_code, unused_variables)]
fn ecore_write_json_kind(
    out: &mut String,
    element_name: Option<&str>,
    path: __package::ObjectPath,
    log: &__package::JsonKindLog,
    indent: usize,
) {
    match &log.child {
        __package::JsonKindContainer::Unset => {}
        __package::JsonKindContainer::Value(__child) => {
            match __child.as_ref() {
                __package::JsonKindChild::Array(__child_log) => {
                    let (__element_name, __emit_xmi_type) = match element_name {
                        Some(name) => (name, true),
                        None => ("json:Array", false),
                    };
                    {
                        let __path = path.clone().variant("array");
                        let mut __attrs = Vec::new();
                        __attrs.push(("xmi:id", __xmi_path_id(&__path)));
                        if __emit_xmi_type {
                            __attrs.push(("xmi:type", "json:Array".to_string()));
                        }
                        let mut __children = String::new();
                        {
                            let __list_base_path = __path.clone();
                            let __positions = moirai_protocol::crdt::eval::BorrowedRead::read_ref(
                                __child_log.positions(),
                            );
                            for __event_id in __positions {
                                if let Some(__child) = __child_log
                                    .children()
                                    .get_child(__event_id)
                                {
                                    let __child_path = __list_base_path
                                        .clone()
                                        .list_element(__event_id.clone());
                                    ecore_write_json_kind(
                                        &mut __children,
                                        Some("items"),
                                        __child_path,
                                        __child,
                                        indent + 1,
                                    );
                                }
                            }
                        }
                        __xmi_write_open(
                            out,
                            indent,
                            __element_name,
                            &__attrs,
                            __children.is_empty(),
                        );
                        if !__children.is_empty() {
                            (out).push_str(&__children);
                            __xmi_write_close(out, indent, __element_name);
                        }
                    }
                }
                __package::JsonKindChild::Object(__child_log) => {
                    let (__element_name, __emit_xmi_type) = match element_name {
                        Some(name) => (name, true),
                        None => ("json:Object", false),
                    };
                    {
                        let __path = path.clone().variant("object");
                        let mut __attrs = Vec::new();
                        __attrs.push(("xmi:id", __xmi_path_id(&__path)));
                        if __emit_xmi_type {
                            __attrs.push(("xmi:type", "json:Object".to_string()));
                        }
                        let mut __children = String::new();
                        {
                            let __map_base_path = __path.clone();
                            let mut __entries = __child_log
                                .children()
                                .iter()
                                .collect::<Vec<_>>();
                            __entries.sort_by_key(|(__key, _)| format!("{:?}", __key));
                            for (__key, __child) in __entries {
                                let __entry_path = __map_base_path
                                    .clone()
                                    .map_entry(format!("{:?}", __key));
                                let mut __attrs = Vec::new();
                                __attrs.push(("xmi:id", __xmi_path_id(&__entry_path)));
                                __attrs.push(("xmi:type", "json:Entry".to_string()));
                                __xmi_push_attr(
                                    &mut __attrs,
                                    "key",
                                    vec![(__key).to_string()],
                                    true,
                                );
                                let mut __entry_children = String::new();
                                ecore_write_json_kind(
                                    &mut __entry_children,
                                    Some("value"),
                                    __entry_path.clone().field("value"),
                                    __child,
                                    indent + 1 + 1,
                                );
                                __xmi_write_open(
                                    &mut __children,
                                    indent + 1,
                                    "entry",
                                    &__attrs,
                                    __entry_children.is_empty(),
                                );
                                if !__entry_children.is_empty() {
                                    (&mut __children).push_str(&__entry_children);
                                    __xmi_write_close(&mut __children, indent + 1, "entry");
                                }
                            }
                        }
                        __xmi_write_open(
                            out,
                            indent,
                            __element_name,
                            &__attrs,
                            __children.is_empty(),
                        );
                        if !__children.is_empty() {
                            (out).push_str(&__children);
                            __xmi_write_close(out, indent, __element_name);
                        }
                    }
                }
                __package::JsonKindChild::String(__child_log) => {
                    let (__element_name, __emit_xmi_type) = match element_name {
                        Some(name) => (name, true),
                        None => ("json:String", false),
                    };
                    {
                        let __path = path.clone().variant("string");
                        let mut __attrs = Vec::new();
                        __attrs.push(("xmi:id", __xmi_path_id(&__path)));
                        if __emit_xmi_type {
                            __attrs.push(("xmi:type", "json:String".to_string()));
                        }
                        let mut __children = String::new();
                        {
                            let value = __xmi_read(__child_log);
                            __xmi_push_attr(
                                &mut __attrs,
                                "value",
                                vec![(value).iter().collect:: < String > ()],
                                true,
                            );
                        }
                        __xmi_write_open(
                            out,
                            indent,
                            __element_name,
                            &__attrs,
                            __children.is_empty(),
                        );
                        if !__children.is_empty() {
                            (out).push_str(&__children);
                            __xmi_write_close(out, indent, __element_name);
                        }
                    }
                }
                __package::JsonKindChild::Number(__child_log) => {
                    let (__element_name, __emit_xmi_type) = match element_name {
                        Some(name) => (name, true),
                        None => ("json:Number", false),
                    };
                    {
                        let __path = path.clone().variant("number");
                        let mut __attrs = Vec::new();
                        __attrs.push(("xmi:id", __xmi_path_id(&__path)));
                        if __emit_xmi_type {
                            __attrs.push(("xmi:type", "json:Number".to_string()));
                        }
                        let mut __children = String::new();
                        {
                            let value = __xmi_read(__child_log);
                            __xmi_push_attr(
                                &mut __attrs,
                                "value",
                                vec![(value).to_string()],
                                true,
                            );
                        }
                        __xmi_write_open(
                            out,
                            indent,
                            __element_name,
                            &__attrs,
                            __children.is_empty(),
                        );
                        if !__children.is_empty() {
                            (out).push_str(&__children);
                            __xmi_write_close(out, indent, __element_name);
                        }
                    }
                }
                __package::JsonKindChild::Boolean(__child_log) => {
                    let (__element_name, __emit_xmi_type) = match element_name {
                        Some(name) => (name, true),
                        None => ("json:Boolean", false),
                    };
                    {
                        let __path = path.clone().variant("boolean");
                        let mut __attrs = Vec::new();
                        __attrs.push(("xmi:id", __xmi_path_id(&__path)));
                        if __emit_xmi_type {
                            __attrs.push(("xmi:type", "json:Boolean".to_string()));
                        }
                        let mut __children = String::new();
                        {
                            let value = __xmi_read(__child_log);
                            __xmi_push_attr(
                                &mut __attrs,
                                "value",
                                vec![(value).to_string()],
                                true,
                            );
                        }
                        __xmi_write_open(
                            out,
                            indent,
                            __element_name,
                            &__attrs,
                            __children.is_empty(),
                        );
                        if !__children.is_empty() {
                            (out).push_str(&__children);
                            __xmi_write_close(out, indent, __element_name);
                        }
                    }
                }
            }
        }
        __package::JsonKindContainer::Conflicts(__children) => {
            for __child in __children {
                match __child {
                    __package::JsonKindChild::Array(__child_log) => {
                        let (__element_name, __emit_xmi_type) = match element_name {
                            Some(name) => (name, true),
                            None => ("json:Array", false),
                        };
                        {
                            let __path = path.clone().variant("array");
                            let mut __attrs = Vec::new();
                            __attrs.push(("xmi:id", __xmi_path_id(&__path)));
                            if __emit_xmi_type {
                                __attrs.push(("xmi:type", "json:Array".to_string()));
                            }
                            let mut __children = String::new();
                            {
                                let __list_base_path = __path.clone();
                                let __positions = moirai_protocol::crdt::eval::BorrowedRead::read_ref(
                                    __child_log.positions(),
                                );
                                for __event_id in __positions {
                                    if let Some(__child) = __child_log
                                        .children()
                                        .get_child(__event_id)
                                    {
                                        let __child_path = __list_base_path
                                            .clone()
                                            .list_element(__event_id.clone());
                                        ecore_write_json_kind(
                                            &mut __children,
                                            Some("items"),
                                            __child_path,
                                            __child,
                                            indent + 1,
                                        );
                                    }
                                }
                            }
                            __xmi_write_open(
                                out,
                                indent,
                                __element_name,
                                &__attrs,
                                __children.is_empty(),
                            );
                            if !__children.is_empty() {
                                (out).push_str(&__children);
                                __xmi_write_close(out, indent, __element_name);
                            }
                        }
                    }
                    __package::JsonKindChild::Object(__child_log) => {
                        let (__element_name, __emit_xmi_type) = match element_name {
                            Some(name) => (name, true),
                            None => ("json:Object", false),
                        };
                        {
                            let __path = path.clone().variant("object");
                            let mut __attrs = Vec::new();
                            __attrs.push(("xmi:id", __xmi_path_id(&__path)));
                            if __emit_xmi_type {
                                __attrs.push(("xmi:type", "json:Object".to_string()));
                            }
                            let mut __children = String::new();
                            {
                                let __map_base_path = __path.clone();
                                let mut __entries = __child_log
                                    .children()
                                    .iter()
                                    .collect::<Vec<_>>();
                                __entries.sort_by_key(|(__key, _)| format!("{:?}", __key));
                                for (__key, __child) in __entries {
                                    let __entry_path = __map_base_path
                                        .clone()
                                        .map_entry(format!("{:?}", __key));
                                    let mut __attrs = Vec::new();
                                    __attrs.push(("xmi:id", __xmi_path_id(&__entry_path)));
                                    __attrs.push(("xmi:type", "json:Entry".to_string()));
                                    __xmi_push_attr(
                                        &mut __attrs,
                                        "key",
                                        vec![(__key).to_string()],
                                        true,
                                    );
                                    let mut __entry_children = String::new();
                                    ecore_write_json_kind(
                                        &mut __entry_children,
                                        Some("value"),
                                        __entry_path.clone().field("value"),
                                        __child,
                                        indent + 1 + 1,
                                    );
                                    __xmi_write_open(
                                        &mut __children,
                                        indent + 1,
                                        "entry",
                                        &__attrs,
                                        __entry_children.is_empty(),
                                    );
                                    if !__entry_children.is_empty() {
                                        (&mut __children).push_str(&__entry_children);
                                        __xmi_write_close(&mut __children, indent + 1, "entry");
                                    }
                                }
                            }
                            __xmi_write_open(
                                out,
                                indent,
                                __element_name,
                                &__attrs,
                                __children.is_empty(),
                            );
                            if !__children.is_empty() {
                                (out).push_str(&__children);
                                __xmi_write_close(out, indent, __element_name);
                            }
                        }
                    }
                    __package::JsonKindChild::String(__child_log) => {
                        let (__element_name, __emit_xmi_type) = match element_name {
                            Some(name) => (name, true),
                            None => ("json:String", false),
                        };
                        {
                            let __path = path.clone().variant("string");
                            let mut __attrs = Vec::new();
                            __attrs.push(("xmi:id", __xmi_path_id(&__path)));
                            if __emit_xmi_type {
                                __attrs.push(("xmi:type", "json:String".to_string()));
                            }
                            let mut __children = String::new();
                            {
                                let value = __xmi_read(__child_log);
                                __xmi_push_attr(
                                    &mut __attrs,
                                    "value",
                                    vec![(value).iter().collect:: < String > ()],
                                    true,
                                );
                            }
                            __xmi_write_open(
                                out,
                                indent,
                                __element_name,
                                &__attrs,
                                __children.is_empty(),
                            );
                            if !__children.is_empty() {
                                (out).push_str(&__children);
                                __xmi_write_close(out, indent, __element_name);
                            }
                        }
                    }
                    __package::JsonKindChild::Number(__child_log) => {
                        let (__element_name, __emit_xmi_type) = match element_name {
                            Some(name) => (name, true),
                            None => ("json:Number", false),
                        };
                        {
                            let __path = path.clone().variant("number");
                            let mut __attrs = Vec::new();
                            __attrs.push(("xmi:id", __xmi_path_id(&__path)));
                            if __emit_xmi_type {
                                __attrs.push(("xmi:type", "json:Number".to_string()));
                            }
                            let mut __children = String::new();
                            {
                                let value = __xmi_read(__child_log);
                                __xmi_push_attr(
                                    &mut __attrs,
                                    "value",
                                    vec![(value).to_string()],
                                    true,
                                );
                            }
                            __xmi_write_open(
                                out,
                                indent,
                                __element_name,
                                &__attrs,
                                __children.is_empty(),
                            );
                            if !__children.is_empty() {
                                (out).push_str(&__children);
                                __xmi_write_close(out, indent, __element_name);
                            }
                        }
                    }
                    __package::JsonKindChild::Boolean(__child_log) => {
                        let (__element_name, __emit_xmi_type) = match element_name {
                            Some(name) => (name, true),
                            None => ("json:Boolean", false),
                        };
                        {
                            let __path = path.clone().variant("boolean");
                            let mut __attrs = Vec::new();
                            __attrs.push(("xmi:id", __xmi_path_id(&__path)));
                            if __emit_xmi_type {
                                __attrs.push(("xmi:type", "json:Boolean".to_string()));
                            }
                            let mut __children = String::new();
                            {
                                let value = __xmi_read(__child_log);
                                __xmi_push_attr(
                                    &mut __attrs,
                                    "value",
                                    vec![(value).to_string()],
                                    true,
                                );
                            }
                            __xmi_write_open(
                                out,
                                indent,
                                __element_name,
                                &__attrs,
                                __children.is_empty(),
                            );
                            if !__children.is_empty() {
                                (out).push_str(&__children);
                                __xmi_write_close(out, indent, __element_name);
                            }
                        }
                    }
                }
            }
        }
    }
}
impl __package::EvalNested<ReadAsEcore> for JsonLog {
    fn execute_query(
        &self,
        _q: ReadAsEcore,
    ) -> <ReadAsEcore as __package::QueryOperation>::Response {
        let mut out = String::new();
        out.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        let root_attrs = vec![
            ("xmi:version", "2.0".to_string()), ("xmlns:xmi", "http://www.omg.org/XMI"
            .to_string()), ("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance"
            .to_string()), (concat!("xmlns:", "json"), "http://www.example.org/json"
            .to_string()),
        ];
        __xmi_write_open(&mut out, 0, "xmi:XMI", &root_attrs, false);
        ecore_write_json_kind(
            &mut out,
            None,
            __package::ObjectPath::new("json").field("json"),
            &self.json_log,
            1,
        );
        __xmi_write_close(&mut out, 0, "xmi:XMI");
        out
    }
}
