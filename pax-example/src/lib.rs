
pub mod hello_rgb;
pub mod fireworks;
pub mod grids;

use pax::*;
use pax::api::{ArgsClick, Property};
use pax_std::primitives::{Group};

use crate::grids::Grids;
use crate::hello_rgb::HelloRGB;
use crate::fireworks::Fireworks;

#[pax_app(
    <Group @click=modulate >
        if current_route == 0 {
            <Grids />
        }
        if current_route == 1 {
            <Fireworks />
        }
        if current_route == 2 {
            <HelloRGB />
        }
    </Group>
)]
pub struct Example {
    pub current_route: Property<usize>,
}


impl Example {
    pub fn modulate(&mut self, args: ArgsClick) {
        const ROUTE_COUNT : usize = 3;

        let old_route = self.current_route.get();
        self.current_route.set((old_route + 1) % ROUTE_COUNT);
    }
}
