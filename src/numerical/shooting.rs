#![allow(dead_code)]

use std::time::Duration;

const GRAVITY: f64 = 9.8; // m/sec^2
const PI: f64 = std::f64::consts::PI;
fn rpm2omega(rpm: f64) -> f64 {
    rpm * PI / 30.0
}

fn omega2rpm(omega: f64) -> f64 {
    omega * 30.0 / PI
}

fn deg2rad(degree: f64) -> f64 {
    degree * PI / 180.0
}

fn rad2deg(radian: f64) -> f64 {
    radian * 180.0 / PI
}

#[derive(Debug)]
struct ShooterConfig {
    wheel_weight: f64,
    wheel_radius: f64,
    ball_weight: f64,
    ball_radius: f64,
    arc_angle: f64,   // the arc angle of the track. this determines the contact time
    motor_power: f64, // the output power of motor
    omega: f64,       // equilibrium rotation speed of the flywheel.
    // this decides the energy level of the flywheel energy reservoir
    min_height: f64,
    max_height: f64,
}

impl Default for ShooterConfig {
    fn default() -> ShooterConfig {
        ShooterConfig {
            wheel_weight: 142.0 * 2.0 / 1000.0, // weight in kg. it's double wheel, each weigh 142g
            wheel_radius: 96.0 / 2.0 / 1000.0,  // radius in m, given diameter = 96 mm
            ball_weight: 0.165 * 0.453592,      // weight in kg. 0.165 pounds
            ball_radius: 5.0 / 2.0 * 0.0254,    // radius in m. diameter = 5.0 inches
            motor_power: 90.0,                  // watt
            omega: rpm2omega(6000.0),           // in radians
            arc_angle: deg2rad(90.0),           // arc in radians
            min_height: 98.45 / 100.0,          // the height of the front edge of the goal
            max_height: (98.45 + 38.10) / 100.0, // the height of the back ridge of the goal
        }
    }
}

impl ShooterConfig {
    /// weight in grams
    /// power in watt
    /// angle in degree
    /// omega in RPM
    /// min_height (max_height) in mm
    fn new(
        wheel_rpm: Option<f64>,
        min_height: Option<f64>,
        max_height: Option<f64>,
        wheel_weight_grams: Option<f64>,
        motor_power_watts: Option<f64>,
        arc_angle_degrees: Option<f64>,
    ) -> Self {
        let mut conf = ShooterConfig::default();
        if let Some(wheel_weight) = wheel_weight_grams {
            conf.wheel_weight = wheel_weight / 1000.0;
        }
        if let Some(motor_power) = motor_power_watts {
            conf.motor_power = motor_power;
        }
        if let Some(arc_angle) = arc_angle_degrees {
            conf.arc_angle = deg2rad(arc_angle);
        }
        if let Some(rpm) = wheel_rpm {
            conf.omega = rpm2omega(rpm);
        }
        if let Some(min_height) = min_height {
            conf.min_height = min_height / 100.0;
        }
        if let Some(max_height) = max_height {
            conf.max_height = max_height / 100.0;
        }

        conf
    }
}

#[derive(Debug)]
struct Wheel {
    weight: f64,
    radius: f64,
}

trait Rot {
    fn inertia(&self) -> f64;
}

impl Wheel {
    fn new(weight: f64, radius: f64) -> Wheel {
        Wheel { weight, radius }
    }
    pub(crate) fn kin(&self, omega: f64) -> f64 {
        omega.powi(2) * self.inertia() / 2.0
    }

    pub(crate) fn omega(&self, energy: f64) -> f64 {
        (2.0 * energy / self.inertia()).sqrt()
    }
}

impl Rot for Wheel {
    /// M R^2 / 2 for solid cylinder and M R^2 for shell
    /// The wheel we use is neither. So, we approximate
    fn inertia(&self) -> f64 {
        self.weight * self.radius.powi(2) / 2.0
    }
}

#[derive(Debug)]
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
    /// I = 2/3 M R^2 around GC
    /// by parallel axis theorem
    /// I = 5/3 M R^ 2 around tangent point on the ball
    fn inertia(&self) -> f64 {
        5f64 / 3f64 * self.weight * self.radius.powi(2)
    }
}

#[derive(Debug)]
struct Shooter {
    ball: Ball,
    wheel: Wheel,
    arc_angle: f64, // the arc angle of the track.
    // this determines the time when the wheel and ball are in contact
    // omega: f64, // equilibrium rotation speed of the flywheel.
    // this decides the energy level of the flywheel energy reservoir
    motor_power: f64, // the output power of motor
    min_height: f64,
    max_height: f64,
}

impl Shooter {
    fn new(config: ShooterConfig) -> Shooter {
        Shooter {
            ball: Ball::new(config.wheel_radius, config.wheel_weight),
            wheel: Wheel::new(config.wheel_weight, config.wheel_radius),
            arc_angle: config.arc_angle,
            // omega: config.omega,
            motor_power: config.motor_power,
            min_height: config.min_height,
            max_height: config.max_height,
        }
    }

