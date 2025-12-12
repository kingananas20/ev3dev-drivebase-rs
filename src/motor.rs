//! Motor configuration and direction types for the drivebase.

use ev3dev_lang_rust::motors::MotorPort;

/// Required metadata for each motor of the drivebase.
///
/// This struct holds the physical configuration of a motor, including which port
/// it's connected to and which direction it needs to turn to make the robot move forward.
///
/// # Examples
///
/// ```
/// use ev3_drivebase::{Motor, Direction};
/// use ev3_drivebase::ev3dev_lang_rust::motors::MotorPort;
///
/// // Left motor with shaft pointing forward
/// let left = Motor::new(MotorPort::OutA, Direction::Clockwise);
///
/// // Right motor with shaft pointing backward
/// let right = Motor::new(MotorPort::OutB, Direction::CounterClockwise);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Motor {
    /// The port of the motor (`OutA`, `OutB`, `OutC`, or `OutD`).
    pub port: MotorPort,
    /// The direction the motor needs to turn to make the robot go forward.
    pub direction: Direction,
}

impl Motor {
    /// Creates a new `Motor` configuration.
    ///
    /// # Parameters
    ///
    /// - `port`: The EV3 output port this motor is connected to
    /// - `direction`: The direction this motor should turn to move the robot forward
    ///
    /// # Examples
    ///
    /// ```
    /// use ev3_drivebase::{Motor, Direction};
    /// use ev3_drivebase::ev3dev_lang_rust::motors::MotorPort;
    ///
    /// let motor = Motor::new(MotorPort::OutA, Direction::Clockwise);
    /// ```
    #[must_use]
    pub const fn new(port: MotorPort, direction: Direction) -> Self {
        Self { port, direction }
    }
}

#[expect(clippy::doc_markdown)]
/// The direction a motor needs to turn to make the robot move forward.
///
/// This depends on how the motor is physically mounted on your robot:
///
/// - **Clockwise**: Use this if the motor shaft points toward the front of the robot
/// - **CounterClockwise**: Use this if the motor shaft points toward the back of the robot
///
/// # Examples
///
/// For a typical two-wheeled robot with motors on opposite sides:
///
/// ```
/// use ev3_drivebase::{Motor, Direction};
/// use ev3_drivebase::ev3dev_lang_rust::motors::MotorPort;
///
/// // Left motor (shaft points forward)
/// let left = Motor::new(MotorPort::OutA, Direction::Clockwise);
///
/// // Right motor (shaft points backward due to mirrored mounting)
/// let right = Motor::new(MotorPort::OutB, Direction::CounterClockwise);
/// ```
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(i8)]
pub enum Direction {
    /// Motor needs to turn clockwise to move the robot forward.
    Clockwise = 1,
    /// Motor needs to turn counter-clockwise to move the robot forward.
    CounterClockwise = -1,
}

impl Direction {
    /// Returns the sign multiplier for this direction.
    ///
    /// - `Clockwise` returns `1`
    /// - `CounterClockwise` returns `-1`
    ///
    /// This is used internally for calculating motor encoder counts.
    ///
    /// # Examples
    ///
    /// ```
    /// use ev3_drivebase::Direction;
    ///
    /// assert_eq!(Direction::Clockwise.sign(), 1);
    /// assert_eq!(Direction::CounterClockwise.sign(), -1);
    /// ```
    #[inline]
    #[must_use]
    pub const fn sign(self) -> i32 {
        self as i8 as i32
    }
}
