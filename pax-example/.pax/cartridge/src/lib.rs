//Prelude: Rust
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::ops::Deref;
use std::rc::Rc;
//Prelude: Pax
use pax_core::pax_properties_coproduct::{PropertiesCoproduct, TypesCoproduct};
use pax_core::repeat::RepeatInstance;
use pax_core::{
    ComponentInstance, ConditionalInstance, ExpressionContext, HandlerRegistry, InstanceRegistry,
    InstantiationArgs, PaxEngine, PropertyExpression, RenderNode, RenderNodePtr, RenderNodePtrList,
    RenderTreeContext, SlotInstance, StackFrame,
};
use pax_runtime_api::numeric::Numeric;
use pax_runtime_api::{PropertyInstance, PropertyLiteral, Size, Size2D, SizePixels, Transform2D};
use piet_common::RenderContext;

// generate imports, pointing to userland cartridge `pub mod pax_reexports`

use pax_example::pax_reexports::grids::RectDef;

use pax_example::pax_reexports::Example;

use pax_example::pax_reexports::pax_std::types::Color;

use pax_example::pax_reexports::pax_std::types::Stroke;

use pax_example::pax_reexports::grids::Grids;

use pax_example::pax_reexports::fireworks::Fireworks;

use pax_example::pax_reexports::pax_std::primitives::Frame;

use pax_example::pax_reexports::f64;

use pax_example::pax_reexports::pax_std::primitives::Group;

use pax_example::pax_reexports::pax_std::primitives::Ellipse;

use pax_example::pax_reexports::std::vec::Vec;

use pax_example::pax_reexports::camera::Camera;

use pax_example::pax_reexports::hello_rgb::HelloRGB;

use pax_example::pax_reexports::pax_std::primitives::Rectangle;

use pax_example::pax_reexports::camera::TypeExample;

use pax_example::pax_reexports::usize;

//pull in entire const token stream here e.g. `const JABBERWOCKY : &str = r#"’Twas brillig, and the slithy toves `...

