fn bouncing_ball(h: f64,  bounce: f64,  window: f64) -> i32 {
    if h <= 0.0 || bounce >= 1.0 || bounce <= 0.0 || window >= h {
        return -1;
    }

    let mut no_of_times_seen = 0;
    let mut new_ball_height = h;

    loop {
        no_of_times_seen += 1;

        new_ball_height *= bounce;

        if new_ball_height > window {
            no_of_times_seen += 1;
        } else {
            return no_of_times_seen;
        }
    }
}
