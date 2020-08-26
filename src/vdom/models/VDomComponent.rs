use std::fmt::Debug;
use virtualdom::computed::{
    Computed::Computed,
};

use crate::vdom::{
    models::{
        VDom::VDom,
        VDomComponentId::VDomComponentId,
    }
};

#[derive(Clone)]
pub struct VDomComponent {
    pub id: VDomComponentId,
    pub render: Computed<Vec<VDom>>,
}

impl VDomComponent {
    pub fn new<T: Debug + 'static>(params: Computed<T>, render: fn(&T) -> Vec<VDom>) -> VDomComponent {

        let componentId = VDomComponentId::new(&params, render);
        let render = params.map(render);

        VDomComponent {
            id: componentId,
            render,
        }
    }
}
