pub(crate) fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance
    let mut r0 = (1. - ref_idx) / (1. + ref_idx);
    r0 = r0.powi(2);
    r0 + (1. - r0) * (1. - cosine).powi(5)
}
