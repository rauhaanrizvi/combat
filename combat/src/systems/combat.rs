//START: boilerplate
use crate::prelude::*;

//START: query
#[system]
#[read_component(WantsToAttack)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();
    //END: query
    //END: boilerplate

    //START: victims
    let victims : Vec<(Entity, Entity)> = attackers// <callout id="co.hsmh.victim_iter" />
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.victim) )// <callout id="co.hsmh.victim_map" />
        .collect();// <callout id="co.hsmh.victim_collect" />
    //END: victims

    //START: damage
    victims.iter().for_each(|(message, victim)| {
        if let Ok(mut health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            println!("Health before attack: {}", health.current);
            health.current -= 1;
            if health.current < 1 {
                commands.remove(*victim);
            }
            println!("Health after attack: {}", health.current);
        }
        commands.remove(*message);
    });
    //END: damage
}
