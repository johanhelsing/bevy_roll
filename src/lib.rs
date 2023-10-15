#[cfg(feature = "bevy_ggrs")]
use bevy::reflect::{GetTypeRegistration, TypePath};
use bevy::{
    ecs::schedule::{run_enter_schedule, ScheduleLabel},
    prelude::*,
};
#[cfg(feature = "bevy_ggrs")]
use bevy_ggrs::GgrsApp;

mod frame_count;

// re-exports
pub use frame_count::{increase_frame_count, RollFrameCount};

pub mod prelude {
    pub use super::RollApp;
}

pub trait RollApp {
    /// Add state transitions to the given schedule
    fn add_roll_state<S: States>(&mut self, schedule: impl ScheduleLabel) -> &mut Self;

    #[cfg(feature = "bevy_ggrs")]
    /// Register this state to be rolled back by bevy_ggrs
    fn register_ggrs_state<S: States + GetTypeRegistration + FromReflect + TypePath>(
        &mut self,
    ) -> &mut Self;
}

impl RollApp for App {
    fn add_roll_state<S: States>(&mut self, schedule: impl ScheduleLabel) -> &mut Self {
        self.init_resource::<NextState<S>>()
            .init_resource::<State<S>>()
            .init_resource::<InitialStateEntered<S>>()
            .add_systems(
                schedule,
                (
                    run_enter_schedule::<S>.run_if(resource_equals(InitialStateEntered::<S>(None))),
                    mark_state_initialized::<S>
                        .run_if(resource_equals(InitialStateEntered::<S>(None))),
                    apply_state_transition::<S>,
                )
                    .chain(),
            )
    }

    #[cfg(feature = "bevy_ggrs")]
    fn register_ggrs_state<S: States + GetTypeRegistration + FromReflect + TypePath>(
        &mut self,
    ) -> &mut Self {
        self.register_rollback_resource::<State<S>>()
            .register_rollback_resource::<NextState<S>>()
            .register_rollback_resource::<InitialStateEntered<S>>()
    }
}

#[derive(Resource, Debug, Reflect, Default, Eq, PartialEq)]
#[reflect(Resource)]
pub struct InitialStateEntered<S: States>(Option<S>); // todo: PhantomData instead?

fn mark_state_initialized<S: States>(mut state_initialized: ResMut<InitialStateEntered<S>>) {
    state_initialized.0 = Some(default());
}
