use chrono::{Duration, NaiveDateTime};
use std::cmp::Ordering;
use std::collections::VecDeque;

// 二维点结构
#[derive(Debug, Clone, PartialEq)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
    pub time: NaiveDateTime,
}

impl Point2D {
    pub fn new(x: f64, y: f64, time: NaiveDateTime) -> Self {
        Point2D { x, y, time }
    }
}

// 轨迹结构
#[derive(Debug, Clone)]
pub struct Trajectory {
    pub id: usize,
    pub points: Vec<Point2D>,
}

// 轨迹聚类包装结构
#[derive(Debug)]
pub struct TrajectoryClusterWrapper {
    pub trajectory: Trajectory,
    pub cluster_id: Option<usize>,
    is_classified: bool,
}

impl TrajectoryClusterWrapper {
    pub fn new(trajectory: Trajectory) -> Self {
        TrajectoryClusterWrapper {
            trajectory,
            cluster_id: None,
            is_classified: false,
        }
    }
}

/// 轨迹重采样函数
/// 参数：
/// - trajectory: 原始轨迹点切片
/// - num_points: 目标采样点数
fn resample_trajectory(trajectory: &[Point2D], num_points: usize) -> Vec<Point2D> {
    // 处理边界情况
    if trajectory.len() < 2 || num_points < 2 {
        return trajectory.to_vec();
    }

    // 计算总时间跨度（秒）
    let total_secs = (trajectory.last().unwrap().time - trajectory[0].time).num_seconds() as f64;

    // 计算时间步长（秒）
    let time_step = total_secs / (num_points - 1) as f64;

    let mut resampled = Vec::with_capacity(num_points);
    resampled.push(trajectory[0].clone()); // 保留第一个点

    let mut current_time = trajectory[0].time + Duration::milliseconds((time_step * 1000.0) as i64);
    let mut i = 1; // 原始轨迹索引

    // 生成中间点
    while resampled.len() < num_points - 1 && i < trajectory.len() {
        match current_time.cmp(&trajectory[i].time) {
            Ordering::Less => {
                // 执行线性插值
                let prev = &trajectory[i - 1];
                let curr = &trajectory[i];

                let delta_t = (curr.time - prev.time).num_seconds() as f64;
                let fraction = if delta_t > 0.0 {
                    (current_time - prev.time).num_seconds() as f64 / delta_t
                } else {
                    0.0
                };

                let x = prev.x + fraction * (curr.x - prev.x);
                let y = prev.y + fraction * (curr.y - prev.y);

                resampled.push(Point2D::new(x, y, current_time));

                // 移动到下一个时间点
                current_time += Duration::milliseconds((time_step * 1000.0) as i64);
            }
            Ordering::Greater => {
                // 移动到下一个原始点
                i += 1;
            }
            Ordering::Equal => {
                // 精确匹配时间点
                resampled.push(trajectory[i].clone());
                current_time += Duration::milliseconds((time_step * 1000.0) as i64);
                i += 1;
            }
        }
    }

    // 确保最后一个点被保留
    if resampled.len() < num_points {
        resampled.push(trajectory.last().unwrap().clone());
    }

    resampled
}

// 标准化处理
fn standardize_trajectories(trajectories: &mut [TrajectoryClusterWrapper]) {
    for wrapper in trajectories.iter_mut() {
        let points = &mut wrapper.trajectory.points;

        // 计算统计量
        let (sum_x, sum_y) = points
            .iter()
            .fold((0.0, 0.0), |(sx, sy), p| (sx + p.x, sy + p.y));

        let n = points.len() as f64;
        let mean_x = sum_x / n;
        let mean_y = sum_y / n;

        let std_dev_x = (points.iter().map(|p| (p.x - mean_x).powi(2)).sum::<f64>() / n).sqrt();

        let std_dev_y = (points.iter().map(|p| (p.y - mean_y).powi(2)).sum::<f64>() / n).sqrt();

        // 应用标准化
        for point in points.iter_mut() {
            point.x = (point.x - mean_x) / std_dev_x.max(1e-6);
            point.y = (point.y - mean_y) / std_dev_y.max(1e-6);
        }
    }
}

// 轨迹距离计算
fn trajectory_distance(t1: &Trajectory, t2: &Trajectory) -> f64 {
    if t1.points.len() != t2.points.len() {
        return f64::INFINITY;
    }

    t1.points
        .iter()
        .zip(t2.points.iter())
        .map(|(p1, p2)| (p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2))
        .sum::<f64>()
        .sqrt()
}

// 主聚类函数
pub fn do_cluster(
    trajectories: &mut [TrajectoryClusterWrapper],
    sample_count: usize,
    eps: f64,
    min_pts: usize,
) {
    // 1. 重采样
    let mut resampled: Vec<_> = trajectories
        .iter_mut()
        .map(|w| {
            let new_points = resample_trajectory(&w.trajectory.points, sample_count);
            TrajectoryClusterWrapper {
                trajectory: Trajectory {
                    id: w.trajectory.id,
                    points: new_points,
                },
                cluster_id: None,
                is_classified: false,
            }
        })
        .collect();

    // 2. 标准化
    standardize_trajectories(&mut resampled);

    // 3. 执行聚类
    let mut cluster_id = 0;

    for i in 0..resampled.len() {
        if resampled[i].is_classified {
            continue;
        }

        let neighbors: Vec<usize> = resampled
            .iter()
            .enumerate()
            .filter(|&(j, _)| j != i)
            .filter(|(_, t)| trajectory_distance(&resampled[i].trajectory, &t.trajectory) <= eps)
            .map(|(j, _)| j)
            .collect();

        if neighbors.len() >= min_pts {
            let mut queue = VecDeque::new();

            // 分配聚类ID
            cluster_id += 1;
            resampled[i].cluster_id = Some(cluster_id);
            resampled[i].is_classified = true;

            // 初始化队列
            for &n in &neighbors {
                if !resampled[n].is_classified {
                    resampled[n].cluster_id = Some(cluster_id);
                    resampled[n].is_classified = true;
                    queue.push_back(n);
                }
            }

            // 扩展聚类
            while let Some(current) = queue.pop_front() {
                let current_neighbors: Vec<usize> = resampled
                    .iter()
                    .enumerate()
                    .filter(|&(j, _)| j != current)
                    .filter(|(_, t)| {
                        trajectory_distance(&resampled[current].trajectory, &t.trajectory) <= eps
                    })
                    .map(|(j, _)| j)
                    .collect();

                if current_neighbors.len() >= min_pts {
                    for n in current_neighbors {
                        if !resampled[n].is_classified {
                            resampled[n].cluster_id = Some(cluster_id);
                            resampled[n].is_classified = true;
                            queue.push_back(n);
                        }
                    }
                }
            }
        } else {
            resampled[i].cluster_id = None;
        }
    }

    // 同步聚类结果到原始数据
    for (orig, resampled) in trajectories.iter_mut().zip(resampled.iter()) {
        orig.cluster_id = resampled.cluster_id;
        orig.is_classified = resampled.is_classified;
    }

    println!("Generated {} clusters", cluster_id);
}
