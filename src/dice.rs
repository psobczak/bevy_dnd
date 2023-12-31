use bevy::prelude::*;
use bevy_turborand::prelude::*;

pub struct DicePlugin;

impl Plugin for DicePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RngPlugin::default())
            .add_event::<RollDiceEvent>()
            .add_event::<RollResultsEvent>()
            .add_systems(Update, (roll_dice.run_if(on_event::<RollDiceEvent>()),));
    }
}

#[derive(Event)]
pub enum RollDiceEvent {
    Initiative(Entity),
}

struct Roll {
    sides: u32,
    times: u32,
}

impl From<&RollDiceEvent> for Roll {
    fn from(value: &RollDiceEvent) -> Self {
        match value {
            RollDiceEvent::Initiative(_) => Roll {
                sides: 20,
                times: 1,
            },
        }
    }
}

impl Roll {
    fn throws(&self, global_rng: &mut ResMut<GlobalRng>) -> Vec<u32> {
        (0..self.times)
            .map(|_| global_rng.u32(1..=self.sides))
            .collect::<Vec<_>>()
    }
}

#[derive(Event, Debug)]
pub struct RollResultsEvent(pub Entity, pub Vec<u32>);

fn roll_dice(
    mut global_rng: ResMut<GlobalRng>,
    mut reader: EventReader<RollDiceEvent>,
    mut writer: EventWriter<RollResultsEvent>,
) {
    for event in reader.iter() {
        let (entity, result) = match event {
            RollDiceEvent::Initiative(entity) => {
                let throws = Roll::from(event).throws(&mut global_rng);
                info!("Entity {:?} rolled for INITIATIVE: {:?}", &entity, &throws);
                (entity, throws)
            }
        };

        writer.send(RollResultsEvent(*entity, result));
    }
}
