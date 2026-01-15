//! The Reset Chance Component is a component that the probability of successfully completing
//! the current split. If there is no active attempt it shows the general chance
//! of completing a run. During an attempt it actively changes based on the current split.

use super::{Json, output_vec};
use crate::component::OwnedComponent;
use crate::key_value_component_state::OwnedKeyValueComponentState;
use livesplit_core::Timer;
use livesplit_core::component::reset_chance::Component as ResetChanceComponent;

/// type
pub type OwnedResetChanceComponent = Box<ResetChanceComponent>;

/// Creates a new PB Chance Component.
#[unsafe(no_mangle)]
pub extern "C" fn ResetChanceComponent_new() -> OwnedResetChanceComponent {
    Box::new(ResetChanceComponent::new())
}

/// drop
#[unsafe(no_mangle)]
pub extern "C" fn ResetChanceComponent_drop(this: OwnedResetChanceComponent) {
    drop(this);
}

/// Converts the component into a generic component suitable for using with a
/// layout.
#[unsafe(no_mangle)]
pub extern "C" fn ResetChanceComponent_into_generic(
    this: OwnedResetChanceComponent,
) -> OwnedComponent {
    Box::new((*this).into())
}

/// Encodes the component's state information as JSON.
#[unsafe(no_mangle)]
pub extern "C" fn ResetChanceComponent_state_as_json(
    this: &mut ResetChanceComponent,
    timer: &Timer,
) -> Json {
    output_vec(|o| {
        this.state(&timer.snapshot()).write_json(o).unwrap();
    })
}

/// Calculates the component's state based on the timer provided.
#[unsafe(no_mangle)]
pub extern "C" fn ResetChanceComponent_state(
    this: &mut ResetChanceComponent,
    timer: &Timer,
) -> OwnedKeyValueComponentState {
    Box::new(this.state(&timer.snapshot()))
}
