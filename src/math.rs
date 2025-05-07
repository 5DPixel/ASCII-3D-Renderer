#[derive(Debug, Clone, Copy)]
pub struct Matrix<const N: usize> {
    pub data: [[f32; N]; N]
}

#[macro_export]
macro_rules! identity_matrix {
    ($n:expr) => {{
        let mut data = [[0.0f32; $n]; $n];
        let mut i = 0;
        while i < $n {
            data[i][i] = 1.0;
            i += 1;
        }
        Matrix::<$n> { data }
    }};
}
