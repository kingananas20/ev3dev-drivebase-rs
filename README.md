# ev3-drivebase

A high-level DriveBase abstraction for LEGO Mindstorms EV3 robots running ev3dev, written in Rust.

This crate provides an easy-to-use interface for controlling two-wheeled robots, similar to the DriveBase classes found in the Python and MicroPython ev3dev libraries.

## Features

- **Simple Movement Control**: Drive forward/backward and turn with precise distance and angle control
- **Configurable Brake Modes**: Choose between Coast, Brake, and Hold modes for different stopping behaviors
- **Acceleration/Deceleration Control**: Set custom ramp rates for smooth motion
- **In-Place and Arc Turns**: Turn on the spot or follow circular paths with specified radii
- **Color Sensor Support**: Optional integration with color sensors for line-following applications
- **Type-Safe API**: Leverage Rust's type system for safer robot control
- **Strict Linting**: Enforces best practices with comprehensive Clippy rules

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ev3-drivebase = "0.1.0"
```

## Quick Start

```rust
use ev3_drivebase::{DriveBase, Motor, Direction, BrakeMode};
use ev3_drivebase::ev3dev_lang_rust::{Ev3Error, motors::MotorPort};

fn main() -> Result<(), Ev3Error> {
    // Define motor configuration
    let left = Motor::new(MotorPort::OutA, Direction::Clockwise);
    let right = Motor::new(MotorPort::OutB, Direction::CounterClockwise);
    
    // Create drivebase with wheel diameter (43.2mm) and axle track (185mm)
    let mut drivebase = DriveBase::new(left, right, 43.2, 185.0)?;
    
    // Configure behavior
    drivebase
        .set_brake_mode(BrakeMode::Hold)?
        .set_acceleration(4000)?
        .set_deceleration(4000)?;
    
    // Drive 500mm forward at 100 deg/s
    drivebase.drive(100, 500, true)?;
    
    // Turn 90 degrees counter-clockwise in place
    drivebase.turn(200, 90, None)?;
    
    // Drive backward 300mm
    drivebase.drive(100, -300, true)?;
    
    Ok(())
}
```

## Understanding Motor Directions

When setting up your drivebase, you need to specify which direction each motor should turn to make the robot move forward. This depends on how your motors are physically mounted:

- **`Direction::Clockwise`**: Use if the motor shaft points toward the **front** of the robot
- **`Direction::CounterClockwise`**: Use if the motor shaft points toward the **back** of the robot

For a typical two-wheeled robot with motors on opposite sides, one motor will be `Clockwise` and the other `CounterClockwise`.

## Physical Measurements

The drivebase requires two measurements:

1. **Wheel Diameter**: Measure the diameter of your wheels in millimeters
2. **Axle Track**: Measure the distance between the points where the left and right wheels touch the ground in millimeters

These values are critical for accurate distance and turning calculations.

## Units

All measurements and parameters use the following units:

- **Distances**: millimeters (mm)
- **Speeds**: degrees per second (deg/s) - motor rotation speed, not robot speed
- **Angles**: degrees
- **Acceleration/Deceleration**: degrees per second squared (deg/s²)

## Core Concepts

### Brake Modes

The brake mode determines how motors behave when stopped:

- **`BrakeMode::Coast`**: Motors freely coast to a stop with no active braking. Lowest power consumption but least precise.
- **`BrakeMode::Brake`**: Motors actively brake to stop quickly but don't hold position afterward.
- **`BrakeMode::Hold`**: Motors actively hold their position after stopping. Most precise but uses more power.

### Driving

The `drive()` method moves the robot forward or backward:

```rust
// Drive 500mm forward at 200 deg/s and stop
drivebase.drive(200, 500, true)?;

// Drive 300mm backward at 150 deg/s and stop
drivebase.drive(150, -300, true)?;

// Drive forward continuously (don't stop after distance)
drivebase.drive(250, 1000, false)?;
```

### Turning

The `turn()` method provides two types of turns:

**In-Place Turns** (robot rotates around its center):
```rust
// Turn 90 degrees counter-clockwise
drivebase.turn(200, 90, None)?;

// Turn 180 degrees clockwise
drivebase.turn(200, -180, None)?;
```

**Arc Turns** (robot follows a circular path):
```rust
// Turn 90 degrees with a 200mm radius
drivebase.turn(200, 90, Some(200.0))?;

// Sharp 45-degree turn with 50mm radius
drivebase.turn(150, 45, Some(50.0))?;
```

- Positive angles turn **counter-clockwise** (left)
- Negative angles turn **clockwise** (right)
- Radius is measured from the turn center to the outer wheel

### Acceleration Control

Set how quickly the robot speeds up and slows down:

```rust
// Smooth acceleration for delicate tasks
drivebase.set_acceleration(2000)?;
drivebase.set_deceleration(2000)?;

// Quick response for fast movements
drivebase.set_acceleration(5000)?;
drivebase.set_deceleration(5000)?;
```

**Recommendations:**
- **Low (500-1500)**: Very smooth, good for delicate operations
- **Medium (2000-4000)**: Balanced, good for general use
- **High (4000-8000)**: Fast response, may cause wheel slip

## Advanced Features

### Color Sensors

Add color sensors for line following:

```rust
use ev3_drivebase::ev3dev_lang_rust::sensors::{ColorSensor, SensorPort};

let mut drivebase = DriveBase::new(left, right, 43.2, 185.0)?;

let left_sensor = ColorSensor::get(SensorPort::In1)?;
let right_sensor = ColorSensor::get(SensorPort::In2)?;

drivebase.add_colorsensor(left_sensor, right_sensor);

