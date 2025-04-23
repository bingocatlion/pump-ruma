//! Types for the [`m.space.parent`] event.
//!
//! [`m.space.parent`]: https://spec.matrix.org/latest/client-server-api/#mspaceparent

use ruma_common::{OwnedRoomId, OwnedServerName};
use ruma_macros::EventContent;
use serde::{Deserialize, Serialize};

/// The content of an `m.space.parent` event.
///
/// Rooms can claim parents via the `m.space.parent` state event.
///
/// Similar to `m.space.child`, the `state_key` is the ID of the parent space, and the content must
/// contain a `via` key which gives a list of candidate servers that can be used to join the
/// parent.
#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
#[ruma_event(type = "m.space.parent", kind = State, state_key_type = OwnedRoomId)]
pub struct SpaceParentEventContent {
    /// List of candidate servers that can be used to join the room.
     pub via: Option<Vec<OwnedServerName>>,
    /// Determines whether this is the main parent for the space.
    ///
    /// When a user joins a room with a canonical parent, clients may switch to view the room in
    /// the context of that space, peeking into it in order to find other rooms and group them
    /// together. In practice, well behaved rooms should only have one `canonical` parent, but
    /// given this is not enforced: if multiple are present the client should select the one with
    /// the lowest room ID, as determined via a lexicographic ordering of the Unicode code-points.
    ///
    /// Defaults to `false`.
    #[serde(default, skip_serializing_if = "ruma_common::serde::is_default")]
    pub canonical: Option<bool>,
    pub unsigned:Option<SpaceParentEventContentUnsigned>,
}


#[derive(Clone, Debug, Default, Deserialize,Serialize)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
pub struct SpaceParentEventContentUnsignedPrveContent {
    pub via: Option<Vec<OwnedServerName>>,
    /// Determines whether this is the main parent for the space.
    ///
    /// When a user joins a room with a canonical parent, clients may switch to view the room in
    /// the context of that space, peeking into it in order to find other rooms and group them
    /// together. In practice, well behaved rooms should only have one `canonical` parent, but
    /// given this is not enforced: if multiple are present the client should select the one with
    /// the lowest room ID, as determined via a lexicographic ordering of the Unicode code-points.
    ///
    /// Defaults to `false`.
    #[serde(default, skip_serializing_if = "ruma_common::serde::is_default")]
    pub canonical: Option<bool>,
}
#[derive(Clone, Debug, Default, Deserialize,Serialize)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
pub struct SpaceParentEventContentUnsigned {
    pub prev_content:SpaceParentEventContentUnsignedPrveContent
}

impl SpaceParentEventContent {
    /// Creates a new `SpaceParentEventContent` with the given routing servers.
    pub fn new(via: Vec<OwnedServerName>) -> Self {
        Self { via:Some(via), canonical: Some(false),unsigned:None }
    }
    pub fn is_empty(&self) -> bool {
       let empty = self.via.is_none()
            &&
            self.canonical.is_none()
            && self.unsigned.is_none();
        let empty2 = self.via.as_ref().is_some_and(|v| v.is_empty())
            && self.unsigned.as_ref().is_some_and(|v|
            v.prev_content.via.is_none() || v.prev_content.via.as_ref().is_some_and(|v| v.is_empty()));
        return empty && empty2;
    }

    pub fn get_via(&self) -> Vec<String> {
        self.via.as_ref().map_or(
            self.unsigned.as_ref().map_or(Vec::new(),|m|
                m.prev_content.via.as_ref().map_or(Vec::new(),|v| {
                    v.iter().map(|id| id.to_string()).collect()
                })),|v|  v.iter().map(|id| id.to_string()).collect() )
    }

    pub fn get_canonical(&self) -> bool {
        self.canonical.map_or(self.unsigned.as_ref().map_or(false,|v| v.prev_content.canonical.map_or(false,|v| v))
        ,|v| v)
    }
}

#[cfg(test)]
mod tests {
    use ruma_common::server_name;
    use serde_json::{from_str, json, to_value as to_json_value};
    use ruma_common::exports::serde_html_form::to_string;
    use ruma_common::serde::{from_raw_json_value, Raw};
    use super::SpaceParentEventContent;

    #[test]
    fn space_parent_serialization() {
        let content = SpaceParentEventContent {
            via: Some(vec![server_name!("example.com").to_owned()]),
            canonical: Some(true),
            unsigned: None,
        };
    //  {'type': 'm.space.parent', 'sender': '@abc3:testname', 'content': {}, 'state_key': '!ZxVlcfgIiXLNRrMcHx:testname', 'origin_server_ts': 1743063313673, 'unsigned': {'replaces_state': '$4C6A2d1sPIte1c491q-rLXrQWdOmdMNODV91_tEHm6I', 'prev_content': {'via': ['testname'], 'canonical': True}, 'prev_sender': '@abc3:testname', 'membership': 'join', 'age': 2330779998}
        let json = json!({
            "via": ["example.com"],
            "canonical": true,
        });
        let textstr = r#"{"type":"m.space.parent","sender":"@abc3:testname","content":{},"state_key":"!ZxVlcfgIiXLNRrMcHx:testname","origin_server_ts":1743063313673,"unsigned":{"replaces_state":"$4C6A2d1sPIte1c491q-rLXrQWdOmdMNODV91_tEHm6I","prev_content":{"via":["testname"],"canonical":true},"prev_sender":"@abc3:testname","membership":"join","age":2330779998}}"#;
        let raw_value =  Raw::<SpaceParentEventContent>::from_json_string(textstr.to_string());
        println!("json string1 {:?}", &raw_value);
        let values = raw_value.unwrap().deserialize();
        println!("values {:?}", &values);
        let zm = values.unwrap();
        println!("via {:?}", zm.get_via());
        println!("empty {:?}", zm.is_empty());

    }

}