pub fn instantiate_expression_table<R: 'static + RenderContext>(
) -> HashMap<usize, Box<dyn Fn(ExpressionContext<R>) -> TypesCoproduct>> {
    let mut vtable: HashMap<usize, Box<dyn Fn(ExpressionContext<R>) -> TypesCoproduct>> =
        HashMap::new();

    //Transform2D::scale(zoom,zoom)*Transform2D::translate(pan_x,pan_y)
    vtable.insert(
        0,
        Box::new(|ec: ExpressionContext<R>| -> TypesCoproduct {
            let pan_x = {
                let properties = if let Some(sf) = (*ec.stack_frame).borrow().peek_nth(0) {
                    Rc::clone(&sf)
                } else {
                    Rc::clone(&ec.stack_frame)
                }
                .borrow()
                .deref()
                .get_properties();
                let properties = &*(*properties).borrow();

                if let PropertiesCoproduct::Camera(p) = properties {
                    Numeric::from(p.pan_x.get())
                } else {
                    unreachable!()
                }
            };

            let pan_y = {
                let properties = if let Some(sf) = (*ec.stack_frame).borrow().peek_nth(0) {
                    Rc::clone(&sf)
                } else {
                    Rc::clone(&ec.stack_frame)
                }
                .borrow()
                .deref()
                .get_properties();
                let properties = &*(*properties).borrow();

                if let PropertiesCoproduct::Camera(p) = properties {
                    Numeric::from(p.pan_y.get())
                } else {
                    unreachable!()
                }
            };

            let zoom = {
                let properties = if let Some(sf) = (*ec.stack_frame).borrow().peek_nth(0) {
                    Rc::clone(&sf)
                } else {
                    Rc::clone(&ec.stack_frame)
                }
                .borrow()
                .deref()
                .get_properties();
                let properties = &*(*properties).borrow();

                if let PropertiesCoproduct::Camera(p) = properties {
                    Numeric::from(p.zoom.get())
                } else {
                    unreachable!()
                }
            };

            #[allow(unused_parens)]
            TypesCoproduct::Transform2D(
                (Transform2D::scale((zoom), (zoom))
                    * (Transform2D::translate((pan_x), (pan_y))).into()),
            )
        }),
    );

    //Transform2D::translate(0,0)
    vtable.insert(
        1,
        Box::new(|ec: ExpressionContext<R>| -> TypesCoproduct {
            #[allow(unused_parens)]
            TypesCoproduct::Transform2D(Transform2D::translate(
                (Numeric::from(0)),
                (Numeric::from(0)),
            ))
        }),
    );

    //Color::rgb(100.0,0,0)
    vtable.insert(
        2,
        Box::new(|ec: ExpressionContext<R>| -> TypesCoproduct {
            #[allow(unused_parens)]
            TypesCoproduct::pax_stdCOCOtypesCOCOColor(Color::rgb(
                (Numeric::from(100.0)),
                (Numeric::from(0)),
                (Numeric::from(0)),
            ))
        }),
    );

    //Transform2D::translate(0,200)
    vtable.insert(
        3,
        Box::new(|ec: ExpressionContext<R>| -> TypesCoproduct {
            #[allow(unused_parens)]
            TypesCoproduct::Transform2D(Transform2D::translate(
                (Numeric::from(0)),
                (Numeric::from(200)),
            ))
        }),
    );

    //Color::rgb(0,100.0,0)
    vtable.insert(
        4,
        Box::new(|ec: ExpressionContext<R>| -> TypesCoproduct {
            #[allow(unused_parens)]
            TypesCoproduct::pax_stdCOCOtypesCOCOColor(Color::rgb(
                (Numeric::from(0)),
                (Numeric::from(100.0)),
                (Numeric::from(0)),
            ))
        }),
    );

    //Color::rgb(0,0,100.0)
    vtable.insert(
        5,
        Box::new(|ec: ExpressionContext<R>| -> TypesCoproduct {
            #[allow(unused_parens)]
            TypesCoproduct::pax_stdCOCOtypesCOCOColor(Color::rgb(
                (Numeric::from(0)),
                (Numeric::from(0)),
                (Numeric::from(100.0)),
            ))
        }),
    );

    //Transform2D::translate(200,0)
    vtable.insert(
        6,
        Box::new(|ec: ExpressionContext<R>| -> TypesCoproduct {
            #[allow(unused_parens)]
            TypesCoproduct::Transform2D(Transform2D::translate(
                (Numeric::from(200)),
                (Numeric::from(0)),
            ))
        }),
    );

    //Color::rgb(0,50.0,50.0)
    vtable.insert(
        7,
        Box::new(|ec: ExpressionContext<R>| -> TypesCoproduct {
            #[allow(unused_parens)]
            TypesCoproduct::pax_stdCOCOtypesCOCOColor(Color::rgb(
                (Numeric::from(0)),
                (Numeric::from(50.0)),
                (Numeric::from(50.0)),
            ))
        }),
    );

    //Transform2D::translate(200,200)
    vtable.insert(
        8,
        Box::new(|ec: ExpressionContext<R>| -> TypesCoproduct {
            #[allow(unused_parens)]
            TypesCoproduct::Transform2D(Transform2D::translate(
                (Numeric::from(200)),
                (Numeric::from(200)),
            ))
        }),
    );

    //current_route==0
    vtable.insert(
        9,
        Box::new(|ec: ExpressionContext<R>| -> TypesCoproduct {
            let current_route = {
                let properties = if let Some(sf) = (*ec.stack_frame).borrow().peek_nth(0) {
                    Rc::clone(&sf)
                } else {
                    Rc::clone(&ec.stack_frame)
                }
                .borrow()
                .deref()
                .get_properties();
                let properties = &*(*properties).borrow();

                if let PropertiesCoproduct::Example(p) = properties {
                    Numeric::from(p.current_route.get())
                } else {
                    unreachable!()
                }
            };

            #[allow(unused_parens)]
            TypesCoproduct::bool((current_route == Numeric::from(0)))
        }),
    );

    //current_route==1
    vtable.insert(
        10,
        Box::new(|ec: ExpressionContext<R>| -> TypesCoproduct {
            let current_route = {
                let properties = if let Some(sf) = (*ec.stack_frame).borrow().peek_nth(0) {
                    Rc::clone(&sf)
                } else {
                    Rc::clone(&ec.stack_frame)
                }
                .borrow()
                .deref()
                .get_properties();
                let properties = &*(*properties).borrow();

                if let PropertiesCoproduct::Example(p) = properties {
                    Numeric::from(p.current_route.get())
                } else {
                    unreachable!()
                }
            };

            #[allow(unused_parens)]
            TypesCoproduct::bool((current_route == Numeric::from(1)))
        }),
    );

    //current_route==2
    vtable.insert(
        11,
        Box::new(|ec: ExpressionContext<R>| -> TypesCoproduct {
            let current_route = {
                let properties = if let Some(sf) = (*ec.stack_frame).borrow().peek_nth(0) {
                    Rc::clone(&sf)
                } else {
                    Rc::clone(&ec.stack_frame)
                }
                .borrow()
                .deref()
                .get_properties();
                let properties = &*(*properties).borrow();

                if let PropertiesCoproduct::Example(p) = properties {
                    Numeric::from(p.current_route.get())
                } else {
                    unreachable!()
                }
            };

            #[allow(unused_parens)]
            TypesCoproduct::bool((current_route == Numeric::from(2)))
        }),
    );

    //current_route==3
    vtable.insert(
        12,
        Box::new(|ec: ExpressionContext<R>| -> TypesCoproduct {
            let current_route = {
                let properties = if let Some(sf) = (*ec.stack_frame).borrow().peek_nth(0) {
                    Rc::clone(&sf)
                } else {
                    Rc::clone(&ec.stack_frame)
                }
                .borrow()
                .deref()
                .get_properties();
                let properties = &*(*properties).borrow();

                if let PropertiesCoproduct::Example(p) = properties {
                    Numeric::from(p.current_route.get())
                } else {
                    unreachable!()
                }
            };

            #[allow(unused_parens)]
            TypesCoproduct::bool((current_route == Numeric::from(3)))
        }),
    );

    //rects
    vtable.insert(
        13,
        Box::new(|ec: ExpressionContext<R>| -> TypesCoproduct {
            let rects = {
                let properties = if let Some(sf) = (*ec.stack_frame).borrow().peek_nth(1) {
                    Rc::clone(&sf)
                } else {
                    Rc::clone(&ec.stack_frame)
                }
                .borrow()
                .deref()
                .get_properties();
                let properties = &*(*properties).borrow();

                if let PropertiesCoproduct::Grids(p) = properties {
                    p.rects.get().clone()
                } else {
                    unreachable!()
                }
            };

            #[allow(unused_parens)]
            TypesCoproduct::Vec_Rc_PropertiesCoproduct___(rects)
        }),
    );

    //Transform2D::translate(elem.x,elem.y)
    vtable.insert(
        14,
        Box::new(|ec: ExpressionContext<R>| -> TypesCoproduct {
            let elemPERIx = {
                let properties = if let Some(sf) = (*ec.stack_frame).borrow().peek_nth(0) {
                    Rc::clone(&sf)
                } else {
                    Rc::clone(&ec.stack_frame)
                }
                .borrow()
                .deref()
                .get_properties();
                let properties = &*(*properties).borrow();

                if let PropertiesCoproduct::RepeatItem(elem, i) = properties {
                    Rc::clone(elem)
                } else {
                    unreachable!()
                }
            };

            let elemPERIy = {
                let properties = if let Some(sf) = (*ec.stack_frame).borrow().peek_nth(0) {
                    Rc::clone(&sf)
                } else {
                    Rc::clone(&ec.stack_frame)
                }
                .borrow()
                .deref()
                .get_properties();
                let properties = &*(*properties).borrow();

                if let PropertiesCoproduct::RepeatItem(elem, i) = properties {
                    Rc::clone(elem)
                } else {
                    unreachable!()
                }
            };

            #[allow(unused_parens)]
            TypesCoproduct::Transform2D(Transform2D::translate((elemPERIx), (elemPERIy)))
        }),
    );

    //elem.width
    vtable.insert(
        15,
        Box::new(|ec: ExpressionContext<R>| -> TypesCoproduct {
            let elemPERIwidth = {
                let properties = if let Some(sf) = (*ec.stack_frame).borrow().peek_nth(0) {
                    Rc::clone(&sf)
                } else {
                    Rc::clone(&ec.stack_frame)
                }
                .borrow()
                .deref()
                .get_properties();
                let properties = &*(*properties).borrow();

                if let PropertiesCoproduct::RepeatItem(elem, i) = properties {
                    Rc::clone(elem)
                } else {
                    unreachable!()
                }
            };

            #[allow(unused_parens)]
            TypesCoproduct::Size(elemPERIwidth)
        }),
    );

    //elem.height
    vtable.insert(
        16,
        Box::new(|ec: ExpressionContext<R>| -> TypesCoproduct {
            let elemPERIheight = {
                let properties = if let Some(sf) = (*ec.stack_frame).borrow().peek_nth(0) {
                    Rc::clone(&sf)
                } else {
                    Rc::clone(&ec.stack_frame)
                }
                .borrow()
                .deref()
                .get_properties();
                let properties = &*(*properties).borrow();

                if let PropertiesCoproduct::RepeatItem(elem, i) = properties {
                    Rc::clone(elem)
                } else {
                    unreachable!()
                }
            };

            #[allow(unused_parens)]
            TypesCoproduct::Size(elemPERIheight)
        }),
    );

    //0..60
    vtable.insert(
        17,
        Box::new(|ec: ExpressionContext<R>| -> TypesCoproduct {
            #[allow(unused_parens)]
            TypesCoproduct::Range_isize_(0..60)
        }),
    );

    //Color::hlc(ticks+i*360.0/30.0,75.0,150.0)
    vtable.insert(
        18,
        Box::new(|ec: ExpressionContext<R>| -> TypesCoproduct {
            let i = {
                let properties = if let Some(sf) = (*ec.stack_frame).borrow().peek_nth(0) {
                    Rc::clone(&sf)
                } else {
                    Rc::clone(&ec.stack_frame)
                }
                .borrow()
                .deref()
                .get_properties();
                let properties = &*(*properties).borrow();

                if let PropertiesCoproduct::RepeatItem(elem, i) = properties {
                    if let PropertiesCoproduct::isize(i) = **elem {
                        Numeric::from(i)
                    } else {
                        unreachable!()
                    }
                } else {
                    unreachable!()
                }
            };

            let ticks = {
                let properties = if let Some(sf) = (*ec.stack_frame).borrow().peek_nth(1) {
                    Rc::clone(&sf)
                } else {
                    Rc::clone(&ec.stack_frame)
                }
                .borrow()
                .deref()
                .get_properties();
                let properties = &*(*properties).borrow();

                if let PropertiesCoproduct::Fireworks(p) = properties {
                    Numeric::from(p.ticks.get())
                } else {
                    unreachable!()
                }
            };

            #[allow(unused_parens)]
            TypesCoproduct::pax_stdCOCOtypesCOCOColor(Color::hlc(
                (ticks + ((i * (Numeric::from(360.0)).into()) / Numeric::from(30.0))),
                (Numeric::from(75.0)),
                (Numeric::from(150.0)),
            ))
        }),
    );

    //Transform2D::anchor(50%,50%)*Transform2D::align(50%,50%)*Transform2D::rotate((i+2)*rotation+ticks/1000.0)*Transform2D::scale(0.75+(i*rotation),0.75+(i*rotation))*Transform2D::scale(1-((rotation/5)+i/1000.0),1-((rotation/5)+i/1000.0))
    vtable.insert(
        19,
        Box::new(|ec: ExpressionContext<R>| -> TypesCoproduct {
            let i = {
                let properties = if let Some(sf) = (*ec.stack_frame).borrow().peek_nth(0) {
                    Rc::clone(&sf)
                } else {
                    Rc::clone(&ec.stack_frame)
                }
                .borrow()
                .deref()
                .get_properties();
                let properties = &*(*properties).borrow();

                if let PropertiesCoproduct::RepeatItem(elem, i) = properties {
                    if let PropertiesCoproduct::isize(i) = **elem {
                        Numeric::from(i)
                    } else {
                        unreachable!()
                    }
                } else {
                    unreachable!()
                }
            };

            let rotation = {
                let properties = if let Some(sf) = (*ec.stack_frame).borrow().peek_nth(1) {
                    Rc::clone(&sf)
                } else {
                    Rc::clone(&ec.stack_frame)
                }
                .borrow()
                .deref()
                .get_properties();
                let properties = &*(*properties).borrow();

                if let PropertiesCoproduct::Fireworks(p) = properties {
                    Numeric::from(p.rotation.get())
                } else {
                    unreachable!()
                }
            };

            let ticks = {
                let properties = if let Some(sf) = (*ec.stack_frame).borrow().peek_nth(1) {
                    Rc::clone(&sf)
                } else {
                    Rc::clone(&ec.stack_frame)
                }
                .borrow()
                .deref()
                .get_properties();
                let properties = &*(*properties).borrow();

                if let PropertiesCoproduct::Fireworks(p) = properties {
                    Numeric::from(p.ticks.get())
                } else {
                    unreachable!()
                }
            };

            #[allow(unused_parens)]
            TypesCoproduct::Transform2D(
                ((((Transform2D::anchor((Size::Percent(50.into())), (Size::Percent(50.into())))
                    * (Transform2D::align(
                        (Size::Percent(50.into())),
                        (Size::Percent(50.into())),
                    ))
                    .into())
                    * (Transform2D::rotate(
                        (((i + Numeric::from(2)) * (rotation).into())
                            + (ticks / Numeric::from(1000.0))),
                    ))
                    .into())
                    * (Transform2D::scale(
                        (Numeric::from(0.75) + (i * (rotation).into())),
                        (Numeric::from(0.75) + (i * (rotation).into())),
                    ))
                    .into())
                    * (Transform2D::scale(
                        (Numeric::from(1)
                            - ((rotation / Numeric::from(5)) + (i / Numeric::from(1000.0)))),
                        (Numeric::from(1)
                            - ((rotation / Numeric::from(5)) + (i / Numeric::from(1000.0)))),
                    ))
                    .into()),
            )
        }),
    );

    //Transform2D::align(50%,50%)*Transform2D::anchor(50%,50%)*Transform2D::rotate(rotation)
    vtable.insert(
        20,
        Box::new(|ec: ExpressionContext<R>| -> TypesCoproduct {
            let rotation = {
                let properties = if let Some(sf) = (*ec.stack_frame).borrow().peek_nth(0) {
                    Rc::clone(&sf)
                } else {
                    Rc::clone(&ec.stack_frame)
                }
                .borrow()
                .deref()
                .get_properties();
                let properties = &*(*properties).borrow();

                if let PropertiesCoproduct::HelloRGB(p) = properties {
                    Numeric::from(p.rotation.get())
                } else {
                    unreachable!()
                }
            };

            #[allow(unused_parens)]
            TypesCoproduct::Transform2D(
                ((Transform2D::align((Size::Percent(50.into())), (Size::Percent(50.into())))
                    * (Transform2D::anchor(
                        (Size::Percent(50.into())),
                        (Size::Percent(50.into())),
                    ))
                    .into())
                    * (Transform2D::rotate((rotation))).into()),
            )
        }),
    );

    //Color::rgb(0.4,0.5,0)
    vtable.insert(
        21,
        Box::new(|ec: ExpressionContext<R>| -> TypesCoproduct {
            #[allow(unused_parens)]
            TypesCoproduct::pax_stdCOCOtypesCOCOColor(Color::rgb(
                (Numeric::from(0.4)),
                (Numeric::from(0.5)),
                (Numeric::from(0)),
            ))
        }),
    );

    vtable
}

