use chrono::Duration;
use flo_curves::{bezier::Curve, BezierCurve};
use num_traits::Float;

use crate::vector::VectorND;

/// A state-space animation helper class.
///
/// # Example
/// ```rust
/// # use state_space_tween::anim::MultiVariableAnimation;
/// # use chrono::Duration;
/// let animation = MultiVariableAnimation::new(
///     [1.0, 2.0, 3.0],
///     [4.0, 5.0, 6.0],
///     [7.0, 8.0, 9.0],
///     [10.0, 11.0, 12.0],
///     Duration::seconds(2)
/// );
/// 
/// assert!(animation.sample(Duration::seconds(0)) == [1.0, 2.0, 3.0]);
/// assert!(animation.sample(Duration::seconds(2)) == [10.0, 11.0, 12.0]);
/// ```
#[derive(Debug, Clone)]
pub struct MultiVariableAnimation<Prim, const STATES: usize>
where
    Prim: Float + Into<f64>,
{
    curve: Curve<VectorND<Prim, STATES>>,
    duration: Duration,
}

impl<Prim, const STATES: usize> MultiVariableAnimation<Prim, STATES>
where
    Prim: Float + Into<f64>,
{
    /// Construct a new animation
    pub fn new(
        start: [Prim; STATES],
        control_1: [Prim; STATES],
        control_2: [Prim; STATES],
        end: [Prim; STATES],
        duration: Duration,
    ) -> Self {
        Self {
            curve: Curve {
                start_point: VectorND(start),
                end_point: VectorND(end),
                control_points: (VectorND(control_1), VectorND(control_2)),
            },
            duration,
        }
    }

    /// Calculate the bezier `t` value for a given time
    pub fn t_for_time(&self, time: Duration) -> f64 {
        time.num_milliseconds() as f64 / self.duration.num_milliseconds() as f64
    }

    /// Calculate the time that best corresponds to a given bezier `t` value
    pub fn time_for_t(&self, t: f64) -> Duration {
        Duration::milliseconds((t * self.duration.num_milliseconds() as f64) as i64)
    }

    /// Sample the system states at a given time
    pub fn sample(&self, time: Duration) -> [Prim; STATES] {
        self.curve.point_at_pos(self.t_for_time(time)).into()
    }

    /// Sample the system states at a given bezier `t` value
    pub fn sample_at_t(&self, t: f64) -> [Prim; STATES] {
        self.curve.point_at_pos(t).into()
    }

    /// Find the time that best corresponds to a given output state
    pub fn time_for_output(&self, output: [Prim; STATES]) -> Option<Duration> {
        self.curve
            .t_for_point(&VectorND(output))
            .map(|t| self.time_for_t(t))
    }

    /// Get the start point
    pub fn start(&self) -> [Prim; STATES] {
        self.curve.start_point.into()
    }

    /// Get the end point
    pub fn end(&self) -> [Prim; STATES] {
        self.curve.end_point.into()
    }

    /// Get the control points
    pub fn control_points(&self) -> ([Prim; STATES], [Prim; STATES]) {
        (
            self.curve.control_points.0.into(),
            self.curve.control_points.1.into(),
        )
    }
}
