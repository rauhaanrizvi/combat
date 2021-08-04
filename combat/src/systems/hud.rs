use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
pub fn hud(ecs: &SubWorld) {
    let mut health_query = <&Health>::query().filter(component::<Player>());// <callout id="co.hsmh.query_player_health" />
    let player_health = health_query
        .iter(ecs)
        .nth(0) // <callout id="co.hsmh.nth" />
        .unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);// <callout id="co.hsmh.hud_target" />
    draw_batch.print_centered(1, 
        "Explore the Dungeon. Cursor keys to move.");// <callout id="co.hsmh.welcome_text" />
    draw_batch.bar_horizontal(// <callout id="co.hsmh.bar_h" />
        Point::zero(),// <callout id="co.hsmh.bar_h_loc" />
        SCREEN_WIDTH*2,// <callout id="co.hsmh.bar_h_w" />
        player_health.current,// <callout id="co.hsmh.bar_h_current" />
        player_health.max,// <callout id="co.hsmh.bar_h_max" />
        ColorPair::new(RED, BLACK)// <callout id="co.hsmh.bar_h_color" />
    );
    draw_batch.print_color_centered(
        0,
        format!(" Health: {} / {} ", 
            player_health.current, 
            player_health.max
        ),
        ColorPair::new(WHITE, RED)
    );
    draw_batch.submit(10000).expect("Batch error");
}
