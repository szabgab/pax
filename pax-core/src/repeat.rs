use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;


use piet_common::RenderContext;
use crate::{ComponentInstance, TabCache, RenderNode, RenderNodePtr, RenderNodePtrList, RenderTreeContext, InstantiationArgs, HandlerRegistry};
use pax_runtime_api::{PropertyInstance, PropertyLiteral, Size2D, Transform2D};
use pax_properties_coproduct::{PropertiesCoproduct, TypesCoproduct};

/// A special "control-flow" primitive, Repeat allows for nodes
/// to be rendered dynamically per data specified in `data_list`.
/// That is: for a `data_list` of length `n`, `Repeat` will render its
/// template `n` times, each with an embedded component context (`RepeatItem`)
/// with an index `i` and a pointer to that relevant datum `data_list[i]`
pub struct RepeatInstance<R: 'static + RenderContext> {
    pub instance_id: u64,
    pub repeated_template: RenderNodePtrList<R>,
    pub transform: Rc<RefCell<dyn PropertyInstance<Transform2D>>>,
    pub data_list: Box<dyn PropertyInstance<Vec<Rc<PropertiesCoproduct>>>>,
    pub virtual_children: RenderNodePtrList<R>,
    tab_cache: TabCache<R>,
}


impl<R: 'static + RenderContext> RenderNode<R> for RepeatInstance<R> {


    fn get_tab_cache(&mut self) -> &mut TabCache<R> {
        &mut self.tab_cache
    }

    fn get_instance_id(&self) -> u64 {
        self.instance_id
    }

    fn instantiate(mut args: InstantiationArgs<R>) -> Rc<RefCell<Self>> where Self: Sized {

        let mut instance_registry = (*args.instance_registry).borrow_mut();
        let instance_id  = instance_registry.mint_id();
        let ret = Rc::new(RefCell::new(RepeatInstance {
            instance_id,
            repeated_template: match args.children {
                None => {Rc::new(RefCell::new(vec![]))}
                Some(children) => children
            },
            transform: args.transform,
            data_list: args.repeat_data_list.unwrap(),
            virtual_children: Rc::new(RefCell::new(vec![])),
            tab_cache: TabCache::new(),
        }));

        instance_registry.register(instance_id, Rc::clone(&ret) as RenderNodePtr<R>);
        ret
    }


    fn compute_properties(&mut self, rtc: &mut RenderTreeContext<R>) {

        let mut any_changes_to_data_list : bool;

        if let Some(data_list) = rtc.compute_vtable_value(self.data_list._get_vtable_id()) {
            let old_value = self.data_list.get().clone();
            let new_value = if let TypesCoproduct::Vec_Rc_PropertiesCoproduct___(v) = data_list { v } else { unreachable!() };

            //if the vec lengths differ, we know there are changes.  If the lengths are the same, then we check each element pairwise for ptr equality.
            any_changes_to_data_list =
                old_value.len() != new_value.len() ||
                {
                    let mut all_equal = true;
                    old_value.iter().enumerate().for_each(|(i, ov)| {
                        all_equal = all_equal && Rc::ptr_eq(ov, &new_value[i]);
                    });
                    !all_equal
                };

            //TODO: this hacked dirty-check shouldn't be necessary once we have more robust dependency-DAG dirty-checking for expressions
            if any_changes_to_data_list {
                self.data_list.set(new_value);
            }
        } else {
            //Assuming PropertyLiteral -- changes need to be enacted if the length of the data_list changes.
            //given that there's not currently a way to imperatively set a `Repeat`'s `data_list` property, this may be OK
            //for now (specifically: catch the case where virtual_children is uninitialized to a literal static value.)
            //More robustly in the future, this can patch into centralized dirty-check logic.
            any_changes_to_data_list = self.data_list.get().len() != (*self.virtual_children).borrow().len();
        }

        if any_changes_to_data_list {

            //Any stated children (repeat template members) of Repeat should be forwarded to the `RepeatItem`-wrapped `ComponentInstance`s
            //so that `Slot` works as expected
            let forwarded_children = match (*rtc.runtime).borrow_mut().peek_stack_frame() {
                Some(frame) => {Rc::clone(&(*frame.borrow()).get_unexpanded_adoptees())},
                None => {Rc::new(RefCell::new(vec![]))},
            };

            //unmount all old virtual_children, permanently (TODO: this can be much-optimized)
            (*(*self.virtual_children).borrow_mut()).iter_mut().for_each(|vc| {
                (*(*(*vc).borrow_mut())).borrow_mut().unmount_recursive(rtc, true);
            });

            let mut instance_registry = (*rtc.engine.instance_registry).borrow_mut();

            //reset children:
            //wrap data_list into `RepeatItems`, which attach
            //the necessary data as stack frame context
            self.virtual_children = Rc::new(RefCell::new(
                self.data_list.get().iter().enumerate().map(|(i, datum)| {
                    let instance_id = instance_registry.mint_id();

                    let render_node : RenderNodePtr<R> = Rc::new(RefCell::new(
                        ComponentInstance {
                            instance_id,
                            children: Rc::clone(&forwarded_children),
                            template: Rc::clone(&self.repeated_template),
                            transform: Rc::new(RefCell::new(PropertyLiteral::new(Transform2D::default()))),
                            properties: Rc::new(RefCell::new(PropertiesCoproduct::RepeatItem(Rc::clone(datum), i))),
                            timeline: None,
                            handler_registry: None,
                            compute_properties_fn: Box::new(|props, rtc|{
                                //no-op since the Repeat RenderNode handles the necessary calc (see `RepeatInstance::compute_properties`)
                            }),
                            tab_cache: TabCache::new(),
                        }
                    ));

                    instance_registry.register(instance_id, Rc::clone(&render_node));

                    render_node
                }).collect()
            ));
        }



        // pax_runtime_api::log(&format!("finished computing repeat properties, virt len: {}", (*self.virtual_children).borrow().len()));
    }

    fn should_flatten(&self) -> bool {
        true
    }
    fn get_rendering_children(&self) -> RenderNodePtrList<R> {
        Rc::clone(&self.virtual_children)
    }
    fn get_size(&self) -> Option<Size2D> { None }
    fn compute_size_within_bounds(&self, bounds: (f64, f64)) -> (f64, f64) { bounds }
    fn get_transform(&mut self) -> Rc<RefCell<dyn PropertyInstance<Transform2D>>> { Rc::clone(&self.transform) }

}


