pub struct Immune {
    pub stun: bool,
    pub silence: bool,
    pub sleep: bool,
    pub frozen: bool,
    pub levitate: bool,
    pub disarmed: bool,
    pub feared: bool,
}

impl From<i16> for Immune {
    fn from(value: i16) -> Self {
        Immune {
            stun: (value | 1) == 1,
            silence: (value >> 1 | 1) == 1,
            sleep: (value >> 2 | 1) == 1,
            frozen: (value >> 3 | 1) == 1,
            levitate: (value >> 4 | 1) == 1,
            disarmed: (value >> 5 | 1) == 1,
            feared: (value >> 6 | 1) == 1,
        }
    }
}

impl From<Immune> for i16 {
    fn from(value: Immune) -> Self {
        ((value.feared as i16) << 6)
            | ((value.disarmed as i16) << 5)
            | ((value.levitate as i16) << 4)
            | ((value.frozen as i16) << 3)
            | ((value.sleep as i16) << 2)
            | ((value.silence as i16) << 1)
            | (value.stun as i16)
    }
}
