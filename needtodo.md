## interaval_x interval_y 
* 反复绘制，数值不确定
* auto_limit

## render_axis
* 调整
定义一个render_grid(来绘制网格，这是必要的)
而关于axis的绘制，这应该看xmin和ymin的值来确定
只有当他们其中一个的值小于等于0时，才绘制axis

## draw
> [!NOTE]
> utils::linspace 实现的非常简单,如果直接把它生成的值作为下标，会导致坐标轴上的点产生偏移

> [!IMPORTANT]
> 目前set_data_prototype 生成的x数值存在问题，可能导致图像无法在坐标轴上显示
> draw_curve会不断进入auto_limit, 目前还不知道原因
> 原先以为是多图像的问题，但好像不是

> [!NOTE]
> 目前Axes对Drawable的引用使用了Rc,可以考虑更换到&dyn Drawble,在库中处理好生命周期,把结构体中的数据 Refcell去掉 

