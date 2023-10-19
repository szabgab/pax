use piet_common::RenderContext;
use std::cell::RefCell;
use std::rc::Rc;

use crate::{
    HandlerRegistry, InstantiationArgs, RenderNode, RenderNodePtr, RenderNodePtrList,
    RenderTreeContext, Runtime,
};
use pax_properties_coproduct::PropertiesCoproduct;

use pax_runtime_api::{CommonProperties, Layer, Size, Timeline};

use crate::PropertiesComputable;

/// A render node with its own runtime context.  Will push a frame
/// to the runtime stack including the specified `slot_children` and
/// `PropertiesCoproduct` object.  `Component` is used at the root of
/// applications, at the root of reusable components like `Stacker`, and
/// in special applications like `Repeat` where it houses the `RepeatItem`
/// properties attached to each of Repeat's virtual nodes.
pub struct ComponentInstance<R: 'static + RenderContext> {
    pub(crate) instance_id: u32,
    pub template: RenderNodePtrList<R>,
    pub slot_children: RenderNodePtrList<R>,
    pub handler_registry: Option<Rc<RefCell<HandlerRegistry<R>>>>,
    pub properties: Rc<RefCell<PropertiesCoproduct>>,
    pub timeline: Option<Rc<RefCell<Timeline>>>,
    pub compute_properties_fn:
        Box<dyn FnMut(Rc<RefCell<PropertiesCoproduct>>, &mut RenderTreeContext<R>)>,

    pub common_properties: CommonProperties,
}

impl<R: 'static + RenderContext> RenderNode<R> for ComponentInstance<R> {
    fn get_common_properties(&self) -> &CommonProperties {
        &self.common_properties
    }
    fn get_instance_id(&self) -> u32 {
        self.instance_id
    }
    fn get_rendering_children(&self) -> RenderNodePtrList<R> {
        Rc::clone(&self.template)
    }
    fn is_component_node(&self) -> bool {
        true
    }

    fn get_handler_registry(&self) -> Option<Rc<RefCell<HandlerRegistry<R>>>> {
        match &self.handler_registry {
            Some(registry) => Some(Rc::clone(&registry)),
            _ => None,
        }
    }

    fn get_slot_children(&self) -> Option<RenderNodePtrList<R>> {
        Some(Rc::clone(&self.slot_children))
    }

    fn instantiate(args: InstantiationArgs<R>) -> Rc<RefCell<Self>> {
        let mut instance_registry = (*args.instance_registry).borrow_mut();
        let instance_id = instance_registry.mint_id();

        let template = match args.component_template {
            Some(t) => t,
            None => Rc::new(RefCell::new(vec![])),
        };

        let ret = Rc::new(RefCell::new(ComponentInstance {
            instance_id,
            template,
            slot_children: match args.children {
                Some(children) => children,
                None => Rc::new(RefCell::new(vec![])),
            },
            common_properties: args.common_properties,
            properties: Rc::new(RefCell::new(args.properties)),
            compute_properties_fn: args
                .compute_properties_fn
                .expect("must pass a compute_properties_fn to a Component instance"),
            timeline: None,
            handler_registry: args.handler_registry,
        }));

        instance_registry.register(instance_id, Rc::clone(&ret) as RenderNodePtr<R>);
        ret
    }

    fn get_size(&self) -> Option<(Size, Size)> {
        None
    }
    fn compute_size_within_bounds(&self, bounds: (f64, f64)) -> (f64, f64) {
        bounds
    }


    fn handle_will_compute_properties(&mut self, rtc: &mut RenderTreeContext<R>) {
        //expand slot_children before adding to stack frame.
        //NOTE: this requires *evaluating properties* for `should_flatten` nodes like Repeat and Conditional, whose
        //      properties must be evaluated before we can know how to handle them as slot_children



       todo!("Problem: current impl of process__* requires properties for subtree\
       to have been fully computed ahead of time (because it relies on get_rendering_children, which,\
       in examples of `for`, requires that properties are computed first.)  This shakes out as no adoptees\
       being pushed to the stack frame, which ultimately breaks computing properties inside of a for loop (via get_forwarded_children)\
       A broad potential approach is to decouple adoptees from the properties stack.\
        Along the way, think about the special treatment we're giving to handle_did_compute_properties for component\
        nodes in recurse_traverse_render_tree.  Perhaps this exact shape of the properties x stack x adoptees isn't quite there yet.");


        let non_flattened_slot_children = Rc::clone(&self.slot_children);

        let flattened_slot_children = Rc::new(RefCell::new(
            (*non_flattened_slot_children)
                .borrow()
                .iter()
                .map(|slot_child| Runtime::process__should_flatten__slot_children_recursive(slot_child, rtc))
                .flatten()
                .collect(),
        ));

        (*rtc.runtime).borrow_mut().push_stack_frame(
            Rc::clone(&flattened_slot_children),
            Rc::clone(&self.properties),
            self.timeline.clone(),
        );
    }

    fn handle_did_compute_properties(&mut self, rtc: &mut RenderTreeContext<R>) {
        (*rtc.runtime).borrow_mut().pop_stack_frame();
    }

    fn handle_compute_properties(&mut self, rtc: &mut RenderTreeContext<R>) {
        self.common_properties.compute_properties(rtc);
        (*self.compute_properties_fn)(Rc::clone(&self.properties), rtc);
    }

    fn get_layer_type(&mut self) -> Layer {
        Layer::DontCare
    }
}
