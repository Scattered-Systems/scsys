/*
    Appellation: impl_ops <module>
    Contrib: @FL03
*/
use super::H256;

impl core::ops::Add<Self> for H256 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut result_bytes = [0; 32];
        for n in 1..9 {
            let results: [u8; 4] = {
                let val = u32::from_be_bytes(self.0[4 * (n - 1)..4 * n].try_into().unwrap());
                let rhs = u32::from_be_bytes(rhs.0[4 * (n - 1)..4 * n].try_into().unwrap());
                let tmp = ((val as u64) + (rhs as u64)) as u32;
                tmp.to_be_bytes()
            };
            result_bytes[4 * (n - 1)] = results[0];
            result_bytes[4 * (n - 1) + 1] = results[1];
            result_bytes[4 * (n - 1) + 2] = results[2];
            result_bytes[4 * (n - 1) + 3] = results[3];
        }
        Self(result_bytes)
    }
}

impl core::ops::AddAssign<Self> for H256 {
    fn add_assign(&mut self, rhs: Self) {
        for n in 1..9 {
            let results: [u8; 4] = {
                let val = u32::from_be_bytes(self.0[4 * (n - 1)..4 * n].try_into().unwrap());
                let rhs = u32::from_be_bytes(rhs.0[4 * (n - 1)..4 * n].try_into().unwrap());
                let tmp = ((val as u64) + (rhs as u64)) as u32;
                tmp.to_be_bytes()
            };
            self[4 * (n - 1)] = results[0];
            self[4 * (n - 1) + 1] = results[1];
            self[4 * (n - 1) + 2] = results[2];
            self[4 * (n - 1) + 3] = results[3];
        }
    }
}

impl core::ops::Add<f64> for H256 {
    type Output = Self;

    fn add(self, rhs: f64) -> Self {
        let mut result_bytes = [0; 32];
        for n in 1..9 {
            let results: [u8; 4] = {
                let v = u32::from_be_bytes(self.0[4 * (n - 1)..4 * n].try_into().unwrap());
                let tmp = ((v as f64) + rhs) as u32;
                tmp.to_be_bytes()
            };
            result_bytes[4 * (n - 1)] = results[0];
            result_bytes[4 * (n - 1) + 1] = results[1];
            result_bytes[4 * (n - 1) + 2] = results[2];
            result_bytes[4 * (n - 1) + 3] = results[3];
        }
        Self(result_bytes)
    }
}

impl core::ops::AddAssign<f64> for H256 {
    fn add_assign(&mut self, rhs: f64) {
        for n in 1..9 {
            let results: [u8; 4] = {
                let v = u32::from_be_bytes(self.0[4 * (n - 1)..4 * n].try_into().unwrap());
                let tmp = ((v as f64) + rhs) as u32;
                tmp.to_be_bytes()
            };
            self[4 * (n - 1)] = results[0];
            self[4 * (n - 1) + 1] = results[1];
            self[4 * (n - 1) + 2] = results[2];
            self[4 * (n - 1) + 3] = results[3];
        }
    }
}

impl core::ops::Div<f64> for H256 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        let mut result_bytes = [0; 32];
        for n in 1..9 {
            let results: [u8; 4] = {
                let v = u32::from_be_bytes(self.0[4 * (n - 1)..4 * n].try_into().unwrap());
                let tmp = ((v as f64) / rhs) as u32;
                tmp.to_be_bytes()
            };
            result_bytes[4 * (n - 1)] = results[0];
            result_bytes[4 * (n - 1) + 1] = results[1];
            result_bytes[4 * (n - 1) + 2] = results[2];
            result_bytes[4 * (n - 1) + 3] = results[3];
        }
        Self(result_bytes)
    }
}

impl core::ops::DivAssign<f64> for H256 {
    fn div_assign(&mut self, rhs: f64) {
        for n in 1..9 {
            let results: [u8; 4] = {
                let v = u32::from_be_bytes(self.0[4 * (n - 1)..4 * n].try_into().unwrap());
                let tmp = ((v as f64) / rhs) as u32;
                tmp.to_be_bytes()
            };
            self[4 * (n - 1)] = results[0];
            self[4 * (n - 1) + 1] = results[1];
            self[4 * (n - 1) + 2] = results[2];
            self[4 * (n - 1) + 3] = results[3];
        }
    }
}

impl core::ops::Mul<f64> for H256 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        let mut result_bytes = [0; 32];
        for n in 1..9 {
            let results: [u8; 4] = {
                let v = u32::from_be_bytes(self.0[4 * (n - 1)..4 * n].try_into().unwrap());
                let tmp = ((v as f64) * rhs) as u32;
                tmp.to_be_bytes()
            };
            result_bytes[4 * (n - 1)] = results[0];
            result_bytes[4 * (n - 1) + 1] = results[1];
            result_bytes[4 * (n - 1) + 2] = results[2];
            result_bytes[4 * (n - 1) + 3] = results[3];
        }
        Self(result_bytes)
    }
}

impl core::ops::MulAssign<f64> for H256 {
    fn mul_assign(&mut self, rhs: f64) {
        for n in 1..9 {
            let results: [u8; 4] = {
                let v = u32::from_be_bytes(self.0[4 * (n - 1)..4 * n].try_into().unwrap());
                let tmp = ((v as f64) * rhs) as u32;
                tmp.to_be_bytes()
            };
            self[4 * (n - 1)] = results[0];
            self[4 * (n - 1) + 1] = results[1];
            self[4 * (n - 1) + 2] = results[2];
            self[4 * (n - 1) + 3] = results[3];
        }
    }
}

impl core::ops::Index<usize> for H256 {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl core::ops::IndexMut<usize> for H256 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl core::ops::Index<core::ops::Range<usize>> for H256 {
    type Output = [u8];

    fn index(&self, index: core::ops::Range<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl core::ops::IndexMut<core::ops::Range<usize>> for H256 {
    fn index_mut(&mut self, index: core::ops::Range<usize>) -> &mut Self::Output {
        &mut self.0[index]
    }
}
