use dbscan::{do_cluster, Trajectory, TrajectoryClusterWrapper};
use plotters::prelude::full_palette::GREY;
use plotters::prelude::*;
use projection::ll_to_wmc;
use reader::read_csv;
use std::collections::HashMap;

fn plot_tracks(data: &HashMap<Option<usize>, Vec<Vec<(f64, f64)>>>) -> Result<(), Box<dyn std::error::Error>> {
    // 1. 创建绘图区域
    let root = BitMapBackend::new("tracks.png", (1920, 1080)).into_drawing_area();
    root.fill(&WHITE)?;

    // 2. 计算坐标范围
    let (min_x, max_x, min_y, max_y) = data.values().flatten().flatten().fold(
        (f64::MAX, f64::MIN, f64::MAX, f64::MIN),
        |(min_x, max_x, min_y, max_y), &(x, y)| {
            (
                min_x.min(x),
                max_x.max(x),
                min_y.min(y),
                max_y.max(y),
            )
        },
    );

    // 3. 创建坐标系
    let mut chart = ChartBuilder::on(&root)
        .caption("Flight Tracks", ("sans-serif", 50).into_font())
        .margin(50)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(
            (min_x - 0.1)..(max_x + 0.1),
            (min_y - 0.1)..(max_y + 0.1),
        )?;

    // 4. 配置坐标轴
    chart
        .configure_mesh()
        .x_desc("Longitude")
        .y_desc("Latitude")
        .axis_desc_style(("sans-serif", 20))
        .draw()?;

    // 5. 定义颜色映射
    let colors = [
        RED, BLUE, GREEN, MAGENTA, CYAN, YELLOW, BLACK
    ];

    // 6. 绘制每条轨迹
    for (track_id, tracks) in data.iter() {
        let color = match track_id {
            Some(n) => colors[n % colors.len()],
            None => GREY, // 未分类轨迹使用灰色
        }.mix(0.7).stroke_width(2);

        for path in tracks.iter() {
            // 绘制路径线段
            chart.draw_series(LineSeries::new(
                path.iter().map(|&(x, y)| (x, y)),
                color.clone(),
            ))?;
        }
    }

    Ok(())
}

fn main() {
    let mut data: Vec<_> =
        // 先读取一下数据
        read_csv("data/ZBAA ADS-B Data 20191018-20191019.csv").unwrap()
            .into_iter()
            // 赋值id
            .enumerate()
            // 转换成聚类算法的航迹
            .map(|(index,x)| Trajectory {
                id: index,
                points: x.data
                    .into_iter()
                    // 投影
                    .map(|x| ll_to_wmc(x.lat as f64, x.long as f64, x.datetime))
                    .collect(),
            })
            // 转换成航迹的聚类包装
            .map(TrajectoryClusterWrapper::new)
            .collect();

    println!("do cluster");

    for i in 30..60 {
        print!("{:.1},", i as f64 * 0.1);
        let mut data_new = data.clone();
        do_cluster(&mut data_new, 100, i as f64 * 0.1, 40);
    }
    // 聚类
    do_cluster(&mut data, 100, 4.2, 40);

    println!("{}", data.len());

    // 导出结果
    let mut result: HashMap<Option<usize>, Vec<TrajectoryClusterWrapper>> = HashMap::new();

    data.into_iter().for_each(|traj| {
        result.entry(traj.cluster_id).or_default();
        result.get_mut(&traj.cluster_id).unwrap().push(traj);
    });

    println!("{:?}", result.keys());

    // 绘图
    let t = result
        .into_iter()
        .map(|(key, value)| {
            (
                key,
                value
                    .into_iter()
                    .map(|p| {
                        p.trajectory
                            .points
                            .into_iter()
                            .map(|p| (p.x, p.y))
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<HashMap<_, _>>();

    plot_tracks(&t).unwrap();
}