/*
lab journal, zb
---------------

To support polymorphic data <T> inside stack frames,
we need a `dyn SomeTrait` contract that stack frame data
can adhere to (instead of arbitrary `T`)

ex. `repeat` element stackframe data:
{
    index: usize,
    datum: T
}

We could have any stack frame abide by this contract:

StackFrameData<T> {
    get_index() -> usize;
    get_datum() -> Box<dyn T>;
}
...but how does the consumer know it's dealing with `T`?  Where does `T` come from?

Ultimately, it's userland.  E.g. consider the user-provided data:
cats = [{fur_color: Colors.WHITE, eye_color: Colors.BLUE}, {fur_color: Colors.BROWN, eye_color: Colors.GREEN}]
describes a schema and thus `T` of {fur_color: Color, eye_color: Color}

Perhaps this gets easier if we introduce our `scope` object here, and deal with a string:value (dynamic) lookup?

This turns our StackFrameData approach into:

StackFrame {
    get_scope() -> Scope;
}

along with

Scope {
    get_type(key: &str) -> PolymorphicType // used for unsafe unboxing of value
    get_value(key: &str) -> PolymorphicValue
}

When working with a Scope inside a `repeat`, the user looks up values & types by (string) key.

Seems like a suitable solution.

 */


//Can we operate on a guarantee that for `n` elements in a repeat, the consumer (expression)
//will be invoked exactly `n` times?  If so, we could push a stackframe for each datum (in reverse)
//so that each invocation consumes a new stack frame, in order.  The tricky piece of this is
//a need to introduce stack frame `pop`s somewhere before the post_eval_properties_in_place lifecycle
//method, in a way that's unique to `repeat`.

//An alternative approach to this problem, which operates with the grain of "one stack frame
//per component instance," is to add an iterator to a new RepeatPropertiesContainer, which
//yields the next `RepeatProperties` on each invocation.  This may require simply modifying
//the inject_and_evaluate logic.  Perhaps we can introduce a `.next` method on Evaluator, with
//a default implementation that's a no-op, but which Repeat can override to step through
//an iterator.

// rtc.runtime.borrow_mut().push_stack_frame(
//     Rc::clone(&self.children),
//       Box::new(Scope {
//           properties: Rc::clone(&self.properties) as Rc<dyn Any>
//       })
// );
