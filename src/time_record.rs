use core::fmt;
use std::ops::{AddAssign};

#[derive(Clone, Copy)]
pub struct TimeRecord {
    pub key_gen: u128,
    pub encap: u128,
    pub decap: u128
}

impl Default for TimeRecord {
    fn default() -> Self {
        TimeRecord { key_gen: 0,
                    encap: 0,
                    decap: 0 }
    }
}

impl AddAssign for TimeRecord {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            encap: self.encap + rhs.encap,
            decap: self.decap + rhs.decap,
            key_gen: self.key_gen + rhs.key_gen
        };
    }
}

impl fmt::Display for TimeRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:<15} {:<15} {:<15}",
                self.key_gen,
                self.encap,
                self.decap)
    }
}

impl TimeRecord {
    pub fn divide(&mut self, number: u128) {
        self.key_gen /= number;
        self.encap /= number;
        self.decap /= number;
    }
}

pub struct TimeHolder {
    pub kyber_1024: TimeRecord,
    pub kyber_1024_90s: TimeRecord,
    pub kyber_768: TimeRecord,
    pub kyber_768_90s: TimeRecord,
    pub kyber_512: TimeRecord,
    pub kyber_512_90s: TimeRecord 
}

impl Default for TimeHolder {
    fn default() -> Self {
        TimeHolder { kyber_1024: TimeRecord::default(),
                    kyber_1024_90s: TimeRecord::default(),
                    kyber_768: TimeRecord::default(),
                    kyber_768_90s: TimeRecord::default(),
                    kyber_512: TimeRecord::default(),
                    kyber_512_90s: TimeRecord::default() }
    }
}

impl  fmt::Display for TimeHolder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:<25}{:<15} {:<15} {:<15}",
                "",
                "Keygen (us)",
                "Encap (us)",
                "Decap (us)").ok();
        write!(f, "{:<25}", "Kyber_1024").ok();
        writeln!(f, "{}", self.kyber_1024).ok();
        write!(f, "{:<25}", "Kyber_1024_90s").ok();
        writeln!(f, "{}", self.kyber_1024_90s).ok();
        write!(f, "{:<25}", "Kyber_768").ok();
        writeln!(f, "{}", self.kyber_768).ok();
        write!(f, "{:<25}", "Kyber_768_90s").ok();
        writeln!(f, "{}", self.kyber_768_90s).ok();
        write!(f, "{:<25}", "Kyber_512").ok();
        writeln!(f, "{}", self.kyber_512).ok();
        write!(f, "{:<25}", "Kyber_512_90s").ok();
        writeln!(f, "{}", self.kyber_512_90s)
    }
}
