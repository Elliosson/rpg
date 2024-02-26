use crate::components::*;
use bevy::prelude::*;

pub fn attack_animation(
    mut commands: Commands,
    mut player: Query<
        (Entity, &mut IsAttacking, &mut DeltaAngle),
        (With<Player>, With<IsAttacking>),
    >,

    time: Res<Time>,
) {
    if let Ok((entity, mut is_attacking, mut delta_angle)) = player.get_single_mut() {
        is_attacking.start_time.tick(time.delta()).elapsed_secs();
        let delta_time = is_attacking.start_time.elapsed_secs();
        println!("delta time {}", delta_time);
        if delta_time < 0.1 {
            delta_angle.delta = -delta_time * 10.;
        } else if delta_time < 0.2 {
            delta_angle.delta = -(delta_time - 0.2) * 10.;
        } else {
            delta_angle.delta = 0.;
            commands.entity(entity).remove::<IsAttacking>();
        }
    }
}
