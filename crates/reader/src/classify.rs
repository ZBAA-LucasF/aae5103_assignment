use crate::models::{AdsbData, Track};
use chrono::Duration;
use std::collections::HashMap;

/// 分类 ADS-B 数据到航迹
pub(crate) fn classify_to_tracks(adsbs: &[AdsbData]) -> Vec<Track> {
    let mut result = Vec::new();

    // 分组键类型
    type GroupKey = (String, String);

    // 第一步：按呼号、ICAO地址和高度过滤分组
    let mut groups: HashMap<GroupKey, Vec<AdsbData>> = HashMap::new();

    for adsb in adsbs.iter().filter(|d| d.altitude <= 19800) {
        let data = adsb;
        let key = (data.callsign.clone(), data.icao_address.clone());
        groups.entry(key).or_default().push(data.clone());
    }

    // 处理每个分组
    for ((callsign, icao_address), mut data) in groups.into_iter() {
        // 按时间排序
        data.sort_by(|a, b| a.datetime.cmp(&b.datetime));

        let mut tracks = Vec::new();
        let mut current_track = if let Some(first) = data.first() {
            Track::new(&callsign, &icao_address, first)
        } else {
            continue;
        };

        // 分割时间窗口
        for d in data.iter().skip(1) {
            let time_diff = d.datetime - current_track.last_time;

            if time_diff <= Duration::minutes(30) {
                current_track.data.push(d.clone());
                current_track.last_time = d.datetime;
            } else {
                if current_track.data.len() >= 15 {
                    tracks.push(current_track);
                }
                current_track = Track::new(&callsign, &icao_address, d);
            }
        }

        // 处理最后一个航迹
        if current_track.data.len() >= 15 {
            tracks.push(current_track);
        }

        // 去重处理
        for track in &mut tracks {
            track.data.sort_by(|a, b| a.datetime.cmp(&b.datetime));
            track.data.dedup_by(|a, b| {
                (a.lat == b.lat)
                    && (a.long == b.long)
                    && (a.altitude == b.altitude)
                    && (a.heading == b.heading)
                    && (a.speed == b.speed)
                    && (a.vertical_speed == b.vertical_speed)
                    && (a.datetime == b.datetime)
            });
        }

        result.extend(tracks);
    }

    result
}
