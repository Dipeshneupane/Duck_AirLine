#![allow(unused_variables)]
fn main(){
    const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0;

    let route = [
        ("KCLE", 41.4075, -81.851111),
        ("LEYIR", 41.51030, -83.88080),
        ("PIONS", 41.65390, -84.48190),
        ("ZOSER", 41.72390, -84.78130),
        ("MODEM", 41.72800, -84.89730),
        ("BRYTO", 41.74170, -85.31320),
        ("FOD", 42.61110, -94.29480),
        ("KSLC", 40.7861, -111.9822)
    ];

    let mut total_distance = 0.0;
    let mut previous_waypoint : Option<(&str, f64, f64)> = None;

    for waypoint in route.iter(){
        match previous_waypoint {
            None => {
                previous_waypoint = Option::from(waypoint.clone());
                continue;
            }
            Some(previous_waypoint_value) => {
                let previous_waypoint_radian = previous_waypoint_value.1.to_radians();
                let waypoint_radians = waypoint.1.to_radians();

                let delta_latidute = (previous_waypoint_value.1 - waypoint.1).to_radians();
                let delta_longitude = (previous_waypoint_value.2 - waypoint.2).to_radians();

                let inner_central_angle = f64::powi((delta_latidute/2.0).sin(),2)
                    + previous_waypoint_radian.cos() * waypoint_radians.cos()
                    * f64::powi((delta_longitude/2.0).sin(),2);

                let central_angle = 2.0 * inner_central_angle.sqrt().asin();
                let distance = EARTH_RADIUS_IN_KILOMETERS * central_angle;

                total_distance += distance;
                previous_waypoint = Option::from(waypoint.clone());

                println!("The distance between {} and {} is {:.1} kilometers", previous_waypoint_value.0, waypoint.0, distance);

            }
        }
    }
}
