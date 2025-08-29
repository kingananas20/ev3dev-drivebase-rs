use ev3dev_lang_rust::motors::MotorPort;

/// Required metadata for each motor of the drivebase
#[derive(Debug, Clone, Copy)]
pub struct Motor {
    /// The port of the motor
    pub port: MotorPort,
    /// The direction the motor needs to turn to make the robot go forward.
    pub direction: Direction,
}

impl Motor {
    /// Creates a new `Motor` struct
    #[must_use]
    pub const fn new(port: MotorPort, direction: Direction) -> Self {
        Self { port, direction }
    }
}

/// The direction to make the robot go forward
#[derive(Debug, Clone, Copy)]
pub enum Direction {
    /// Motor needs to turn clockwise to move forward
    Clockwise,
    /// Motor needs to turn counter-clockwise to move forward
    CounterClockwise,
}
