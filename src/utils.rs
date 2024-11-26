pub fn calculate_angle(delta_x: f32, delta_y: f32) -> f32 {
    // atan2 gives the angle in radians relative to the positive X-axis.
    let angle_radians = delta_y.atan2(delta_x);

    // Convert the angle to degrees and adjust to start from north (0 degrees).
    let angle_degrees = angle_radians.to_degrees();
    let adjusted_angle = (90.0 - angle_degrees).rem_euclid(360.0); // Normalize to [0, 360)
    
    adjusted_angle
}