//! Provides the Reset Chance Component and relevant types for using it.
//! The Reset Chance Component is a component that the probability of successfully completing
//! the current split. If there is no active attempt it shows the general chance
//! of completing a run. During an attempt it actively changes based on the current split.

use super::key_value;
use crate::{
    TimerPhase,
    analysis::reset_chance::{self, SuccessCounts},
    platform::prelude::*,
    settings::{Color, Field, Gradient, SettingsDescription, Value},
    timing::Snapshot,
};
use core::fmt::Write;
use serde_derive::{Deserialize, Serialize};

/// The Reset Chance Component is a component that the probability of successfully completing
/// the current split. If there is no active attempt it shows the general chance
/// of completing a run. During an attempt it actively changes based on the current split.
#[derive(Default, Clone)]
pub struct Component {
    settings: Settings,
    timer_phase: Option<TimerPhase>,
    split_index: Option<usize>,
    success_counts: Option<SuccessCounts>,
}

/// The Settings for this component.
#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    /// The background shown behind the component.
    pub background: Gradient,
    /// Specifies whether to display the name of the component and its value in
    /// two separate rows.
    pub display_two_rows: bool,
    /// The color of the label. If `None` is specified, the color is taken from
    /// the layout.
    pub label_color: Option<Color>,
    /// The color of the value. If `None` is specified, the color is taken from
    /// the layout.
    pub value_color: Option<Color>,
    /// Instead of showing a the reset chance, show the success chance (which is the
    /// 100% minus the reset chance).
    pub show_successes: bool,
    /// In addition to the reset or success chance, show the attempt counts which are
    /// used for the calcuation.
    pub show_attempt_details: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            background: key_value::DEFAULT_GRADIENT,
            display_two_rows: false,
            label_color: None,
            value_color: None,
            show_successes: false,
            show_attempt_details: false,
        }
    }
}

impl Component {
    /// Creates a new Reset Chance Component.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new Reset Chance Component with the given settings.
    pub const fn with_settings(settings: Settings) -> Self {
        Self {
            settings,
            timer_phase: None,
            split_index: None,
            success_counts: None,
        }
    }

    /// Accesses the settings of the component.
    pub const fn settings(&self) -> &Settings {
        &self.settings
    }

    /// Grants mutable access to the settings of the component.
    pub const fn settings_mut(&mut self) -> &mut Settings {
        &mut self.settings
    }

    /// Accesses the name of the component.
    pub const fn name(&self) -> &'static str {
        "Reset Chance"
    }

    /// Updates the component's state based on the timer provided.
    pub fn update_state(&mut self, state: &mut key_value::State, timer: &Snapshot) {
        state.background = self.settings.background;
        state.key_color = self.settings.label_color;
        state.value_color = self.settings.value_color;
        state.semantic_color = Default::default();

        state.key.clear();
        state.key.push_str(if self.settings.show_successes {
            "Success Chance"
        } else {
            "Reset Chance"
        });

        if Some(timer.current_phase()) != self.timer_phase
            || timer.current_phase() == TimerPhase::NotRunning
        {
            self.timer_phase = Some(timer.current_phase());
            self.success_counts = None;
        }
        if timer.current_split_index() != self.split_index {
            self.split_index = timer.current_split_index();
            self.success_counts = None;
        }
        if self.success_counts.is_none() {
            self.success_counts = Some(reset_chance::calculate(timer));
        }
        let mut counts = self.success_counts.clone().unwrap_or_default();
        if !self.settings.show_successes {
            counts.successful_attempts = counts.total_attempts - counts.successful_attempts
        }
        let chance = if counts.total_attempts == 0 {
            if self.settings.show_successes {
                1.0
            } else {
                0.0
            }
        } else {
            counts.successful_attempts as f64 / counts.total_attempts as f64
        };

        state.value.clear();
        if self.settings.show_attempt_details {
            let _ = write!(
                state.value,
                "{}/{} ({:.1}%)",
                counts.successful_attempts,
                counts.total_attempts,
                100.0 * chance
            );
        } else {
            let _ = write!(state.value, "{:.1}%", 100.0 * chance);
        }

        state.key_abbreviations.clear();
        state.display_two_rows = self.settings.display_two_rows;
        state.updates_frequently = false;
    }

    /// Calculates the component's state based on the timer provided.
    pub fn state(&mut self, timer: &Snapshot) -> key_value::State {
        let mut state = Default::default();
        self.update_state(&mut state, timer);
        state
    }

    /// Accesses a generic description of the settings available for this
    /// component and their current values.
    pub fn settings_description(&self) -> SettingsDescription {
        SettingsDescription::with_fields(vec![
            Field::new(
                "Background".into(),
                "The background shown behind the component.".into(),
                self.settings.background.into(),
            ),
            Field::new(
                "Display 2 Rows".into(),
                "Specifies whether to display the name of the component and the reset chance in two separate rows."
                    .into(),
                self.settings.display_two_rows.into(),
            ),
            Field::new(
                "Label Color".into(),
                "The color of the component's name. If not specified, the color is taken from the layout."
                    .into(),
                self.settings.label_color.into(),
            ),
            Field::new(
                "Value Color".into(),
                "The color of the PB chance. If not specified, the color is taken from the layout."
                    .into(),
                self.settings.value_color.into(),
            ),
            Field::new(
                "Show Successes".into(),
                "Instead of showing the reset chance, show the success chance for the current split.".into(),
                self.settings.show_successes.into(),
            ),
            Field::new(
                "Show Attempt Details".into(),
                "In addition to showing the reset chance, show the attempt counts used for the calculation.".into(),
                self.settings.show_attempt_details.into(),
            )
        ])
    }

    /// Sets a setting's value by its index to the given value.
    ///
    /// # Panics
    ///
    /// This panics if the type of the value to be set is not compatible with
    /// the type of the setting's value. A panic can also occur if the index of
    /// the setting provided is out of bounds.
    pub fn set_value(&mut self, index: usize, value: Value) {
        match index {
            0 => self.settings.background = value.into(),
            1 => self.settings.display_two_rows = value.into(),
            2 => self.settings.label_color = value.into(),
            3 => self.settings.value_color = value.into(),
            4 => self.settings.show_successes = value.into(),
            5 => self.settings.show_attempt_details = value.into(),
            _ => panic!("Unsupported Setting Index"),
        }
    }
}