    /// calculate the interval (duration) to power the flywheel
    /// both initial and target rotation speed are given in RPM
    fn charge(&self, init_rpm: f64, target_rpm: f64) -> Duration {
        if target_rpm <= init_rpm {
            panic!(
                "Target rpm {} is smaller than init_rpm {}",
                target_rpm, init_rpm
            );
        }
        let init_omega = rpm2omega(init_rpm);
        let target_omega = rpm2omega(target_rpm);
        let original = self.wheel.kin(init_omega);
        let target_energy = self.wheel.kin(target_omega);
        let denergy = target_energy - original;
        let sec = denergy / self.motor_power;
        Duration::from_secs_f64(sec)
    }

    /// given flywheel rotation RPM, calculate the projection speed of the ball
    fn proj_speed(&self, rpm: f64) -> f64 {
        // Model of the loss of angular momentum on the flywheel
        // let f = the friction between ball and track
        // let F = friction between the flywheel and ball
        // immediately after the impact,
        // the angular moment of flywheel = I1 ω1 - R1 F t0
        // the angular moment of the ball = I2 ω2 - R2 F t
        // the linear speed of the ball = F t / m2
        // assuming f << F
        let factor = 2.0 * self.wheel.weight / (4.0 * self.ball.weight + 5.0 * self.wheel.weight);
        let omega = rpm2omega(rpm);
        factor * self.wheel.radius * omega
    }

    /// To shoot a ball to goal at (dist, height), both in meters
    /// the angular velocity of the flywheel decreases due to the lost of kinetic energy.
    /// calculate the required initial and ending flywheel rotation speed in RPM
    fn rpms(&self, dist: f64, height: f64) -> (f64, f64) {
        let v = self.ball.min_velocity(dist, height);
        let factor = 2.0 * self.wheel.weight / (4.0 * self.ball.weight + 5.0 * self.wheel.weight);
        let init = v / self.wheel.radius / factor;
        let ending = 2.5 * v / self.wheel.radius;
        (omega2rpm(init), omega2rpm(ending))
    }

    /// To shoot three balls from a distance
    /// return the minimum duration and the maximum RPM of the flywheel
    fn shoot_evenly(&self, dist: f64) -> Duration {
        let middle = (self.max_height + self.min_height) / 2.0;
        let (rpm, end_rpm) = self.rpms(dist, middle);

        let dur1 = self.charge(end_rpm, rpm);

        println!(
            "{rpm} -shoot-> {end_rpm} -charge-> {rpm} -shoot-> {end_rpm} -charge-> {rpm} -shoot-> {end_rpm}"
        );

        dur1 + dur1
    }

    fn shoot_cascading(&self, dist: f64) -> Duration {
        // first shoot can aim the max_height
        let (rpm1, end_rpm1) = self.rpms(dist, self.max_height);

        // second shoot can aim anywhere in the middle
        let middle = (self.max_height + self.min_height) / 2.0;
        let (rpm2, end_rpm2) = self.rpms(dist, middle);
        // last shoot can aim the min_height
        let (rpm3, end_rpm3) = self.rpms(dist, self.min_height);

        let dur1 = self.charge(end_rpm1, rpm2);
        let dur2 = self.charge(end_rpm2, rpm3);

        println!(
            "{rpm1} -shoot-> {end_rpm1} -charge-> {rpm2} -shoot-> {end_rpm2} -charge-> {rpm3} -shoot-> {end_rpm3}"
        );

        dur1 + dur2
    }
}

#[cfg(test)]
mod tests {
    use crate::numerical::shooting::{Ball, Rot, Shooter, ShooterConfig};
    use std::time::Duration;

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
    fn test_shoot_cascading() {
        let conf = ShooterConfig::new(None, Some(117.0), Some(130.0), None, None, None);

        let shooter = Shooter::new(conf);
        let duration = shooter.shoot_cascading(1f64);
        assert_prox!(duration.as_millis() as f64, 66f64, 1e-2);
    }
    #[test]

    fn test_shoot_evenly() {
        let conf = ShooterConfig::new(None, Some(117.0), Some(130.0), None, None, None);

        let shooter = Shooter::new(conf);
        let duration = shooter.shoot_evenly(1f64);
        assert_prox!(duration.as_millis() as f64, 78f64, 1e-2);
    }

    #[test]
    fn test_wheel_ball_weight_ratio() {
        let conf = ShooterConfig::default();
        assert_prox!(conf.ball_weight / conf.wheel_weight, 0.263f64, 1e-2);
    }

    #[test]
    fn test_proj_speed() {
        let conf = ShooterConfig::default();
        let shooter = Shooter::new(conf);
        let rpm = 6000.0;
        let speed = shooter.proj_speed(rpm);
        assert_prox!(speed, 10.62f64, 1e-2);
    }

    #[test]
    fn test_charge_zero_to_target() {
        let conf = ShooterConfig::default();
        let shooter = Shooter::new(conf);
        let target_rpm = 6000.0;
        let intv = shooter.charge(0.0, target_rpm);
        assert_prox!(intv.as_millis() as f64, 717f64, 1e-2);
    }

    #[test]
    fn test_compensation_charge() {
        let conf = ShooterConfig::default();
        let shooter = Shooter::new(conf);
        let init_rpm = 5900.0;
        let target_rpm = 6000.0;
        let intv = shooter.charge(init_rpm, target_rpm);
        assert_prox!(intv.as_millis() as f64, 23f64, 1e-2);
    }
}
