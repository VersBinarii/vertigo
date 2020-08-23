use std::collections::HashMap;

use crate::vdom::models::VDomComponent::VDomComponent;

#[derive(Clone)]
pub struct VDomNode {
    pub name: String,
    pub attr: HashMap<String, String>,
    pub child: Vec<VDom>,
}


#[derive(Clone)]
pub enum VDom {
    Node {
        node: VDomNode,
    },
    Text {
        value: String,
    },
    Component {
        node: VDomComponent,
    },
}
