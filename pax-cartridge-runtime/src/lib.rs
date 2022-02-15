use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use pax_core::{ComponentInstance, PropertyExpression, RenderNodePtrList, RenderTreeContext, ExpressionContext};
use pax_core::pax_properties_coproduct::{PropertiesCoproduct, TypesCoproduct};

use pax_runtime_api::{Property, PropertyLiteral, Transform};

//generate dependencies, pointing to userland cartridge (same logic as in PropertiesCoproduct)
use pax_example::pax_types::{RootProperties};
use pax_example::pax_types::pax_std::primitives::{GroupProperties, RectangleProperties};
use pax_example::pax_types::pax_std::types::{Color, StrokeProperties, Size};

//dependency paths below come from pax_primitive macro, where these crate+module paths are passed as parameters:
use pax_std_primitives::{RectangleInstance, GroupInstance };

pub fn instantiate_expression_table() -> HashMap<String, Box<dyn Fn(ExpressionContext) -> TypesCoproduct>> {
    let mut map : HashMap<String, Box<dyn Fn(ExpressionContext) -> TypesCoproduct>> = HashMap::new();

    map.insert("a".to_string(), Box::new(|ec: ExpressionContext| -> TypesCoproduct {
        //note that type coercion should happen here, too:
        //(must know symbol name as well as source & destination types)
        //(compiler can keep a dict of operand types)

        //for @frames_elapsed
        #[allow(non_snake_case)]
        let __AT__frames_elapsed = ec.engine.frames_elapsed as f64;

        TypesCoproduct::Transform(
            Transform::origin(Size::Percent(50.0), Size::Percent(50.0)) *
            Transform::scale(2.3, 2.3) *
            Transform::rotate( __AT__frames_elapsed / 270.0) *
            Transform::origin(Size::Percent(f64::sin(__AT__frames_elapsed / 1000.0) * 100.0), Size::Percent(f64::cos(__AT__frames_elapsed / 1000.0) * 100.0)) *
            Transform::rotate((f64::cos(__AT__frames_elapsed / 100.0) * 0.0010 + 1.0) * (__AT__frames_elapsed) / 100.0) *
            Transform::align(0.5, 0.5)
        )
    }));

    map.insert("b".to_string(), Box::new(|ec: ExpressionContext| -> TypesCoproduct {
        #[allow(non_snake_case)]
        let __AT__frames_elapsed = ec.engine.frames_elapsed as f64;

        TypesCoproduct::Transform(
            Transform::origin(Size::Percent(50.0), Size::Percent(50.0)) *
            Transform::scale(1.8, 1.8) *
            Transform::rotate(__AT__frames_elapsed / 100.0) *
            Transform::origin(Size::Percent(f64::sin(__AT__frames_elapsed / 1000.0) * 150.0), Size::Percent(f64::cos(__AT__frames_elapsed / 1000.0) * 150.0)) *
            Transform::rotate((f64::cos(__AT__frames_elapsed / 100.0) * 0.0010 + 1.0) * (__AT__frames_elapsed) / 100.0) *
            Transform::align(0.5, 0.5)
        )
    }));

    map
}

pub fn instantiate_root_component() -> Rc<RefCell<ComponentInstance>> {
    RootInstance::instantiate(
        PropertiesCoproduct::Root(RootProperties {
            num_clicks: Box::new(PropertyLiteral {value: 0} ),
            current_rotation: Box::new(PropertyLiteral {value: 0.0}),
            deeper_struct: Box::new(PropertyLiteral {value: Default::default()})
        }),
        Rc::new(RefCell::new(PropertyLiteral{value: Transform::default()})),
        Rc::new(RefCell::new(vec![
            GroupInstance::instantiate(
                PropertiesCoproduct::Group(GroupProperties {}),
                Rc::new(RefCell::new(PropertyLiteral {value: Transform::default()})),
                Rc::new(RefCell::new(vec![
                    RectangleInstance::instantiate(
                        PropertiesCoproduct::Rectangle(
                            RectangleProperties {
                                stroke: Box::new(PropertyLiteral { value: StrokeProperties {
                                    color: Box::new(PropertyLiteral {value: Color::rgba(1.0, 1.0, 0.0, 1.0)}),
                                    width: Box::new(PropertyLiteral{value: 7.0}),
                                }}),
                                fill: PropertyLiteral::new(Color::rgba(0.0, 1.0, 0.0, 0.25)),
                            }
                        ),
                        Rc::new(RefCell::new(
                            PropertyExpression { id: "a".to_string(), cached_value: Default::default()})),
                        [PropertyLiteral::new(Size::Pixel(300.0)), PropertyLiteral::new(Size::Pixel(300.0))]
                    ),
                    RectangleInstance::instantiate(
                        PropertiesCoproduct::Rectangle(
                            RectangleProperties {
                                stroke: Box::new(PropertyLiteral { value: StrokeProperties {
                                    color: Box::new(PropertyLiteral {value: Color::rgba(0.0, 1.0, 1.0, 1.0)}),
                                    width: Box::new(PropertyLiteral{value: 7.0}),
                                }}),
                                fill: PropertyLiteral::new(Color::rgba(1.0, 0.0, 1.0, 1.0)),
                            }
                        ),
                        Rc::new(RefCell::new(
                            PropertyExpression { id: "b".to_string(), cached_value: Default::default()})),
                        [PropertyLiteral::new(Size::Pixel(300.0)), PropertyLiteral::new(Size::Pixel(300.0))]
                    ),
                ])),
            ),
            // Rc::new(RefCell::new()),
            // Rc::new(RefCell::new()),
        ]))
    )
}

pub struct RootInstance {}
impl RootInstance {
    pub fn instantiate(properties: PropertiesCoproduct, transform: Rc<RefCell<dyn Property<Transform>>>, children: RenderNodePtrList /*, adoptees*/) -> Rc<RefCell<ComponentInstance>> {
        Rc::new(RefCell::new(ComponentInstance {
            template: children,
            adoptees: Rc::new(RefCell::new(vec![])),
            transform,
            properties: Rc::new(RefCell::new(properties)),
            compute_properties_fn: Box::new(|properties: Rc<RefCell<PropertiesCoproduct>>, rtc: &mut RenderTreeContext|{

                let properties_unwrapped = &mut *properties.deref().borrow_mut();
                if let PropertiesCoproduct::Root(properties_cast) =  properties_unwrapped {
                    //Note: this is code-genned based on parsed knowledge of the properties
                    //      of `Root`
                    //TODO: unwrap into register_id and expression/timeline lookup mechanism
                    // properties_cast.deeper_struct.compute_in_place(rtc);
                    // properties_cast.current_rotation.compute_in_place(rtc);
                    // properties_cast.num_clicks.compute_in_place(rtc);
                } else {unreachable!()}
            }),
            timeline: None
        }))
    }

}

//Root => get_instance()


//Rectangle => get_instance()
//Group => get_instance()