//Begin component factory literals

pub fn instantiate_crate_camera_Camera<R: 'static + RenderContext>(
    instance_registry: Rc<RefCell<InstanceRegistry<R>>>,
    mut args: InstantiationArgs<R>,
) -> Rc<RefCell<ComponentInstance<R>>> {
    args.component_template = Some(Rc::new(RefCell::new(vec![
        pax_std_primitives::frame::FrameInstance::instantiate(InstantiationArgs {
            properties: PropertiesCoproduct::Frame(Frame {}),
            handler_registry: Some(Rc::new(RefCell::new(HandlerRegistry {
                click_handlers: vec![|stack_frame, ctx, args| {
                    let properties = (*stack_frame).borrow().get_properties();
                    let properties = &mut *properties.as_ref().borrow_mut();
                    let properties = if let PropertiesCoproduct::Camera(p) = properties {
                        p
                    } else {
                        unreachable!()
                    };
                    Camera::handle_click(properties, ctx, args);
                }],
                will_render_handlers: vec![],
                did_mount_handlers: vec![],
                scroll_handlers: vec![],
            }))),
            instance_registry: Rc::clone(&instance_registry),
            transform: Rc::new(RefCell::new(PropertyLiteral::new(Default::default()))),
            size: Some(Rc::new(RefCell::new([
                Box::new(PropertyLiteral::new(Size::Percent(100.into()))),
                Box::new(PropertyLiteral::new(Size::Percent(100.into()))),
            ]))),
            children: Some(Rc::new(RefCell::new(vec![
                pax_std_primitives::group::GroupInstance::instantiate(InstantiationArgs {
                    properties: PropertiesCoproduct::Group(Group {}),
                    handler_registry: Some(Rc::new(RefCell::new(HandlerRegistry {
                        click_handlers: vec![],
                        will_render_handlers: vec![],
                        did_mount_handlers: vec![],
                        scroll_handlers: vec![],
                    }))),
                    instance_registry: Rc::clone(&instance_registry),
                    transform: Rc::new(RefCell::new(PropertyExpression::new(0))),
                    size: Some(Rc::new(RefCell::new([
                        Box::new(PropertyLiteral::new(Default::default())),
                        Box::new(PropertyLiteral::new(Default::default())),
                    ]))),
                    children: Some(Rc::new(RefCell::new(vec![
                        pax_std_primitives::rectangle::RectangleInstance::instantiate(
                            InstantiationArgs {
                                properties: PropertiesCoproduct::Rectangle(Rectangle {
                                    stroke: Box::new(PropertyLiteral::new(Default::default())),

                                    fill: Box::new(PropertyExpression::new(2)),
                                }),
                                handler_registry: Some(Rc::new(RefCell::new(HandlerRegistry {
                                    click_handlers: vec![],
                                    will_render_handlers: vec![],
                                    did_mount_handlers: vec![],
                                    scroll_handlers: vec![],
                                }))),
                                instance_registry: Rc::clone(&instance_registry),
                                transform: Rc::new(RefCell::new(PropertyExpression::new(1))),
                                size: Some(Rc::new(RefCell::new([
                                    Box::new(PropertyLiteral::new(Size::Pixels(100.into()))),
                                    Box::new(PropertyLiteral::new(Size::Pixels(100.into()))),
                                ]))),
                                children: Some(Rc::new(RefCell::new(vec![]))),
                                component_template: None,
                                scroller_args: None,
                                slot_index: None,
                                repeat_source_expression_vec: None,
                                repeat_source_expression_range: None,
                                conditional_boolean_expression: None,
                                compute_properties_fn: None,
                            },
                        ),
                        pax_std_primitives::rectangle::RectangleInstance::instantiate(
                            InstantiationArgs {
                                properties: PropertiesCoproduct::Rectangle(Rectangle {
                                    stroke: Box::new(PropertyLiteral::new(Default::default())),

                                    fill: Box::new(PropertyExpression::new(4)),
                                }),
                                handler_registry: Some(Rc::new(RefCell::new(HandlerRegistry {
                                    click_handlers: vec![],
                                    will_render_handlers: vec![],
                                    did_mount_handlers: vec![],
                                    scroll_handlers: vec![],
                                }))),
                                instance_registry: Rc::clone(&instance_registry),
                                transform: Rc::new(RefCell::new(PropertyExpression::new(3))),
                                size: Some(Rc::new(RefCell::new([
                                    Box::new(PropertyLiteral::new(Size::Pixels(100.into()))),
                                    Box::new(PropertyLiteral::new(Size::Pixels(100.into()))),
                                ]))),
                                children: Some(Rc::new(RefCell::new(vec![]))),
                                component_template: None,
                                scroller_args: None,
                                slot_index: None,
                                repeat_source_expression_vec: None,
                                repeat_source_expression_range: None,
                                conditional_boolean_expression: None,
                                compute_properties_fn: None,
                            },
                        ),
                        pax_std_primitives::rectangle::RectangleInstance::instantiate(
                            InstantiationArgs {
                                properties: PropertiesCoproduct::Rectangle(Rectangle {
                                    stroke: Box::new(PropertyLiteral::new(Default::default())),

                                    fill: Box::new(PropertyExpression::new(5)),
                                }),
                                handler_registry: Some(Rc::new(RefCell::new(HandlerRegistry {
                                    click_handlers: vec![],
                                    will_render_handlers: vec![],
                                    did_mount_handlers: vec![],
                                    scroll_handlers: vec![],
                                }))),
                                instance_registry: Rc::clone(&instance_registry),
                                transform: Rc::new(RefCell::new(PropertyExpression::new(6))),
                                size: Some(Rc::new(RefCell::new([
                                    Box::new(PropertyLiteral::new(Size::Pixels(100.into()))),
                                    Box::new(PropertyLiteral::new(Size::Pixels(100.into()))),
                                ]))),
                                children: Some(Rc::new(RefCell::new(vec![]))),
                                component_template: None,
                                scroller_args: None,
                                slot_index: None,
                                repeat_source_expression_vec: None,
                                repeat_source_expression_range: None,
                                conditional_boolean_expression: None,
                                compute_properties_fn: None,
                            },
                        ),
                        pax_std_primitives::ellipse::EllipseInstance::instantiate(
                            InstantiationArgs {
                                properties: PropertiesCoproduct::Ellipse(Ellipse {
                                    stroke: Box::new(PropertyLiteral::new(Default::default())),

                                    fill: Box::new(PropertyExpression::new(7)),
                                }),
                                handler_registry: Some(Rc::new(RefCell::new(HandlerRegistry {
                                    click_handlers: vec![],
                                    will_render_handlers: vec![],
                                    did_mount_handlers: vec![],
                                    scroll_handlers: vec![],
                                }))),
                                instance_registry: Rc::clone(&instance_registry),
                                transform: Rc::new(RefCell::new(PropertyExpression::new(8))),
                                size: Some(Rc::new(RefCell::new([
                                    Box::new(PropertyLiteral::new(Size::Pixels(100.into()))),
                                    Box::new(PropertyLiteral::new(Size::Pixels(100.into()))),
                                ]))),
                                children: Some(Rc::new(RefCell::new(vec![]))),
                                component_template: None,
                                scroller_args: None,
                                slot_index: None,
                                repeat_source_expression_vec: None,
                                repeat_source_expression_range: None,
                                conditional_boolean_expression: None,
                                compute_properties_fn: None,
                            },
                        ),
                    ]))),
                    component_template: None,
                    scroller_args: None,
                    slot_index: None,
                    repeat_source_expression_vec: None,
                    repeat_source_expression_range: None,
                    conditional_boolean_expression: None,
                    compute_properties_fn: None,
                }),
            ]))),
            component_template: None,
            scroller_args: None,
            slot_index: None,
            repeat_source_expression_vec: None,
            repeat_source_expression_range: None,
            conditional_boolean_expression: None,
            compute_properties_fn: None,
        }),
    ])));

    args.handler_registry = Some(Rc::new(RefCell::new(HandlerRegistry {
        click_handlers: vec![],
        will_render_handlers: vec![|properties, ctx| {
            let properties = &mut *properties.as_ref().borrow_mut();
            let properties = if let PropertiesCoproduct::Camera(p) = properties {
                p
            } else {
                unreachable!()
            };
            Camera::handle_will_render(properties, ctx);
        }],
        did_mount_handlers: vec![|properties, ctx| {
            let properties = &mut *properties.as_ref().borrow_mut();
            let properties = if let PropertiesCoproduct::Camera(p) = properties {
                p
            } else {
                unreachable!()
            };
            Camera::handle_did_mount(properties, ctx);
        }],
        scroll_handlers: vec![],
    })));

    args.compute_properties_fn = Some(Box::new(|properties, rtc| {
        let properties = &mut *properties.as_ref().borrow_mut();
        let properties = if let PropertiesCoproduct::Camera(p) = properties {
            p
        } else {
            unreachable!()
        };

        if let Some(new_value) = rtc.compute_eased_value(properties.ticks._get_transition_manager())
        {
            properties.ticks.set(new_value);
        } else if let Some(new_value) = rtc.compute_vtable_value(properties.ticks._get_vtable_id())
        {
            let new_value = if let TypesCoproduct::usize(v) = new_value {
                v
            } else {
                unreachable!()
            };
            properties.ticks.set(new_value);
        }

        if let Some(new_value) = rtc.compute_eased_value(properties.zoom._get_transition_manager())
        {
            properties.zoom.set(new_value);
        } else if let Some(new_value) = rtc.compute_vtable_value(properties.zoom._get_vtable_id()) {
            let new_value = if let TypesCoproduct::f64(v) = new_value {
                v
            } else {
                unreachable!()
            };
            properties.zoom.set(new_value);
        }

        if let Some(new_value) = rtc.compute_eased_value(properties.pan_x._get_transition_manager())
        {
            properties.pan_x.set(new_value);
        } else if let Some(new_value) = rtc.compute_vtable_value(properties.pan_x._get_vtable_id())
        {
            let new_value = if let TypesCoproduct::f64(v) = new_value {
                v
            } else {
                unreachable!()
            };
            properties.pan_x.set(new_value);
        }

        if let Some(new_value) = rtc.compute_eased_value(properties.pan_y._get_transition_manager())
        {
            properties.pan_y.set(new_value);
        } else if let Some(new_value) = rtc.compute_vtable_value(properties.pan_y._get_vtable_id())
        {
            let new_value = if let TypesCoproduct::f64(v) = new_value {
                v
            } else {
                unreachable!()
            };
            properties.pan_y.set(new_value);
        }

        if let Some(new_value) =
            rtc.compute_eased_value(properties.type_example._get_transition_manager())
        {
            properties.type_example.set(new_value);
        } else if let Some(new_value) =
            rtc.compute_vtable_value(properties.type_example._get_vtable_id())
        {
            let new_value = if let TypesCoproduct::crateCOCOcameraCOCOTypeExample(v) = new_value {
                v
            } else {
                unreachable!()
            };
            properties.type_example.set(new_value);
        }
    }));

    ComponentInstance::instantiate(args)
}

