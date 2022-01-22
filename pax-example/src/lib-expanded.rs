#[macro_use]
extern crate lazy_static;


use pax::*;

pub struct DeeperStruct {
    a: i64,
    b: &'static str,
}

//#[pax] was here
pub struct Root {
    //rewrite to pub `num_clicks : Property<i64>` etc. AND register metadata with dev server
    pub num_clicks : i64,
    pub current_rotation: f64,
    pub deeper_struct: DeeperStruct,
}


#[cfg(feature="parser")]
use pax::message::ComponentDefinition;
#[cfg(feature="parser")]
use pax::parser;
#[cfg(feature="parser")]
use std::collections::HashSet;
#[cfg(feature="parser")]
use std::{env, fs};
#[cfg(feature="parser")]
use std::path::{Path, PathBuf};
#[cfg(feature="parser")]
use pax::parser::ManifestContext;
#[cfg(feature="parser")]
lazy_static! {
    static ref source_id : String = parser::get_uuid();
}
#[cfg(feature="parser")]
lazy_static! {
    static ref this : String = String::from("Root");
}
//generated if lib.rs
#[cfg(feature="parser")]
pub fn main() {
    let mut ctx = ManifestContext{
        visited_source_ids: HashSet::new(),
        component_definitions: vec![],
    };
    ctx = Root::parse_to_manifest(ctx);
}
#[cfg(feature="parser")]
impl Root {
    pub fn parse_to_manifest(mut ctx: ManifestContext) -> ManifestContext {

        match ctx.visited_source_ids.get(&source_id as &str) {
            None => {
                //First time visiting this file/source — parse the relevant contents
                //then recurse through child nodes, unrolled here in the macro as
                //parsed from the template
                ctx.visited_source_ids.insert(source_id.clone());

                //GENERATE: gen explict_path value with macro
                let explicit_path : Option<String> = Some("lib.pax".to_string());
                //TODO: support inline pax as an alternative to file
                //GENERATE: inject pascal_identifier
                let PASCAL_IDENTIFIER = "Root";
                let component_definition_for_this_file = parser::handle_file(file!(), explicit_path, PASCAL_IDENTIFIER);
                ctx.component_definitions.push(component_definition_for_this_file);


                // //Note:  the file!() here will be substitutable for the #[pax file={}] attribute, fixing
                // //       the apparent problem of using file!() at this phase of the macro lifecycle
                // println!("macro needs to unroll: {:?}", compiletime::process_pax_file_for_pascal_identifiers(file!()));
                //
                // //these macros are generated by parsing template for unique
                // //component names, which are assumed to be
                // //in this scope (e.g. via `use`)
                // //TODO: reasonable userland error message for missing imports
                // //******** dynamic macro logic here

                //GENERATE:
                // // ctx = Spread::parse_to_manifest(ctx);
                // ctx = Rectangle::parse_to_manifest(ctx);
                // ctx = Group::parse_to_manifest(ctx);
                // ctx = Text::parse_to_manifest(ctx);
                // //******** end dynamic macro logic

                ctx
            },
            _ => {ctx} //early return; this file has already been parsed
        }


        /*
        <Spread id="main-spread">
            <Rectangle id="rect-1" />
            <Rectangle id="rect-2" />
            <Group>
                <Text id="label" content="Hello!" />
                <Rectangle id="rect-3" />
            </Group>
        </Spread>
         */
        //code-gen manifest recursion


        //note: duplicates are managed by
        //      the file_id hack — can keep a registry in
        //      o



        // file!
        // module!
        // find children; recurse get_manifest()
        // in future: get schema of methods, types of properties
    }
}


impl Root {

    pub fn new() -> Self {
        Self {
            //Default values.  Could shorthand this into a macro via PAXEL
            num_clicks: 0,
            current_rotation: 0.0,
            deeper_struct: DeeperStruct {
                a: 100,
                b: "Profundo!",
            }
        }
    }

    //On click, increment num_clicks and update the rotation

    //Note the userland ergonomics here, using .get() and .set()
    //vs. the constructor and struct definition of bare types (e.g. i64, which doesn't have a .get() or .set() method)
    //Approaches:
    // - rewrite the struct at macro time; also rewrite the constructor
    // - inject something other than self into increment_clicker, including a .gettable and .settable wrapper
    //   around (note that this injected struct, if it's going to have the pattern struct.num_clicks.set, will
    //   still require some codegen; can't be achieved with generics alone


    // pub fn increment_clicker(&mut self, args: ClickArgs) {
    //     self.num_clicks.set(self.num_clicks + 1);
    //     self.current_rotation.setTween( //also: setTweenLater, to enqueue a tween after the current (if any) is done
    //         self.num_clicks.get() * 3.14159 / 4,
    //         Tween {duration: 1000, curve: Tween::Ease}
    //     );
    // }

}


/* Approaches for dirty-handling of properties:
    - Check dataframes on each tick (brute-force)
    - inject a setter, ideally with primitive ergonomics (`self.x = self.x + 1`)
        probably done with a macro decorating the struct field
        - setter(a): generate a `set_field_name<T>(new: T)` method for each decorated `field_name: T`
       ***setter(b):   `num_clicks: T` becomes `self.num_clicks.get() //-> T` and `self.num_clicks.set(new: T)`
                       in the expression language, `num_clicks` automatically unwraps `get()`
                       `.get()` feels fine for Rust ergonomics, in line with `unwrap()`
                       `.set(new: T)` is also not the worst, even if it could be better.
                       In TS we can have better ergonomics with `properties`
 */




//DONE: is all descendent property access via Actions + selectors? `$('#some-desc').some_property`
//      or do we need a way to support declaring desc. properties?
//      We do NOT need a way to declar desc. properties here — because they are declared in the
//      `properties` blocks of .dash