// Access sensors through drivebase.left_sensor and drivebase.right_sensor
```

### Status Checks

Monitor the state of your robot:

```rust
// Check if motors are running
if drivebase.is_running()? {
    println!("Motors are active");
}

// Check if motors are accelerating
if drivebase.is_ramping()? {
    println!("Motors are ramping up");
}

// Check if motors are holding position
if drivebase.is_holding()? {
    println!("Motors are holding");
}

// Check for problems
if drivebase.is_stalled()? {
    println!("Warning: Motors stalled!");
    drivebase.stop()?;
}

if drivebase.is_overloaded()? {
    println!("Warning: Motors overloaded!");
    drivebase.stop()?;
}
```

### Manual Control

Stop the robot at any time:

```rust
drivebase.stop()?;  // Stops according to brake mode
```

Reset all motor parameters:

```rust
drivebase.reset()?;  // Resets and stops motors
```

## Building for EV3

To compile your program for the EV3, you'll need to cross-compile for the ARMv5TE architecture:

### Setup

1. Add the target:
```bash
rustup target add armv5te-unknown-linux-musleabi
```

2. Install a linker (if not already available):
```bash
# On Ubuntu/Debian
sudo apt-get install gcc-arm-linux-gnueabi

# Or use rust-lld (recommended, no external dependencies)
```

3. Configure Cargo (create `.cargo/config.toml` in your project):
```toml
[build]
target = "armv5te-unknown-linux-musleabi"

[target.armv5te-unknown-linux-musleabi]
linker = "rust-lld"
```

### Building

```bash
cargo build --release --target armv5te-unknown-linux-musleabi
```

Your binary will be in `target/armv5te-unknown-linux-musleabi/release/`.

### Deployment

Transfer the binary to your EV3 and run it:

```bash
# Using scp
scp target/armv5te-unknown-linux-musleabi/release/my-robot robot@ev3dev.local:~/

# SSH into the EV3
ssh robot@ev3dev.local

# Make executable and run
chmod +x my-robot
./my-robot
```

### Tip: Use ev3-runner

For faster development, consider using [ev3-runner](https://github.com/kingananas20/ev3-runner) to automatically upload and run your programs:

```bash
cargo install ev3-runner

# On your EV3, start the server
ev3-runner server

# From your computer
ev3-runner client run ./target/armv5te-unknown-linux-musleabi/release/my-robot --host 192.168.x.x:6767
```

## Examples

### Basic Movement

```rust
use ev3_drivebase::{DriveBase, Motor, Direction, BrakeMode};
use ev3_drivebase::ev3dev_lang_rust::{Ev3Error, motors::MotorPort};

fn main() -> Result<(), Ev3Error> {
    let left = Motor::new(MotorPort::OutA, Direction::Clockwise);
    let right = Motor::new(MotorPort::OutB, Direction::CounterClockwise);
    let mut drivebase = DriveBase::new(left, right, 43.2, 185.0)?;
    
    drivebase.set_brake_mode(BrakeMode::Hold)?;
    
    // Drive in a square
    for _ in 0..4 {
        drivebase.drive(300, 500, true)?;  // 50cm forward
        drivebase.turn(200, 90, None)?;     // 90° turn
    }
    
    Ok(())
}
```

### Obstacle Avoidance Pattern

```rust
use ev3_drivebase::{DriveBase, Motor, Direction, BrakeMode};
use ev3_drivebase::ev3dev_lang_rust::{Ev3Error, motors::MotorPort};

fn main() -> Result<(), Ev3Error> {
    let left = Motor::new(MotorPort::OutD, Direction::Clockwise);
    let right = Motor::new(MotorPort::OutC, Direction::CounterClockwise);
    let mut drivebase = DriveBase::new(left, right, 43.2, 185.0)?;
    
    drivebase
        .set_brake_mode(BrakeMode::Hold)?
        .set_acceleration(3000)?;
    
    // Simple avoidance: back up and turn
    drivebase.drive(200, -150, true)?;   // Back up 15cm
    drivebase.turn(250, 45, None)?;      // Turn 45° right
    drivebase.drive(300, 300, true)?;    // Move forward 30cm
    
    Ok(())
}
```

### Smooth Curved Path

```rust
use ev3_drivebase::{DriveBase, Motor, Direction, BrakeMode};
use ev3_drivebase::ev3dev_lang_rust::{Ev3Error, motors::MotorPort};

fn main() -> Result<(), Ev3Error> {
    let left = Motor::new(MotorPort::OutA, Direction::Clockwise);
    let right = Motor::new(MotorPort::OutB, Direction::CounterClockwise);
    let mut drivebase = DriveBase::new(left, right, 43.2, 185.0)?;
    
    drivebase
        .set_brake_mode(BrakeMode::Brake)?
        .set_acceleration(2000)?
        .set_deceleration(2000)?;
    
    // Drive in a smooth S-curve
    drivebase.turn(300, 90, Some(300.0))?;   // Gentle left curve
    drivebase.turn(300, -90, Some(300.0))?;  // Gentle right curve
    
    Ok(())
}
```

## Documentation

For detailed API documentation, run:

```bash
cargo doc --open
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Related Projects

- [ev3dev-lang-rust](https://github.com/pixix4/ev3dev-lang-rust) - The underlying Rust bindings for ev3dev
- [ev3-runner](https://github.com/kingananas20/ev3-runner) - Fast development tool for uploading and running EV3 programs

## Acknowledgments

This crate is inspired by the DriveBase implementations in:
- The official ev3dev Python library
- The Pybricks MicroPython library

## Support

If you encounter any issues or have questions:
- Open an issue on [GitHub](https://github.com/Kingananas20/ev3dev-drivebase-rs/issues)
- Check the [ev3dev documentation](https://www.ev3dev.org/docs/)
