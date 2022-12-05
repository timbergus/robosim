use js_sys::Array;
use nalgebra::*;
use wasm_bindgen::prelude::*;

pub struct Join {
    theta: f64,
    d: f64,
    a: f64,
    alpha: f64,
}

pub fn dh_matrix(join: Join) -> Matrix4<f64> {
    let theta = join.theta.to_radians();
    let alpha = join.alpha.to_radians();

    let m = Matrix4::new(
        f64::cos(theta),
        -f64::cos(alpha) * f64::sin(theta),
        f64::sin(alpha) * f64::sin(theta),
        join.a * f64::cos(theta),
        f64::sin(theta),
        f64::cos(alpha) * f64::cos(theta),
        -f64::sin(alpha) * f64::cos(theta),
        join.a * f64::sin(theta),
        0.0,
        f64::sin(alpha),
        f64::cos(alpha),
        join.d,
        0.0,
        0.0,
        0.0,
        1.0,
    );

    return m;
}

#[wasm_bindgen]
pub fn get_robot(theta_1: f64, theta_2: f64, theta_3: f64, l_1: f64, l_2: f64, l_3: f64) -> Array {
    let join_0 = Join {
        theta: 0.0,
        d: 0.0,
        a: 0.0,
        alpha: 0.0,
    };

    let join_1 = Join {
        theta: theta_1,
        d: 0.0,
        a: l_1,
        alpha: 0.0,
    };

    let join_2 = Join {
        theta: theta_2,
        d: 0.0,
        a: l_2,
        alpha: 0.0,
    };

    let join_3 = Join {
        theta: theta_3,
        d: 0.0,
        a: l_3,
        alpha: 0.0,
    };

    let dh_0 = dh_matrix(join_0);
    let dh_1 = dh_matrix(join_1);
    let dh_2 = dh_matrix(join_2);
    let dh_3 = dh_matrix(join_3);

    let dot = Matrix4x1::new(0.0, 0.0, 0.0, 1.0);

    let join_0 = dh_0 * dot;
    let join_1 = dh_0 * dh_1 * dot;
    let join_2 = dh_0 * dh_1 * dh_2 * dot;
    let join_3 = dh_0 * dh_1 * dh_2 * dh_3 * dot;

    let robot = Array::new();

    for m in (join_0).iter() {
        robot.push(&JsValue::from_f64(*m));
    }

    for m in (join_1).iter() {
        robot.push(&JsValue::from_f64(*m));
    }

    for m in (join_2).iter() {
        robot.push(&JsValue::from_f64(*m));
    }

    for m in (join_3).iter() {
        robot.push(&JsValue::from_f64(*m));
    }

    return robot;
}
