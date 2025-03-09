use fraction::GenericFraction;

pub fn frac_to_tuple(frac: GenericFraction<u64>) -> (u32, u32) {
    (
        *frac.numer().unwrap_or(&0) as u32,
        *frac.denom().unwrap_or(&0) as u32,
    )
}
