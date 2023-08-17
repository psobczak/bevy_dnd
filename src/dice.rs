use bevy::prelude::*;
use bevy_turborand::prelude::*;

pub struct DicePlugin;

impl Plugin for DicePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RngPlugin::default())
            .add_event::<RollDiceEvent>()
            .add_event::<RollResultsEvent>()
            .add_systems(
                Update,
                (
                    roll_dice.run_if(on_event::<RollDiceEvent>()),
                    debug_roll.run_if(on_event::<RollResultsEvent>()),
                ),
            );
    }
}

#[derive(Event)]
pub struct RollDiceEvent {
    pub who_rolled: Entity,
    pub sides: u32,
    pub times: u32,
}

#[derive(Event)]
pub struct RollResultsEvent(pub Entity, pub Vec<u32>);

fn roll_dice(
    mut global_rng: ResMut<GlobalRng>,
    mut reader: EventReader<RollDiceEvent>,
    mut writer: EventWriter<RollResultsEvent>,
) {
    for event in reader.iter() {
        let throws = (0..event.times)
            .map(|_| global_rng.u32(1..=event.sides))
            .collect::<Vec<_>>();

        writer.send(RollResultsEvent(event.who_rolled, throws))
    }
}

fn debug_roll(mut reader: EventReader<RollResultsEvent>) {
    for event in reader.iter() {
        info!("Entity {:?} threw: {:?}", event.0, event.1)
    }
}
