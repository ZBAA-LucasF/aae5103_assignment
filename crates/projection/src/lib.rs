use chrono::NaiveDateTime;
use dbscan::Point2D;

// 墨卡托投影最大纬度 (85.05112878 degrees in radians)
const MAX_LAT: f64 = 1.4844222297455562;
// WGS84椭球长半轴 (单位：米)
const A: f64 = 6378137.0;

/// 将地理纬度转换为墨卡托角
fn geodetic_latitude_to_mercator_angle(latitude_rad: f64) -> f64 {
    let clamped_lat = latitude_rad.clamp(-MAX_LAT, MAX_LAT);
    let sin_lat = clamped_lat.sin();
    0.5 * ((1.0 + sin_lat) / (1.0 - sin_lat)).ln()
}

/// 经纬度转Web墨卡托坐标
pub fn ll_to_wmc(lat_deg: f64, lon_deg: f64, time: NaiveDateTime) -> Point2D {
    let lat_rad = lat_deg.to_radians();
    let lon_rad = lon_deg.to_radians();

    let x = A * lon_rad;
    let y = A * geodetic_latitude_to_mercator_angle(lat_rad);

    Point2D::new(x, y, time)
}
