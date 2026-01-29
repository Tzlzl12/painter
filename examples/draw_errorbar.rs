use painter::{
  Config, Figure,
  primitive::{ErrorBar, ErrorBarType},
};

fn main() {
  let mut figure = Figure::new(Config::default());

  // 创建error bar实例
  let mut error_bar = ErrorBar::new("实验组对比".to_string());

  // 方法1: 使用set_data传入原始数据，自动计算平均值和范围
  // 组A: 控制组 - 5个重复测量值
  let control_group: [f32; 5] = [4.8, 5.1, 4.9, 5.2, 5.0];
  error_bar.set_data(&control_group);

  // 组B: 实验组1 - 5个重复测量值
  let treatment1: [f32; 5] = [6.8, 7.2, 6.5, 7.5, 7.0];
  error_bar.set_data(&treatment1);

  // 组C: 实验组2 - 5个重复测量值
  let treatment2: [f32; 5] = [5.5, 6.2, 5.8, 6.5, 6.0];
  error_bar.set_data(&treatment2);

  // 方法2: 使用set_data_prototype直接指定平均值和范围
  // 组D: 实验组3 - 直接指定统计结果
  error_bar.set_data_prototype(8.5, 7.5, 9.5); // 平均值8.5，范围7.5-9.5

  // 组E: 实验组4 - 直接指定统计结果
  error_bar.set_data_prototype(9.0, 8.0, 10.0); // 平均值9.0，范围8.0-10.0

  // 组F: 实验组5 - 直接指定统计结果（误差范围较小）
  error_bar.set_data_prototype(7.0, 6.8, 7.2); // 平均值7.0，范围6.8-7.2

  error_bar.set_type(ErrorBarType::BaseOnY);
  // 添加error bar到图形
  let ax = figure.nth(0).unwrap();
  ax.add(Box::new(error_bar));
  ax.set_strategy(painter::ScaleStrategy::Stretch);

  // 显示图形
  figure.show();
}