pub fn instantiate_main_component<R: 'static + RenderContext>(
    instance_registry: Rc<RefCell<InstanceRegistry<R>>>,
) -> Rc<RefCell<ComponentInstance<R>>> {
    ComponentInstance::instantiate(InstantiationArgs {
        properties: PropertiesCoproduct::Example(Example::default()),
        handler_registry: Some(Rc::new(RefCell::new(HandlerRegistry {
            click_handlers: vec![],
            will_render_handlers: vec![],
            did_mount_handlers: vec![],
            scroll_handlers: vec![],
        }))),
        instance_registry: Rc::clone(&instance_registry),
        transform: Transform2D::default_wrapped(),
        size: None,
        children: None,
        component_template: Some(Rc::new(RefCell::new(vec![
            pax_std_primitives::frame::FrameInstance::instantiate(InstantiationArgs {
                properties: PropertiesCoproduct::Frame(Frame {}),
                handler_registry: Some(Rc::new(RefCell::new(HandlerRegistry {
                    click_handlers: vec![|stack_frame, ctx, args| {
                        let properties = (*stack_frame).borrow().get_properties();
                        let properties = &mut *properties.as_ref().borrow_mut();
                        let properties = if let PropertiesCoproduct::Example(p) = properties {
                            p
                        } else {
                            unreachable!()
                        };
                        Example::modulate(properties, ctx, args);
                    }],
                    will_render_handlers: vec![],
                    did_mount_handlers: vec![],
                    scroll_handlers: vec![],
                }))),
                instance_registry: Rc::clone(&instance_registry),
                transform: Rc::new(RefCell::new(PropertyLiteral::new(Default::default()))),
                size: Some(Rc::new(RefCell::new([
                    Box::new(PropertyLiteral::new(Size::Percent(100.into()))),
                    Box::new(PropertyLiteral::new(Size::Percent(100.into()))),
                ]))),
                children: Some(Rc::new(RefCell::new(vec![
                    ConditionalInstance::instantiate(InstantiationArgs {
                        properties: PropertiesCoproduct::None,
                        handler_registry: Some(Rc::new(RefCell::new(HandlerRegistry {
                            click_handlers: vec![],
                            will_render_handlers: vec![],
                            did_mount_handlers: vec![],
                            scroll_handlers: vec![],
                        }))),
                        instance_registry: Rc::clone(&instance_registry),
                        transform: Rc::new(RefCell::new(PropertyLiteral::new(Default::default()))),
                        size: Some(Rc::new(RefCell::new([
                            Box::new(PropertyLiteral::new(Default::default())),
                            Box::new(PropertyLiteral::new(Default::default())),
                        ]))),
                        children: Some(Rc::new(RefCell::new(vec![instantiate_crate_grids_Grids(
                            Rc::clone(&instance_registry),
                            InstantiationArgs {
                                properties: PropertiesCoproduct::Grids(Grids {
                                    ticks: Box::new(PropertyLiteral::new(Default::default())),

                                    rects: Box::new(PropertyLiteral::new(Default::default())),
                                }),
                                handler_registry: Some(Rc::new(RefCell::new(HandlerRegistry {
                                    click_handlers: vec![],
                                    will_render_handlers: vec![],
                                    did_mount_handlers: vec![],
                                    scroll_handlers: vec![],
                                }))),
                                instance_registry: Rc::clone(&instance_registry),
                                transform: Rc::new(RefCell::new(PropertyLiteral::new(
                                    Default::default(),
                                ))),
                                size: Some(Rc::new(RefCell::new([
                                    Box::new(PropertyLiteral::new(Default::default())),
                                    Box::new(PropertyLiteral::new(Default::default())),
                                ]))),
                                children: Some(Rc::new(RefCell::new(vec![]))),
                                component_template: None,
                                scroller_args: None,
                                slot_index: None,
                                repeat_source_expression_vec: None,
                                repeat_source_expression_range: None,
                                conditional_boolean_expression: None,
                                compute_properties_fn: None,
                            },
                        )]))),
                        component_template: None,
                        scroller_args: None,
                        slot_index: None,
                        repeat_source_expression_vec: None,
                        repeat_source_expression_range: None,
                        conditional_boolean_expression: Some(Box::new(PropertyExpression::new(9))),
                        compute_properties_fn: None,
                    }),
                    ConditionalInstance::instantiate(InstantiationArgs {
                        properties: PropertiesCoproduct::None,
                        handler_registry: Some(Rc::new(RefCell::new(HandlerRegistry {
                            click_handlers: vec![],
                            will_render_handlers: vec![],
                            did_mount_handlers: vec![],
                            scroll_handlers: vec![],
                        }))),
                        instance_registry: Rc::clone(&instance_registry),
                        transform: Rc::new(RefCell::new(PropertyLiteral::new(Default::default()))),
                        size: Some(Rc::new(RefCell::new([
                            Box::new(PropertyLiteral::new(Default::default())),
                            Box::new(PropertyLiteral::new(Default::default())),
                        ]))),
                        children: Some(Rc::new(RefCell::new(vec![
                            instantiate_crate_fireworks_Fireworks(
                                Rc::clone(&instance_registry),
                                InstantiationArgs {
                                    properties: PropertiesCoproduct::Fireworks(Fireworks {
                                        rotation: Box::new(
                                            PropertyLiteral::new(Default::default()),
                                        ),

                                        ticks: Box::new(PropertyLiteral::new(Default::default())),
                                    }),
                                    handler_registry: Some(Rc::new(RefCell::new(
                                        HandlerRegistry {
                                            click_handlers: vec![],
                                            will_render_handlers: vec![],
                                            did_mount_handlers: vec![],
                                            scroll_handlers: vec![],
                                        },
                                    ))),
                                    instance_registry: Rc::clone(&instance_registry),
                                    transform: Rc::new(RefCell::new(PropertyLiteral::new(
                                        Default::default(),
                                    ))),
                                    size: Some(Rc::new(RefCell::new([
                                        Box::new(PropertyLiteral::new(Default::default())),
                                        Box::new(PropertyLiteral::new(Default::default())),
                                    ]))),
                                    children: Some(Rc::new(RefCell::new(vec![]))),
                                    component_template: None,
                                    scroller_args: None,
                                    slot_index: None,
                                    repeat_source_expression_vec: None,
                                    repeat_source_expression_range: None,
                                    conditional_boolean_expression: None,
                                    compute_properties_fn: None,
                                },
                            ),
                        ]))),
                        component_template: None,
                        scroller_args: None,
                        slot_index: None,
                        repeat_source_expression_vec: None,
                        repeat_source_expression_range: None,
                        conditional_boolean_expression: Some(Box::new(PropertyExpression::new(10))),
                        compute_properties_fn: None,
                    }),
                    ConditionalInstance::instantiate(InstantiationArgs {
                        properties: PropertiesCoproduct::None,
                        handler_registry: Some(Rc::new(RefCell::new(HandlerRegistry {
                            click_handlers: vec![],
                            will_render_handlers: vec![],
                            did_mount_handlers: vec![],
                            scroll_handlers: vec![],
                        }))),
                        instance_registry: Rc::clone(&instance_registry),
                        transform: Rc::new(RefCell::new(PropertyLiteral::new(Default::default()))),
                        size: Some(Rc::new(RefCell::new([
                            Box::new(PropertyLiteral::new(Default::default())),
                            Box::new(PropertyLiteral::new(Default::default())),
                        ]))),
                        children: Some(Rc::new(RefCell::new(vec![
                            instantiate_crate_hello_rgb_HelloRGB(
                                Rc::clone(&instance_registry),
                                InstantiationArgs {
                                    properties: PropertiesCoproduct::HelloRGB(HelloRGB {
                                        rotation: Box::new(
                                            PropertyLiteral::new(Default::default()),
                                        ),
                                    }),
                                    handler_registry: Some(Rc::new(RefCell::new(
                                        HandlerRegistry {
                                            click_handlers: vec![],
                                            will_render_handlers: vec![],
                                            did_mount_handlers: vec![],
                                            scroll_handlers: vec![],
                                        },
                                    ))),
                                    instance_registry: Rc::clone(&instance_registry),
                                    transform: Rc::new(RefCell::new(PropertyLiteral::new(
                                        Default::default(),
                                    ))),
                                    size: Some(Rc::new(RefCell::new([
                                        Box::new(PropertyLiteral::new(Default::default())),
                                        Box::new(PropertyLiteral::new(Default::default())),
                                    ]))),
                                    children: Some(Rc::new(RefCell::new(vec![]))),
                                    component_template: None,
                                    scroller_args: None,
                                    slot_index: None,
                                    repeat_source_expression_vec: None,
                                    repeat_source_expression_range: None,
                                    conditional_boolean_expression: None,
                                    compute_properties_fn: None,
                                },
                            ),
                        ]))),
                        component_template: None,
                        scroller_args: None,
                        slot_index: None,
                        repeat_source_expression_vec: None,
                        repeat_source_expression_range: None,
                        conditional_boolean_expression: Some(Box::new(PropertyExpression::new(11))),
                        compute_properties_fn: None,
                    }),
                    ConditionalInstance::instantiate(InstantiationArgs {
                        properties: PropertiesCoproduct::None,
                        handler_registry: Some(Rc::new(RefCell::new(HandlerRegistry {
                            click_handlers: vec![],
                            will_render_handlers: vec![],
                            did_mount_handlers: vec![],
                            scroll_handlers: vec![],
                        }))),
                        instance_registry: Rc::clone(&instance_registry),
                        transform: Rc::new(RefCell::new(PropertyLiteral::new(Default::default()))),
                        size: Some(Rc::new(RefCell::new([
                            Box::new(PropertyLiteral::new(Default::default())),
                            Box::new(PropertyLiteral::new(Default::default())),
                        ]))),
                        children: Some(Rc::new(RefCell::new(vec![
                            instantiate_crate_camera_Camera(
                                Rc::clone(&instance_registry),
                                InstantiationArgs {
                                    properties: PropertiesCoproduct::Camera(Camera {
                                        ticks: Box::new(PropertyLiteral::new(Default::default())),

                                        zoom: Box::new(PropertyLiteral::new(Default::default())),

                                        pan_x: Box::new(PropertyLiteral::new(Default::default())),

                                        pan_y: Box::new(PropertyLiteral::new(Default::default())),

                                        type_example: Box::new(PropertyLiteral::new(
                                            Default::default(),
                                        )),
                                    }),
                                    handler_registry: Some(Rc::new(RefCell::new(
                                        HandlerRegistry {
                                            click_handlers: vec![],
                                            will_render_handlers: vec![],
                                            did_mount_handlers: vec![],
                                            scroll_handlers: vec![],
                                        },
                                    ))),
                                    instance_registry: Rc::clone(&instance_registry),
                                    transform: Rc::new(RefCell::new(PropertyLiteral::new(
                                        Default::default(),
                                    ))),
                                    size: Some(Rc::new(RefCell::new([
                                        Box::new(PropertyLiteral::new(Default::default())),
                                        Box::new(PropertyLiteral::new(Default::default())),
                                    ]))),
                                    children: Some(Rc::new(RefCell::new(vec![]))),
                                    component_template: None,
                                    scroller_args: None,
                                    slot_index: None,
                                    repeat_source_expression_vec: None,
                                    repeat_source_expression_range: None,
                                    conditional_boolean_expression: None,
                                    compute_properties_fn: None,
                                },
                            ),
                        ]))),
                        component_template: None,
                        scroller_args: None,
                        slot_index: None,
                        repeat_source_expression_vec: None,
                        repeat_source_expression_range: None,
                        conditional_boolean_expression: Some(Box::new(PropertyExpression::new(12))),
                        compute_properties_fn: None,
                    }),
                ]))),
                component_template: None,
                scroller_args: None,
                slot_index: None,
                repeat_source_expression_vec: None,
                repeat_source_expression_range: None,
                conditional_boolean_expression: None,
                compute_properties_fn: None,
            }),
        ]))),
        scroller_args: None,
        slot_index: None,
        repeat_source_expression_vec: None,
        repeat_source_expression_range: None,
        conditional_boolean_expression: None,
        compute_properties_fn: Some(Box::new(|properties, rtc| {
            let properties = &mut *properties.as_ref().borrow_mut();
            let properties = if let PropertiesCoproduct::Example(p) = properties {
                p
            } else {
                unreachable!()
            };

            if let Some(new_value) =
                rtc.compute_eased_value(properties.current_route._get_transition_manager())
            {
                properties.current_route.set(new_value);
            } else if let Some(new_value) =
                rtc.compute_vtable_value(properties.current_route._get_vtable_id())
            {
                let new_value = if let TypesCoproduct::usize(v) = new_value {
                    v
                } else {
                    unreachable!()
                };
                properties.current_route.set(new_value);
            }
        })),
    })
}

