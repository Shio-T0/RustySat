use core::todo;

/// The current State of the Can,
///
/// The ground station should show every state by decoding its number (preserve thy bytes)
#[repr(u8)]
pub enum CanState {
    /// Just turned the Can On, start every component -> Here until self-tests end
    Boot = 0,

    /// Send message to Ground station -> Here until > 3G accel (for more than 0.1s) is detected
    /// + [barometer backup] and [set mpu at ±16]
    Pad = 1,

    /// Nothing to do here, -> Here until high jerk or force after 20s
    Ascent = 2,

    /// Deployed -> Here until parafoil is operational (we'd guess by
    /// using at least 4s after launch and barometer shows falling for 1s, force after 20s)
    Deploy = 3,

    /// Parafoil operational, wait for around 4s to settle + some sink rate -> Here until that was completed
    Settle = 4,

    /// Do one 360deg circle and measure air velocity -> Here until one circle is complete
    Wind = 5,

    /// Start heading to waypoint -> Here until near waypoint or too strong a wind
    Homing = 6,

    /// Circle around to lose height -> Here until landing (Can not moving) or too strong a wind
    Loiter = 7,

    /// Case where wind is too strong(or we are bellow 40m), just head towards it -> Here until landing, Can stopped
    Final = 8,

    /// Can stopped moving, hence landed -> Here for 4s, checking if actually landed
    Landed = 9,

    /// Start beeping and send GPS TM -> Here until we check it actually worked ...
    Recovery = 10,

    /// Failing Case, back to neutral brakes -> Enter when:
    /// - no GPS fix for 5 s;
    /// - a spin much faster than your planned turns (say > 180°/s for 2 s);
    /// - sinking faster than 12 m/s for 3 s, which is outside rule 5.11 and means the wing has collapsed; (might not be worth it but its here)
    /// - low battery or a sensor fault.
    FailSafe = 11,
}

// REMEMBER: to save everything in the SD card

pub fn next(s: CanState, ms_in_state: u32, ms_since_launch: Option<u32>) -> CanState {
    match s {
        CanState::Boot => CanState::Pad,
        CanState::Pad => CanState::Ascent,
        CanState::Ascent => CanState::Deploy,
        CanState::Deploy => CanState::Settle,
        CanState::Settle => CanState::Wind,
        CanState::Wind => CanState::Homing,
        CanState::Homing => CanState::Loiter,
        CanState::Loiter => CanState::Final,
        CanState::Final => CanState::Landed,
        CanState::Landed => CanState::Recovery,
        CanState::Recovery => CanState::Recovery,
        CanState::FailSafe => {
            // TODO: Make a return to operational checker.

            todo!()
        }
    }
}
