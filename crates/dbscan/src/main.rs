use chrono::{Duration, Utc};
use dbscan::{Point2D, Trajectory, TrajectoryClusterWrapper, do_cluster};

fn main() {
    // 创建测试轨迹数据
    let base_time = Utc::now().naive_local();
    let mut trajs = vec![
        TrajectoryClusterWrapper::new(Trajectory {
            id: 1,
            points: vec![
                Point2D::new(0.0, 0.0, base_time),
                Point2D::new(1.0, 1.0, base_time + Duration::minutes(10)),
                Point2D::new(2.0, 2.0, base_time + Duration::minutes(20)),
            ],
        }),
        TrajectoryClusterWrapper::new(Trajectory {
            id: 2,
            points: vec![
                Point2D::new(0.1, 0.1, base_time),
                Point2D::new(1.1, 1.1, base_time + Duration::minutes(10)),
                Point2D::new(2.1, 2.1, base_time + Duration::minutes(20)),
            ],
        }),
        TrajectoryClusterWrapper::new(Trajectory {
            id: 3,
            points: vec![
                Point2D::new(0.0, 0.0, base_time),
                Point2D::new(1.0, 1.0, base_time + Duration::minutes(10)),
                Point2D::new(2.0, 2.0, base_time + Duration::minutes(20)),
                Point2D::new(4.0, 4.0, base_time + Duration::minutes(30)),
                Point2D::new(8.0, 8.0, base_time + Duration::minutes(40)),
                Point2D::new(16.0, 16.0, base_time + Duration::minutes(50)),
            ],
        }),
        TrajectoryClusterWrapper::new(Trajectory {
            id: 4,
            points: vec![
                Point2D::new(0.1, 0.1, base_time),
                Point2D::new(1.1, 1.1, base_time + Duration::minutes(10)),
                Point2D::new(2.1, 2.1, base_time + Duration::minutes(20)),
                Point2D::new(4.1, 4.1, base_time + Duration::minutes(30)),
                Point2D::new(8.1, 8.1, base_time + Duration::minutes(40)),
                Point2D::new(16.1, 16.1, base_time + Duration::minutes(50)),
            ],
        }),
        TrajectoryClusterWrapper::new(Trajectory {
            id: 5,
            points: vec![
                Point2D::new(0.1, 0.1, base_time),
                Point2D::new(8.1, 2.1, base_time + Duration::minutes(20)),
                Point2D::new(0.0, 0.0, base_time + Duration::minutes(30)),
            ],
        }),
    ];

    // 执行聚类
    do_cluster(&mut trajs, 3, 0.5, 1);

    // 输出结果
    for t in &trajs {
        println!("Trajectory {}: Cluster {:?}", t.trajectory.id, t.cluster_id);
    }
}
