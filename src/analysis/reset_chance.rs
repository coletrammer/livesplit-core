//! Calculates the number of successful and total number of attempts for a given split.
//! If there's no active attempt, the counts are relative to the entire run instead.

use crate::{Run, TimerPhase, timing::Snapshot};

/// The split success counts calculated by the reset chance analysis.
#[derive(Default, Clone)]
pub struct SuccessCounts {
    /// The total number of attempts which completed the relevant split or run.
    pub successful_attempts: u32,

    /// The total number of attempts which reached this split (or the total number of attempts for
    /// the entire run).
    pub total_attempts: u32,
}

/// Calculates the total number of attempts which were completed for the given run.
pub fn total_successful_attempts(run: &Run) -> u32 {
    run.attempt_history()
        .iter()
        .filter(|a| a.time().real_time.is_some())
        .count() as u32
}

/// Caulcates the success counts for a given timer snapshot. For active runs this returns
/// the counts for the current split rather than the entire run.
pub fn calculate(timer: &Snapshot) -> SuccessCounts {
    let phase = timer.current_phase();
    let run = timer.run();

    match phase {
        TimerPhase::Running | TimerPhase::Paused => {
            let current_index = timer.current_split_index().unwrap_or_default();
            let total_attempts = if current_index == 0 {
                run.attempt_count()
            } else {
                run.segments()[current_index - 1]
                    .segment_history()
                    .iter_actual_runs()
                    .count() as u32
            };
            let successful_attempts = run.segments()[current_index]
                .segment_history()
                .iter_actual_runs()
                .count() as u32;

            SuccessCounts {
                successful_attempts,
                total_attempts,
            }
        }
        TimerPhase::Ended => {
            let count = 1 + total_successful_attempts(run);
            SuccessCounts {
                successful_attempts: count,
                total_attempts: count,
            }
        }
        TimerPhase::NotRunning => SuccessCounts {
            successful_attempts: total_successful_attempts(run),
            total_attempts: run.attempt_count(),
        },
    }
}