pub fn instantiate_crate_grids_Grids<R: 'static + RenderContext>(
    instance_registry: Rc<RefCell<InstanceRegistry<R>>>,
    mut args: InstantiationArgs<R>,
) -> Rc<RefCell<ComponentInstance<R>>> {
    args.component_template = Some(Rc::new(RefCell::new(vec![RepeatInstance::instantiate(
        InstantiationArgs {
            properties: PropertiesCoproduct::None,
            handler_registry: Some(Rc::new(RefCell::new(HandlerRegistry {
                click_handlers: vec![],
                will_render_handlers: vec![],
                did_mount_handlers: vec![],
                scroll_handlers: vec![],
            }))),
            instance_registry: Rc::clone(&instance_registry),
            transform: Rc::new(RefCell::new(PropertyLiteral::new(Default::default()))),
            size: Some(Rc::new(RefCell::new([
                Box::new(PropertyLiteral::new(Default::default())),
                Box::new(PropertyLiteral::new(Default::default())),
            ]))),
            children: Some(Rc::new(RefCell::new(vec![
                pax_std_primitives::rectangle::RectangleInstance::instantiate(InstantiationArgs {
                    properties: PropertiesCoproduct::Rectangle(Rectangle {
                        stroke: Box::new(PropertyLiteral::new(Default::default())),

                        fill: Box::new(PropertyLiteral::new(Default::default())),
                    }),
                    handler_registry: Some(Rc::new(RefCell::new(HandlerRegistry {
                        click_handlers: vec![],
                        will_render_handlers: vec![],
                        did_mount_handlers: vec![],
                        scroll_handlers: vec![],
                    }))),
                    instance_registry: Rc::clone(&instance_registry),
                    transform: Rc::new(RefCell::new(PropertyExpression::new(14))),
                    size: Some(Rc::new(RefCell::new([
                        Box::new(PropertyExpression::new(15)),
                        Box::new(PropertyExpression::new(16)),
                    ]))),
                    children: Some(Rc::new(RefCell::new(vec![]))),
                    component_template: None,
                    scroller_args: None,
                    slot_index: None,
                    repeat_source_expression_vec: None,
                    repeat_source_expression_range: None,
                    conditional_boolean_expression: None,
                    compute_properties_fn: None,
                }),
            ]))),
            component_template: None,
            scroller_args: None,
            slot_index: None,
            repeat_source_expression_vec: Some(Box::new(PropertyExpression::new(13))),
            repeat_source_expression_range: None,
            conditional_boolean_expression: None,
            compute_properties_fn: None,
        },
    )])));

    args.handler_registry = Some(Rc::new(RefCell::new(HandlerRegistry {
        click_handlers: vec![],
        will_render_handlers: vec![],
        did_mount_handlers: vec![|properties, ctx| {
            let properties = &mut *properties.as_ref().borrow_mut();
            let properties = if let PropertiesCoproduct::Grids(p) = properties {
                p
            } else {
                unreachable!()
            };
            Grids::handle_did_mount(properties, ctx);
        }],
        scroll_handlers: vec![],
    })));

    args.compute_properties_fn = Some(Box::new(|properties, rtc| {
        let properties = &mut *properties.as_ref().borrow_mut();
        let properties = if let PropertiesCoproduct::Grids(p) = properties {
            p
        } else {
            unreachable!()
        };

        if let Some(new_value) = rtc.compute_eased_value(properties.ticks._get_transition_manager())
        {
            properties.ticks.set(new_value);
        } else if let Some(new_value) = rtc.compute_vtable_value(properties.ticks._get_vtable_id())
        {
            let new_value = if let TypesCoproduct::usize(v) = new_value {
                v
            } else {
                unreachable!()
            };
            properties.ticks.set(new_value);
        }

        if let Some(new_value) = rtc.compute_eased_value(properties.rects._get_transition_manager())
        {
            properties.rects.set(new_value);
        } else if let Some(new_value) = rtc.compute_vtable_value(properties.rects._get_vtable_id())
        {
            let new_value =
                if let TypesCoproduct::stdCOCOvecCOCOVecLABRcrateCOCOgridsCOCORectDefRABR(v) =
                    new_value
                {
                    v
                } else {
                    unreachable!()
                };
            properties.rects.set(new_value);
        }
    }));

    ComponentInstance::instantiate(args)
}

