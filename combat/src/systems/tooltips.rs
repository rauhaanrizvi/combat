//START: boilerplate
use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(Health)]
pub fn tooltips(
    ecs: &SubWorld,
    #[resource] mouse_pos: &Point,// <callout id="co.hsmh.tt_point" />
    #[resource] camera: &Camera// <callout id="co.hsmh.tt_camera" />
) {
    let mut positions = <(Entity, &Point, &Name)>::query();// <callout id="co.hsmh.tt_query_ptname" />
    //END: boilerplate
    //START: tooltips
    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = *mouse_pos + offset;// <callout id="co.hsmh.tt_offset" />
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    positions
        .iter(ecs)// <callout id="co.hsmh.tt_iter_ent" />
        .filter(|(_, pos, _)| **pos == map_pos )// <callout id="co.hsmh.tt_iter_filter" />
        .for_each(|(entity, _, name) | {
            let screen_pos = *mouse_pos * 4;// <callout id="co.hsmh.tt_multiply" />
            let display = if let Ok(health) = ecs.entry_ref(*entity)// <callout id="co.hsmh.tt_iflet" />
                .unwrap()
                .get_component::<Health>() 
            {
                format!("{} : {} hp", &name.0, health.current)// <callout id="co.hsmh.tt_format" />
            } else {// <callout id="co.hsmh.tt_else" />
                name.0.clone()
            };
            draw_batch.print(screen_pos, &display);
        });
    draw_batch.submit(10100).expect("Batch error");
    //END: tooltips
}
