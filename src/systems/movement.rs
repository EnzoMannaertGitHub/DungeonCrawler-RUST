use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
#[read_component(FieldOfView)]
#[read_component(Health)]
#[write_component(Health)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &mut Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(want_move.destination) {
        commands.add_component(want_move.entity, want_move.destination);

        if let Ok(entry) = ecs.entry_ref(want_move.entity) {
            if let Ok(fov) = entry.get_component::<FieldOfView>() {
                commands.add_component(want_move.entity, fov.clone_dirty());

                if entry.get_component::<Player>().is_ok() {
                    camera.on_player_move(want_move.destination, map);
                    fov.visible_tiles.iter().for_each(|pos| {
                        map.revealed_tiles[map_idx(pos.x, pos.y)] = true;
                    })
                }
            }
        }
    } else {
        if map.tiles[map_idx(want_move.destination.x, want_move.destination.y)] == TileType::Trap {
            if let Ok(mut entry) = ecs.entry_mut(want_move.entity) {
                if entry.get_component::<Player>().is_ok() {
                    let mut health = entry.get_component_mut::<Health>().unwrap();
                    health.current -= 1;
                }
            }
        }
    }
    commands.remove(*entity); //remove messages once they are posted
}