pub fn instantiate_crate_fireworks_Fireworks<R: 'static + RenderContext>(
    instance_registry: Rc<RefCell<InstanceRegistry<R>>>,
    mut args: InstantiationArgs<R>,
) -> Rc<RefCell<ComponentInstance<R>>> {
    args.component_template = Some(Rc::new(RefCell::new(vec![
        pax_std_primitives::group::GroupInstance::instantiate(InstantiationArgs {
            properties: PropertiesCoproduct::Group(Group {}),
            handler_registry: Some(Rc::new(RefCell::new(HandlerRegistry {
                click_handlers: vec![],
                will_render_handlers: vec![],
                did_mount_handlers: vec![],
                scroll_handlers: vec![|stack_frame, ctx, args| {
                    let properties = (*stack_frame).borrow().get_properties();
                    let properties = &mut *properties.as_ref().borrow_mut();
                    let properties = if let PropertiesCoproduct::Fireworks(p) = properties {
                        p
                    } else {
                        unreachable!()
                    };
                    Fireworks::handle_scroll(properties, ctx, args);
                }],
            }))),
            instance_registry: Rc::clone(&instance_registry),
            transform: Rc::new(RefCell::new(PropertyLiteral::new(Default::default()))),
            size: Some(Rc::new(RefCell::new([
                Box::new(PropertyLiteral::new(Default::default())),
                Box::new(PropertyLiteral::new(Default::default())),
            ]))),
            children: Some(Rc::new(RefCell::new(vec![RepeatInstance::instantiate(
                InstantiationArgs {
                    properties: PropertiesCoproduct::None,
                    handler_registry: Some(Rc::new(RefCell::new(HandlerRegistry {
                        click_handlers: vec![],
                        will_render_handlers: vec![],
                        did_mount_handlers: vec![],
                        scroll_handlers: vec![],
                    }))),
                    instance_registry: Rc::clone(&instance_registry),
                    transform: Rc::new(RefCell::new(PropertyLiteral::new(Default::default()))),
                    size: Some(Rc::new(RefCell::new([
                        Box::new(PropertyLiteral::new(Default::default())),
                        Box::new(PropertyLiteral::new(Default::default())),
                    ]))),
                    children: Some(Rc::new(RefCell::new(vec![
                        pax_std_primitives::rectangle::RectangleInstance::instantiate(
                            InstantiationArgs {
                                properties: PropertiesCoproduct::Rectangle(Rectangle {
                                    stroke: Box::new(PropertyLiteral::new(Default::default())),

                                    fill: Box::new(PropertyExpression::new(18)),
                                }),
                                handler_registry: Some(Rc::new(RefCell::new(HandlerRegistry {
                                    click_handlers: vec![],
                                    will_render_handlers: vec![],
                                    did_mount_handlers: vec![],
                                    scroll_handlers: vec![],
                                }))),
                                instance_registry: Rc::clone(&instance_registry),
                                transform: Rc::new(RefCell::new(PropertyExpression::new(19))),
                                size: Some(Rc::new(RefCell::new([
                                    Box::new(PropertyLiteral::new(Size::Pixels(300.into()))),
                                    Box::new(PropertyLiteral::new(Size::Pixels(300.into()))),
                                ]))),
                                children: Some(Rc::new(RefCell::new(vec![]))),
                                component_template: None,
                                scroller_args: None,
                                slot_index: None,
                                repeat_source_expression_vec: None,
                                repeat_source_expression_range: None,
                                conditional_boolean_expression: None,
                                compute_properties_fn: None,
                            },
                        ),
                    ]))),
                    component_template: None,
                    scroller_args: None,
                    slot_index: None,
                    repeat_source_expression_vec: None,
                    repeat_source_expression_range: Some(Box::new(PropertyExpression::new(17))),
                    conditional_boolean_expression: None,
                    compute_properties_fn: None,
                },
            )]))),
            component_template: None,
            scroller_args: None,
            slot_index: None,
            repeat_source_expression_vec: None,
            repeat_source_expression_range: None,
            conditional_boolean_expression: None,
            compute_properties_fn: None,
        }),
    ])));

    args.handler_registry = Some(Rc::new(RefCell::new(HandlerRegistry {
        click_handlers: vec![],
        will_render_handlers: vec![|properties, ctx| {
            let properties = &mut *properties.as_ref().borrow_mut();
            let properties = if let PropertiesCoproduct::Fireworks(p) = properties {
                p
            } else {
                unreachable!()
            };
            Fireworks::handle_will_render(properties, ctx);
        }],
        did_mount_handlers: vec![],
        scroll_handlers: vec![],
    })));

    args.compute_properties_fn = Some(Box::new(|properties, rtc| {
        let properties = &mut *properties.as_ref().borrow_mut();
        let properties = if let PropertiesCoproduct::Fireworks(p) = properties {
            p
        } else {
            unreachable!()
        };

        if let Some(new_value) =
            rtc.compute_eased_value(properties.rotation._get_transition_manager())
        {
            properties.rotation.set(new_value);
        } else if let Some(new_value) =
            rtc.compute_vtable_value(properties.rotation._get_vtable_id())
        {
            let new_value = if let TypesCoproduct::f64(v) = new_value {
                v
            } else {
                unreachable!()
            };
            properties.rotation.set(new_value);
        }

        if let Some(new_value) = rtc.compute_eased_value(properties.ticks._get_transition_manager())
        {
            properties.ticks.set(new_value);
        } else if let Some(new_value) = rtc.compute_vtable_value(properties.ticks._get_vtable_id())
        {
            let new_value = if let TypesCoproduct::usize(v) = new_value {
                v
            } else {
                unreachable!()
            };
            properties.ticks.set(new_value);
        }
    }));

    ComponentInstance::instantiate(args)
}

