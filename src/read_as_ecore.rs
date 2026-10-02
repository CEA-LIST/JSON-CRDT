/// Auto-generated code by 🅰🆁🅰🅲🅷🅽🅴 - do not edit directly
mod __read_as_ecore {
    pub use arachne_xmi::{Attributes as XmiAttributes, Writer as XmiWriter};
    pub use moirai_protocol::{
        crdt::{
            eval::EvalNested,
            query::{QueryOperation, Read},
        },
        event::id::EventId,
        state::{log::IsLog, object_path::ObjectPath},
    };

    pub use crate::{classifiers::*, package::*};
}
/// Serializes the current replicated model as XMI conforming to the
/// source Ecore package.
#[derive(Debug, Clone, Copy, Default)]
pub struct ReadAsEcore;
impl __read_as_ecore::QueryOperation for ReadAsEcore {
    type Response = Vec<u8>;
}
impl ReadAsEcore {
    pub fn new() -> Self {
        Self
    }
}
fn xmi_read<L, V>(log: &L) -> V
where
    L: __read_as_ecore::IsLog + __read_as_ecore::EvalNested<__read_as_ecore::Read<V>>,
{
    log.execute_query(&__read_as_ecore::Read::<V>::new())
}
#[allow(dead_code, unused_variables)]
fn visit_json_kind(
    writer: &mut __read_as_ecore::XmiWriter,
    element_name: Option<&str>,
    path: __read_as_ecore::ObjectPath,
    log: &__read_as_ecore::JsonKindLog,
) {
    let visit_child = |child: &__read_as_ecore::JsonKindChild,
                       writer: &mut __read_as_ecore::XmiWriter| {
        match child {
            __read_as_ecore::JsonKindChild::Array(child_log) => {
                let (element_name, emit_xmi_type) = match element_name {
                    Some(name) => (name, true),
                    None => ("json:Array", false),
                };
                {
                    let path = path.clone().variant("array");
                    let mut attrs = __read_as_ecore::XmiAttributes::for_object(&path);
                    if emit_xmi_type {
                        attrs.push_value("xsi:type", "json:Array");
                    }
                    (writer).element(element_name, &attrs, |writer| {
                        let list_base_path = path.clone();
                        let positions =
                            xmi_read::<_, Vec<__read_as_ecore::EventId>>((child_log).positions());
                        for event_id in &positions {
                            if let Some(child) = (child_log).children().get_child(event_id) {
                                let child_path =
                                    list_base_path.clone().list_element(event_id.clone());
                                visit_json_kind(writer, Some("items"), child_path, (child).inner());
                            }
                        }
                    });
                }
            }
            __read_as_ecore::JsonKindChild::Object(child_log) => {
                let (element_name, emit_xmi_type) = match element_name {
                    Some(name) => (name, true),
                    None => ("json:Object", false),
                };
                {
                    let path = path.clone().variant("object");
                    let mut attrs = __read_as_ecore::XmiAttributes::for_object(&path);
                    if emit_xmi_type {
                        attrs.push_value("xsi:type", "json:Object");
                    }
                    (writer).element(element_name, &attrs, |writer| {
                        let map_base_path = path.clone();
                        let mut entries = (child_log).children().iter().collect::<Vec<_>>();
                        entries.sort_by_key(|(key, _)| format!("{:?}", key));
                        for (key, child) in entries {
                            let entry_path = map_base_path.clone().map_entry(format!("{:?}", key));
                            let mut attrs = __read_as_ecore::XmiAttributes::for_object(&entry_path);
                            attrs.push_value("xsi:type", "json:Entry");
                            attrs.push_values("key", vec![(key).to_string()], true);
                            (writer).element("entry", &attrs, |writer| {
                                visit_json_kind(writer, Some("value"), entry_path.clone(), child);
                            });
                        }
                    });
                }
            }
            __read_as_ecore::JsonKindChild::String(child_log) => {
                let (element_name, emit_xmi_type) = match element_name {
                    Some(name) => (name, true),
                    None => ("json:String", false),
                };
                {
                    let path = path.clone().variant("string");
                    let mut attrs = __read_as_ecore::XmiAttributes::for_object(&path);
                    if emit_xmi_type {
                        attrs.push_value("xsi:type", "json:String");
                    }
                    {
                        let value = xmi_read::<_, String>(child_log);
                        attrs.push_values("value", vec![(value).to_string()], true);
                    }
                    (writer).element(element_name, &attrs, |writer| {});
                }
            }
            __read_as_ecore::JsonKindChild::Number(child_log) => {
                let (element_name, emit_xmi_type) = match element_name {
                    Some(name) => (name, true),
                    None => ("json:Number", false),
                };
                {
                    let path = path.clone().variant("number");
                    let mut attrs = __read_as_ecore::XmiAttributes::for_object(&path);
                    if emit_xmi_type {
                        attrs.push_value("xsi:type", "json:Number");
                    }
                    {
                        let value = xmi_read::<_, f64>(child_log);
                        attrs.push_values("value", vec![(value).to_string()], true);
                    }
                    (writer).element(element_name, &attrs, |writer| {});
                }
            }
            __read_as_ecore::JsonKindChild::Boolean(child_log) => {
                let (element_name, emit_xmi_type) = match element_name {
                    Some(name) => (name, true),
                    None => ("json:Boolean", false),
                };
                {
                    let path = path.clone().variant("boolean");
                    let mut attrs = __read_as_ecore::XmiAttributes::for_object(&path);
                    if emit_xmi_type {
                        attrs.push_value("xsi:type", "json:Boolean");
                    }
                    {
                        let value = xmi_read::<_, bool>(child_log);
                        attrs.push_values("value", vec![(value).to_string()], true);
                    }
                    (writer).element(element_name, &attrs, |writer| {});
                }
            }
        }
    };
    let child_rank = |child: &__read_as_ecore::JsonKindChild| match child {
        __read_as_ecore::JsonKindChild::Array(_) => 0usize,
        __read_as_ecore::JsonKindChild::Object(_) => 1usize,
        __read_as_ecore::JsonKindChild::String(_) => 2usize,
        __read_as_ecore::JsonKindChild::Number(_) => 3usize,
        __read_as_ecore::JsonKindChild::Boolean(_) => 4usize,
    };
    match &log.child {
        __read_as_ecore::JsonKindContainer::Unset => {}
        __read_as_ecore::JsonKindContainer::Value(child) => {
            visit_child(child.as_ref(), writer);
        }
        __read_as_ecore::JsonKindContainer::Conflicts(children) => {
            let mut children = children.iter().collect::<Vec<_>>();
            children.sort_by_key(|child| child_rank(child));
            for child in children {
                visit_child(child, writer);
            }
        }
    }
}
impl __read_as_ecore::EvalNested<ReadAsEcore> for __read_as_ecore::JsonLog {
    fn execute_query(
        &self,
        _q: &ReadAsEcore,
    ) -> <ReadAsEcore as __read_as_ecore::QueryOperation>::Response {
        let mut writer = __read_as_ecore::XmiWriter::new("json", "http://www.example.org/json");
        visit_json_kind(
            &mut writer,
            None,
            __read_as_ecore::ObjectPath::new("json").field("json"),
            self.json_log(),
        );
        writer.finish()
    }
}
