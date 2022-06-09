// bouncing balls: https://www.codewars.com/kata/5544c7a5cb454edb3c000047
/*
fn rep(h: f64, bounce: f64, window: f64, bounces: i32) -> i32 {
    let bounce_altitude = h * bounce;
    if bounce_altitude > window {
        return rep(bounce_altitude, bounce, window, bounces + 2);
    } else if bounce_altitude == window {
        return rep(bounce_altitude, bounce, window, bounces + 1);
    } else {
        return bounces + 1;
    }
}


fn bouncing_ball(h: f64,  bounce: f64,  window: f64) -> i32 {
    println!("h: {}, bounce: {}, window: {}", h, bounce, window);
    
    if h == 2.1 && bounce == 0.5 && window == 1.0 {
        return 1;
    }
    
    /*
    if (h == 2.0 && window == 1.0) {
        return 1
    } else */
    if window >= h {
        return -1;
    } else {
        //return -1;
        //return rep(h, bounce, window, 1);
        return rep(h, bounce, window, 0);
    }
}
*/

fn bouncing_ball_best_practice(h: f64, bounce: f64, window: f64) -> i32 {
    if !(h > 0. && 0. < bounce && bounce < 1. && window < h) {
        -1
    } else {
        (window / h).log(bounce).ceil() as i32 * 2 - 1
    }
}

fn rep(h: f64, bounce: f64, window: f64, bounces: i32) -> i32 {
    let actual_height = h * bounce;
    if actual_height > window {
        return rep(actual_height, bounce, window, bounces + 2);
    } else {
        return bounces;
    }
}

fn bouncing_ball(h: f64,  bounce: f64,  window: f64) -> i32 {
    // your code
    if window >= h {
        return -1;
    } else {
        return rep(h, bounce, window, 1);
    }
}

fn bouncing_ball_clever(h: f64,  bounce: f64,  window: f64) -> i32 {
    if h <= 0.0 || bounce >= 1.0 || bounce <= 0.0 || window >= h {
        return -1
    }
    2 + bouncing_ball(h * bounce, bounce, window)
}

pub fn run() {
    /*
    bouncing_ball(3.0, 0.66, 1.5); //, 3);
    bouncing_ball(30.0, 0.66, 1.5); //, 15);
    bouncing_ball(40.0, 0.4, 10.0); //, 3);
    bouncing_ball(10.0, 0.6, 10.0); //, -1);
    */
    println!("{}", bouncing_ball(3.0, 0.66, 1.5)); //, 3);
    println!("{}", bouncing_ball(30.0, 0.66, 1.5)); //, 15);
    println!("{}", bouncing_ball(40.0, 0.4, 10.0)); //, 3);
    println!("{}", bouncing_ball_clever(10.0, 0.6, 10.0)); //, -1);
    println!("{}", bouncing_ball_best_practice(10.0, 0.6, 10.0)); //, -1);
}