pub fn instantiate_crate_hello_rgb_HelloRGB<R: 'static + RenderContext>(
    instance_registry: Rc<RefCell<InstanceRegistry<R>>>,
    mut args: InstantiationArgs<R>,
) -> Rc<RefCell<ComponentInstance<R>>> {
    args.component_template = Some(Rc::new(RefCell::new(vec![
        pax_std_primitives::ellipse::EllipseInstance::instantiate(InstantiationArgs {
            properties: PropertiesCoproduct::Ellipse(Ellipse {
                stroke: Box::new(PropertyLiteral::new(Default::default())),

                fill: Box::new(PropertyExpression::new(21)),
            }),
            handler_registry: Some(Rc::new(RefCell::new(HandlerRegistry {
                click_handlers: vec![|stack_frame, ctx, args| {
                    let properties = (*stack_frame).borrow().get_properties();
                    let properties = &mut *properties.as_ref().borrow_mut();
                    let properties = if let PropertiesCoproduct::HelloRGB(p) = properties {
                        p
                    } else {
                        unreachable!()
                    };
                    HelloRGB::handle_click(properties, ctx, args);
                }],
                will_render_handlers: vec![],
                did_mount_handlers: vec![],
                scroll_handlers: vec![|stack_frame, ctx, args| {
                    let properties = (*stack_frame).borrow().get_properties();
                    let properties = &mut *properties.as_ref().borrow_mut();
                    let properties = if let PropertiesCoproduct::HelloRGB(p) = properties {
                        p
                    } else {
                        unreachable!()
                    };
                    HelloRGB::handle_scroll(properties, ctx, args);
                }],
            }))),
            instance_registry: Rc::clone(&instance_registry),
            transform: Rc::new(RefCell::new(PropertyExpression::new(20))),
            size: Some(Rc::new(RefCell::new([
                Box::new(PropertyLiteral::new(Size::Percent(50.into()))),
                Box::new(PropertyLiteral::new(Size::Percent(50.into()))),
            ]))),
            children: Some(Rc::new(RefCell::new(vec![]))),
            component_template: None,
            scroller_args: None,
            slot_index: None,
            repeat_source_expression_vec: None,
            repeat_source_expression_range: None,
            conditional_boolean_expression: None,
            compute_properties_fn: None,
        }),
    ])));

    args.handler_registry = Some(Rc::new(RefCell::new(HandlerRegistry {
        click_handlers: vec![],
        will_render_handlers: vec![],
        did_mount_handlers: vec![],
        scroll_handlers: vec![],
    })));

    args.compute_properties_fn = Some(Box::new(|properties, rtc| {
        let properties = &mut *properties.as_ref().borrow_mut();
        let properties = if let PropertiesCoproduct::HelloRGB(p) = properties {
            p
        } else {
            unreachable!()
        };

        if let Some(new_value) =
            rtc.compute_eased_value(properties.rotation._get_transition_manager())
        {
            properties.rotation.set(new_value);
        } else if let Some(new_value) =
            rtc.compute_vtable_value(properties.rotation._get_vtable_id())
        {
            let new_value = if let TypesCoproduct::f64(v) = new_value {
                v
            } else {
                unreachable!()
            };
            properties.rotation.set(new_value);
        }
    }));

    ComponentInstance::instantiate(args)
}
