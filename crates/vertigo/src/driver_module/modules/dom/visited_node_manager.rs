use std::{collections::HashMap, rc::Rc};
use crate::transaction;
use crate::DomId;

use crate::struct_mut::HashMapMut;

use super::driver_data::DriverData;

struct VisitedNode {
    on_mouse_leave: Option<Rc<dyn Fn()>>,
}

impl VisitedNode {
    pub fn new(on_mouse_enter: Option<Rc<dyn Fn()>>, on_mouse_leave: Option<Rc<dyn Fn()>>) -> VisitedNode {
        if let Some(on_mouse_enter) = on_mouse_enter {
            on_mouse_enter();
        }

        VisitedNode { on_mouse_leave }
    }
}
impl Drop for VisitedNode {
    fn drop(&mut self) {
        let on_mouse_leave = std::mem::replace(&mut self.on_mouse_leave, None);

        if let Some(on_mouse_leave) = on_mouse_leave {
            on_mouse_leave();
        }
    }
}

//struktura do zarządzania ostnio odwiedzonymi węzłami

pub(crate) struct VisitedNodeManager {
    nodes: HashMapMut<DomId, VisitedNode>,
}

impl VisitedNodeManager {
    pub(crate) fn new() -> VisitedNodeManager {
        VisitedNodeManager {
            nodes: HashMapMut::new()
        }
    }

    pub fn clear(&self) {
        let VisitedNodeManager {nodes, ..} = self;

        transaction(move |_| {
            let new_state = HashMap::<DomId, VisitedNode>::new();
            let _ = nodes.mem_replace(new_state);
        });
    }

    pub fn push_new_nodes(&self, driver_data: &DriverData, new_nodes: Vec<DomId>) {
        let VisitedNodeManager {nodes} = self;

        transaction(move |_| {
            let mut new_state = HashMap::<DomId, VisitedNode>::new();

            for node_id in new_nodes {
                let old_node = nodes.remove(&node_id);

                if let Some(old_node) = old_node {
                    new_state.insert(node_id, old_node);
                    continue;
                }

                let on_enter = driver_data.get_from_node(
                    &node_id,
                    |elem| elem.on_mouse_enter.clone()
                );

                let on_leave = driver_data.get_from_node(
                    &node_id,
                    |elem| elem.on_mouse_leave.clone()
                );

                new_state.insert(node_id, VisitedNode::new(on_enter, on_leave));
            }

            nodes.mem_replace(new_state);
        });
    }
}
