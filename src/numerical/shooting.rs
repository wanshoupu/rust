const GRAVITY: f64 = 9.8; // m/sec^2
const PI: f64 = std::f64::consts::PI;
struct ShooterConfig {
    wheel_weight: f64,
    wheel_radius: f64,
    ball_weight: f64,
    ball_radius: f64,
    arc_angle: f64,   // the arc angle of the track. this determines the contact time
    motor_power: f64, // the output power of motor
    eq_omega: f64,    // equilibrium rotation speed of the flywheel.
                      // this decides the energy level of the flywheel energy reservoir
}

impl Default for ShooterConfig {
    fn default() -> ShooterConfig {
        ShooterConfig {
            wheel_weight: 1.0 * 0.453592,      // weight in kg. 1 pound
            wheel_radius: 96.0 / 2.0 / 1000.0, // radius in inches, given diameter = 96 mm
            ball_weight: 0.165 * 0.453592,     // weight in kg. 0.165 pounds
            ball_radius: 5.0 / 2.0 * 0.0254,   // radius in m. diameter = 5.0 inches
            motor_power: 90.0,
            arc_angle: 90. * std::f64::consts::PI / 180.0, // arc in radians
            eq_omega: 6000.0,
        }
    }
}

impl ShooterConfig {
    fn new(wheel_weight: f64, motor_power: f64, arc_angle: f64, eq_omega: f64) -> Self {
        let mut conf = ShooterConfig::default();
        conf.wheel_weight = wheel_weight;
        conf.motor_power = motor_power;
        conf.arc_angle = arc_angle;
        conf.eq_omega = eq_omega;

        conf
    }
}

struct Wheel {
    weight: f64,
    radius: f64,
    motor_power: f64, // the output power of motor
    eq_omega: f64,    // equilibrium rotation speed of the flywheel.
                      // this decides the energy level of the flywheel energy reservoir
}

trait Rot {
    fn inertia(&self) -> f64;
}

impl Wheel {
    fn new(weight: f64, radius: f64, motor_power: f64, eq_omega: f64) -> Wheel {
        Wheel {
            weight,
            radius,
            motor_power,
            eq_omega,
        }
    }
}

impl Rot for Wheel {
    /// M R^2 / 2 for solid cylinder and M R^2 for shell
    /// The wheel we use is neither. So, we approximate
    fn inertia(&self) -> f64 {
        self.weight * self.radius.powi(2) / 1.5
    }
}

struct Ball {
    weight: f64,
    radius: f64,
}
impl Ball {
    fn new(weight: f64, radius: f64) -> Ball {
        Ball { weight, radius }
    }

    /// given the horizontal distance from and the vertical height of the goal, both in meters,
    /// calculate the velocity, assume no dragging, no Magnus effect
    /// θ the angle
    /// v^2 = gl^2 / (l sin(2θ) - h cos(2θ) - h)
    /// the minimum velocity is achieved when cos(2θ) + h/l sin(2θ) = 0.
    /// let a = h/l
    /// min velocity: v^2 = gl^2 / a / (sqrt(1+a^2) - 1)
    /// where h = height, l = dist, g = gravity acceleration
    /// velocity unit: m / sec
    fn min_velocity(&self, dist: f64, height: f64) -> f64 {
        let hypo = (dist.powi(2) + height.powi(2)).sqrt();
        let v2 = GRAVITY * (height + hypo);
        v2.sqrt()
    }

    fn min_energy(&self, dist: f64, height: f64) -> f64 {
        let v = self.min_velocity(dist, height);
        let lin_kin = self.weight * v.powi(2) / 2f64;
        let omega = v / self.radius;
        let rot_kin = self.inertia() * omega.powi(2) / 2f64;
        lin_kin + rot_kin
    }
}

impl Rot for Ball {
    /// I = 2/3 M R^2
    fn inertia(&self) -> f64 {
        2f64 / 3f64 * self.weight * self.radius.powi(2)
    }
}

struct Shooter {
    ball: Ball,
    wheel: Wheel,
    arc_angle: f64, // the arc angle of the track.
                    // this determines the time when the wheel and ball are in contact
}

impl Shooter {
    fn new(config: ShooterConfig) -> Shooter {
        Shooter {
            ball: Ball::new(config.wheel_radius, config.wheel_weight),
            wheel: Wheel::new(
                config.wheel_weight,
                config.wheel_radius,
                config.motor_power,
                config.eq_omega,
            ),
            arc_angle: config.arc_angle,
        }
    }

    /// Rotational energy of flywheel
    /// transfer of rotational kinetic energy to ball
    /// as rotational and linear kinetic energy

    /// minimum shooting interval to shoot a target at (dist, height)
    /// unit of dist and height are inches
    /// return value is in unit of milliseconds
    fn interval(&self, dist: f64, height: f64) -> f64 {
        let energy = self.ball.min_energy(dist, height);
        // assuming non-slipping motion along the track
        let omega = self.ball.min_velocity(dist, height) / self.ball.radius;
        // assume contact time is ignorably short
        // to transfer energy to ball, it takes factor as much from flywheel
        let factor = self.wheel.eq_omega / omega;

        // the time it takes to replenish the lost energy
        factor * energy / self.wheel.motor_power
    }
}

#[cfg(test)]
mod tests {
    use crate::numerical::shooting::{Ball, Rot, Shooter, ShooterConfig};
    use std::f32::consts::PI;

    macro_rules! assert_prox {
        ($x:expr, $y:expr, $e:expr) => {{
            let d = ($x - $y).abs();
            assert!(d < $e, "abs({} - {}) >= tolerance {}", $x, $y, $e);
        }};
    }
    #[test]
    fn test_ball_velocity_vertical() {
        let ball = Ball::new(1.0, 2.0);
        let height = 2.0;
        let v = ball.min_velocity(0.0, height);
        assert_prox!(v, (2.0 * super::GRAVITY * height).sqrt(), 1e-5)
    }
    #[test]
    fn test_ball_velocity_horizontal() {
        let ball = Ball::new(1.0, 2.0);
        let dist = 2.0;
        let v = ball.min_velocity(dist, 0.0);
        assert_prox!(v, (super::GRAVITY * dist).sqrt(), 1e-5)
    }

    #[test]
    fn test_ball_velocity_45_degrees() {
        let ball = Ball::new(1.0, 2.0);
        let height = 1.0;
        let v = ball.min_velocity(height, height);
        let expected = (super::GRAVITY * height * (1.0 + 2.0f64.sqrt())).sqrt();
        assert_prox!(v, expected, 1e-5)
    }

    #[test]
    fn test_ball_energy_45_degrees() {
        let ball = Ball::new(1.0, 2.0);
        let height = 1.0;
        let energy = ball.min_energy(height, height);
        let kin = (super::GRAVITY * height * (1.0 + 2.0f64.sqrt())) / 2.0;
        let velocity = (super::GRAVITY * height * (1.0 + 2.0f64.sqrt())).sqrt();
        let rot = ball.inertia() * (velocity / ball.radius).powi(2) / 2.0;
        assert_prox!(energy, kin + rot, 1e-5)
    }

    #[test]
    fn test_interval() {
        let conf = ShooterConfig::default();
        let shooter = Shooter::new(conf);
        let intv = shooter.interval(10f64, 10f64);
        assert_prox!(intv, 200f64, 1e-2);
    }
}
