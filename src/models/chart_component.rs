use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::chart::{AreaChart, BarChart, CandlestickChart, LineChart, PieChart};
use gpui_component::*;

use crate::models::{ComponentMeta, subtitle};

pub struct ChartComponentView {
    date_data: Vec<DateValue>,
    chart_type: ChartType,
    time_range: TimeRange,

    timestamp_data: Vec<TimestampValue>,
    max_points: usize,
    counter: usize,
}

impl ChartComponentView {
    pub fn new() -> Self {
        Self {
            date_data: vec![
                DateValue {
                    date: "2023-01".to_string(),
                    value: 200.0,
                },
                DateValue {
                    date: "2023-02".to_string(),
                    value: 250.0,
                },
                DateValue {
                    date: "2023-03".to_string(),
                    value: 220.0,
                },
            ],
            chart_type: ChartType::Line,
            time_range: TimeRange {
                start: "2023-01".to_string(),
                end: "2023-12".to_string(),
            },
            timestamp_data: vec![
                TimestampValue {
                    timestamp: "T0".to_string(),
                    value: 100.0,
                },
                TimestampValue {
                    timestamp: "T1".to_string(),
                    value: 150.0,
                },
                TimestampValue {
                    timestamp: "T2".to_string(),
                    value: 120.0,
                },
            ],
            max_points: 20,
            counter: 3,
        }
    }

    fn filtered_data(&self) -> Vec<DateValue> {
        self.date_data
            .iter()
            .filter(|d| self.time_range.contains(&d.date))
            .cloned()
            .collect()
    }

    fn add_data_point(&mut self, point: TimestampValue) {
        self.timestamp_data.push(point);
        if self.timestamp_data.len() > self.max_points {
            self.timestamp_data.remove(0); // Remove oldest point
        }
    }

    fn start_auto_update(&mut self, cx: &mut Context<Self>) {
        cx.spawn(async |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            loop {
                cx.background_executor()
                    .timer(std::time::Duration::from_millis(800))
                    .await;

                let should_continue = this
                    .update(cx, |view, cx| {
                        // Generate a new data point with wave-like variation
                        let last_value =
                            view.timestamp_data.last().map(|p| p.value).unwrap_or(100.0);
                        let delta = (view.counter as f64 * 0.5).sin() * 20.0
                            + (view.counter as f64 * 0.3).cos() * 10.0;
                        let new_value = (last_value + delta).clamp(50.0, 200.0);

                        let point = TimestampValue {
                            timestamp: format!("{}", view.counter),
                            value: new_value,
                        };
                        view.add_data_point(point);
                        view.counter += 1;
                        cx.notify();
                        true
                    })
                    .ok()
                    .unwrap_or(false);

                if !should_continue {
                    break;
                }
            }
        })
        .detach();
    }
}

impl ComponentMeta for ChartComponentView {
    const DESCRIPTION: &'static str = "A comprehensive charting library providing Line, Bar, Area, Pie, and Candlestick charts for data visualization. \nThe charts feature smooth animations, customizable styling, tooltips, legends, \nand automatic theming that adapts to your application's theme.";
    const LINK: &'static str = "https://longbridge.github.io/gpui-component/docs/components/chart";
}

impl Render for ChartComponentView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .w_full()
            .max_w_96()
            .child(subtitle("Basic Line Chart"))
            .child(self.basic_line_chart())
            .child(subtitle("Line Chart Variants"))
            .child(self.line_chart_variants(_cx))
            .child(subtitle("Tick Control"))
            .child(self.tick_control())
            .child(subtitle("Basic Bar Chart"))
            .child(self.basic_bar_chart())
            .child(subtitle("Bar Chart Customization"))
            .child(self.bar_chart_customization(_cx))
            .child(subtitle("Basic Area Chart"))
            .child(self.basic_area_chart())
            .child(subtitle("Stacked Area Charts"))
            .child(self.stacked_area_charts(_cx))
            .child(subtitle("Area Chart Styling"))
            .child(self.area_chart_styling(_cx))
            .child(subtitle("Basic Pie Chart"))
            .child(self.basic_pie_chart())
            .child(subtitle("Donut Chart"))
            .child(self.donut_chart())
            .child(subtitle("Pie Chart Customization"))
            .child(self.pie_chart_customization())
            .child(subtitle("Basic Candlestick Chart"))
            .child(self.basic_candlestick_chart())
            .child(subtitle("Candlestick Chart Customization"))
            .child(self.candlestick_chart_customization())
            .child(subtitle("Container Setup"))
            .child(self.container_setup(_cx))
            .child(subtitle("Theme Integration"))
            .child(self.theme_integration(_cx))
            .child(subtitle("Sales Dashboard"))
            .child(self.sales_dashboard(_cx))
            .child(subtitle("Multi-Series Time Chart"))
            .child(self.multi_series_time_chart(_cx))
            .child(subtitle("Financial Chart"))
            .child(self.financial_chart(_cx))
            .child(subtitle("Color Schemes"))
            .child(self.color_schemes(_cx))
            .child(subtitle("Responsive Design"))
            .child(self.responsive_design())
            .child(subtitle("Large Datasets"))
            .child(self.large_datasets())
            .child(subtitle("Memory Optimization"))
            .child(self.memory_optimization())
            .child(subtitle("With State Management"))
            .child(self.with_state_management())
            .child(subtitle("Real-time Updates"))
            .child(self.real_time_updates(_cx))
    }
}

impl ChartComponentView {
    /// Example code for the Chart component

    fn basic_line_chart(&self) -> AnyElement {
        let data = vec![
            DataPoint {
                x: "Jan".to_string(),
                y: 100.0,
            },
            DataPoint {
                x: "Feb".to_string(),
                y: 150.0,
            },
            DataPoint {
                x: "Mar".to_string(),
                y: 120.0,
            },
        ];

        LineChart::new(data)
            .x(|d| d.x.clone())
            .y(|d| d.y)
            .into_any_element()
    }

    fn line_chart_variants(&self, cx: &mut Context<Self>) -> AnyElement {
        let data = vec![
            MonthValue {
                month: "Jan".to_string(),
                value: 100.0,
            },
            MonthValue {
                month: "Feb".to_string(),
                value: 150.0,
            },
            MonthValue {
                month: "Mar".to_string(),
                value: 120.0,
            },
        ];

        v_flex()
            .gap_4()
            .child(
                // Basic curved line (default)
                LineChart::new(data.clone())
                    .x(|d| d.month.clone())
                    .y(|d| d.value),
            )
            .child(
                // Linear interpolation
                LineChart::new(data.clone())
                    .x(|d| d.month.clone())
                    .y(|d| d.value)
                    .linear(),
            )
            .child(
                // Step after interpolation
                LineChart::new(data.clone())
                    .x(|d| d.month.clone())
                    .y(|d| d.value)
                    .step_after(),
            )
            .child(
                // With dots at data points
                LineChart::new(data.clone())
                    .x(|d| d.month.clone())
                    .y(|d| d.value)
                    .dot(),
            )
            .child(
                // Custom stroke color
                LineChart::new(data.clone())
                    .x(|d| d.month.clone())
                    .y(|d| d.value)
                    .stroke(cx.theme().success),
            )
            .into_any_element()
    }

    fn tick_control(&self) -> AnyElement {
        let data = vec![
            MonthValue {
                month: "Jan".to_string(),
                value: 100.0,
            },
            MonthValue {
                month: "Feb".to_string(),
                value: 150.0,
            },
            MonthValue {
                month: "Mar".to_string(),
                value: 120.0,
            },
        ];

        v_flex()
            .gap_4()
            .child(
                // Show every tick
                LineChart::new(data.clone())
                    .x(|d| d.month.clone())
                    .y(|d| d.value)
                    .tick_margin(1),
            )
            .child(
                // Show every 2nd tick
                LineChart::new(data.clone())
                    .x(|d| d.month.clone())
                    .y(|d| d.value)
                    .tick_margin(2),
            )
            .into_any_element()
    }

    fn basic_bar_chart(&self) -> AnyElement {
        let data = vec![
            CategoryValue {
                category: "A".to_string(),
                value: 30.0,
                color: Hsla::default(),
                category_index: 0,
            },
            CategoryValue {
                category: "B".to_string(),
                value: 80.0,
                color: Hsla::default(),
                category_index: 1,
            },
            CategoryValue {
                category: "C".to_string(),
                value: 45.0,
                color: Hsla::default(),
                category_index: 2,
            },
        ];

        BarChart::new(data)
            .x(|d| d.category.clone())
            .y(|d| d.value)
            .into_any_element()
    }

    fn bar_chart_customization(&self, cx: &mut Context<Self>) -> AnyElement {
        let data = vec![
            CategoryValue {
                category: "A".to_string(),
                value: 30.0,
                color: cx.theme().chart_1,
                category_index: 0,
            },
            CategoryValue {
                category: "B".to_string(),
                value: 80.0,
                color: cx.theme().chart_2,
                category_index: 1,
            },
            CategoryValue {
                category: "C".to_string(),
                value: 45.0,
                color: cx.theme().chart_3,
                category_index: 2,
            },
        ];

        v_flex()
            .gap_4()
            .child(
                // Custom fill colors
                BarChart::new(data.clone())
                    .x(|d| d.category.clone())
                    .y(|d| d.value)
                    .fill(|d| d.color),
            )
            .child(
                // With labels on bars
                BarChart::new(data.clone())
                    .x(|d| d.category.clone())
                    .y(|d| d.value)
                    .label(|d| format!("{}", d.value)),
            )
            .child(
                // Custom tick spacing
                BarChart::new(data.clone())
                    .x(|d| d.category.clone())
                    .y(|d| d.value)
                    .tick_margin(2),
            )
            .into_any_element()
    }

    fn basic_area_chart(&self) -> AnyElement {
        let data = vec![
            TimeValue {
                time: "2023-01".to_string(),
                value: 200.0,
            },
            TimeValue {
                time: "2023-02".to_string(),
                value: 250.0,
            },
            TimeValue {
                time: "2023-03".to_string(),
                value: 220.0,
            },
        ];

        AreaChart::new(data)
            .x(|d| d.time.clone())
            .y(|d| d.value)
            .into_any_element()
    }

    fn stacked_area_charts(&self, cx: &mut Context<Self>) -> AnyElement {
        let data = vec![
            DailyDevice {
                date: "2023-01".to_string(),
                desktop: 120.0,
                mobile: 80.0,
            },
            DailyDevice {
                date: "2023-02".to_string(),
                desktop: 150.0,
                mobile: 100.0,
            },
            DailyDevice {
                date: "2023-03".to_string(),
                desktop: 130.0,
                mobile: 90.0,
            },
        ];

        // Multi-series area chart
        AreaChart::new(data)
            .x(|d| d.date.clone())
            .y(|d| d.desktop) // First series
            .stroke(cx.theme().chart_1)
            .fill(cx.theme().chart_1.opacity(0.4))
            .y(|d| d.mobile) // Second series
            .stroke(cx.theme().chart_2)
            .fill(cx.theme().chart_2.opacity(0.4))
            .into_any_element()
    }

    fn area_chart_styling(&self, cx: &mut Context<Self>) -> AnyElement {
        let data = vec![
            MonthValue {
                month: "Jan".to_string(),
                value: 100.0,
            },
            MonthValue {
                month: "Feb".to_string(),
                value: 150.0,
            },
            MonthValue {
                month: "Mar".to_string(),
                value: 120.0,
            },
        ];

        v_flex()
            .gap_4()
            .child(
                // With gradient fill
                AreaChart::new(data.clone())
                    .x(|d| d.month.clone())
                    .y(|d| d.value)
                    .fill(linear_gradient(
                        0.,
                        linear_color_stop(cx.theme().chart_1.opacity(0.4), 1.),
                        linear_color_stop(cx.theme().background.opacity(0.3), 0.),
                    )),
            )
            .child(
                // Different interpolation styles
                AreaChart::new(data.clone())
                    .x(|d| d.month.clone())
                    .y(|d| d.value)
                    .linear(), // or .step_after()
            )
            .into_any_element()
    }

    fn basic_pie_chart(&self) -> AnyElement {
        let data = vec![
            AmountValue {
                amount: 40.0,
                color: Hsla::default(),
            },
            AmountValue {
                amount: 30.0,
                color: Hsla::default(),
            },
            AmountValue {
                amount: 30.0,
                color: Hsla::default(),
            },
        ];

        PieChart::new(data)
            .value(|d| d.amount as f32)
            .outer_radius(100.)
            .into_any_element()
    }

    fn donut_chart(&self) -> AnyElement {
        let data = vec![
            AmountValue {
                amount: 40.0,
                color: Hsla::default(),
            },
            AmountValue {
                amount: 30.0,
                color: Hsla::default(),
            },
            AmountValue {
                amount: 30.0,
                color: Hsla::default(),
            },
        ];

        PieChart::new(data)
            .value(|d| d.amount as f32)
            .outer_radius(100.)
            .inner_radius(60.) // Creates donut effect
            .into_any_element()
    }

    fn pie_chart_customization(&self) -> AnyElement {
        let data = vec![
            AmountValue {
                amount: 40.0,
                color: Hsla::default(),
            },
            AmountValue {
                amount: 30.0,
                color: Hsla::default(),
            },
            AmountValue {
                amount: 30.0,
                color: Hsla::default(),
            },
        ];

        v_flex()
            .gap_4()
            .child(
                // Custom colors
                PieChart::new(data.clone())
                    .value(|d| d.amount as f32)
                    .outer_radius(100.)
                    .color(|d| d.color),
            )
            .child(
                // With padding between slices
                PieChart::new(data.clone())
                    .value(|d| d.amount as f32)
                    .outer_radius(100.)
                    .inner_radius(60.)
                    .pad_angle(4. / 100.), // 4% padding
            )
            .into_any_element()
    }

    fn basic_candlestick_chart(&self) -> AnyElement {
        let data = vec![
            StockPrice {
                date: "Jan".to_string(),
                open: 100.0,
                high: 110.0,
                low: 95.0,
                close: 105.0,
                volume: 1500,
            },
            StockPrice {
                date: "Feb".to_string(),
                open: 105.0,
                high: 115.0,
                low: 100.0,
                close: 112.0,
                volume: 2000,
            },
            StockPrice {
                date: "Mar".to_string(),
                open: 112.0,
                high: 120.0,
                low: 108.0,
                close: 115.0,
                volume: 1800,
            },
        ];

        CandlestickChart::new(data)
            .x(|d| d.date.clone())
            .open(|d| d.open)
            .high(|d| d.high)
            .low(|d| d.low)
            .close(|d| d.close)
            .into_any_element()
    }

    fn candlestick_chart_customization(&self) -> AnyElement {
        let data = vec![
            StockPrice {
                date: "Jan".to_string(),
                open: 100.0,
                high: 110.0,
                low: 95.0,
                close: 105.0,
                volume: 1500,
            },
            StockPrice {
                date: "Feb".to_string(),
                open: 105.0,
                high: 115.0,
                low: 100.0,
                close: 112.0,
                volume: 2000,
            },
            StockPrice {
                date: "Mar".to_string(),
                open: 112.0,
                high: 120.0,
                low: 108.0,
                close: 115.0,
                volume: 1800,
            },
        ];

        v_flex()
            .gap_4()
            .child(
                // Adjust body width ratio (default: 0.6)
                CandlestickChart::new(data.clone())
                    .x(|d| d.date.clone())
                    .open(|d| d.open)
                    .high(|d| d.high)
                    .low(|d| d.low)
                    .close(|d| d.close)
                    .body_width_ratio(0.4), // Narrower bodies
            )
            .child(
                // Custom tick spacing
                CandlestickChart::new(data.clone())
                    .x(|d| d.date.clone())
                    .open(|d| d.open)
                    .high(|d| d.high)
                    .low(|d| d.low)
                    .close(|d| d.close)
                    .tick_margin(2), // Show every 2nd tick
            )
            .into_any_element()
    }

    fn container_setup(&self, cx: &mut Context<Self>) -> AnyElement {
        let title = "Container Title";
        let chart = LineChart::new(vec![
            MonthValue {
                month: "Jan".to_string(),
                value: 100.0,
            },
            MonthValue {
                month: "Feb".to_string(),
                value: 150.0,
            },
            MonthValue {
                month: "Mar".to_string(),
                value: 120.0,
            },
        ])
        .x(|d| d.month.clone())
        .y(|d| d.value);
        let center = true;

        chart_container(title, chart, center, cx).into_any_element()
    }

    fn theme_integration(&self, cx: &mut Context<Self>) -> AnyElement {
        let data = vec![
            DateValue {
                date: "2023-01".to_string(),
                value: 200.0,
            },
            DateValue {
                date: "2023-02".to_string(),
                value: 250.0,
            },
            DateValue {
                date: "2023-03".to_string(),
                value: 220.0,
            },
        ];

        // Charts automatically use theme colors
        LineChart::new(data)
            .x(|d| d.date.clone())
            .y(|d| d.value)
            .stroke(cx.theme().chart_1) // Uses theme chart colors
            .into_any_element()
    }

    fn sales_dashboard(&self, cx: &mut Context<Self>) -> AnyElement {
        let chart_1 = cx.theme().chart_1;
        let chart_2 = cx.theme().chart_2;
        let chart_3 = cx.theme().chart_3;
        let chart_4 = cx.theme().chart_4;
        let chart_5 = cx.theme().chart_5;

        let data = vec![
            SalesData {
                month: "January".to_string(),
                revenue: 50000.0,
                profit: 15000.0,
                region: "North".to_string(),
            },
            SalesData {
                month: "February".to_string(),
                revenue: 60000.0,
                profit: 20000.0,
                region: "South".to_string(),
            },
            SalesData {
                month: "March".to_string(),
                revenue: 55000.0,
                profit: 18000.0,
                region: "East".to_string(),
            },
            SalesData {
                month: "April".to_string(),
                revenue: 70000.0,
                profit: 25000.0,
                region: "West".to_string(),
            },
        ];

        v_flex()
            .gap_4()
            .child(
                h_flex()
                    .gap_4()
                    .child(chart_container(
                        "Monthly Revenue",
                        LineChart::new(data.clone())
                            .x(|d| d.month.clone())
                            .y(|d| d.revenue)
                            .stroke(chart_1)
                            .dot(),
                        false,
                        cx,
                    ))
                    .child(chart_container(
                        "Profit Breakdown",
                        PieChart::new(data.clone())
                            .value(|d| d.profit as f32)
                            .outer_radius(80.)
                            .color(move |d| match d.region.as_str() {
                                "North" => chart_1,
                                "South" => chart_2,
                                "East" => chart_3,
                                "West" => chart_4,
                                _ => chart_5,
                            }),
                        true,
                        cx,
                    )),
            )
            .child(chart_container(
                "Regional Performance",
                BarChart::new(data)
                    .x(|d| d.region.clone())
                    .y(|d| d.revenue)
                    .fill(move |d| match d.region.as_str() {
                        "North" => chart_1,
                        "South" => chart_2,
                        "East" => chart_3,
                        "West" => chart_4,
                        _ => chart_5,
                    })
                    .label(|d| format!("${:.0}k", d.revenue / 1000.)),
                false,
                cx,
            ))
            .into_any_element()
    }

    fn multi_series_time_chart(&self, cx: &mut Context<Self>) -> AnyElement {
        let data = vec![
            DeviceUsage {
                date: "2023-01".to_string(),
                desktop: 120.0,
                mobile: 80.0,
                tablet: 40.0,
            },
            DeviceUsage {
                date: "2023-02".to_string(),
                desktop: 150.0,
                mobile: 100.0,
                tablet: 50.0,
            },
            DeviceUsage {
                date: "2023-03".to_string(),
                desktop: 130.0,
                mobile: 90.0,
                tablet: 60.0,
            },
        ];

        chart_container(
            "Device Usage Over Time",
            AreaChart::new(data)
                .x(|d| d.date.clone())
                .y(|d| d.desktop)
                .stroke(cx.theme().chart_1)
                .fill(linear_gradient(
                    0.,
                    linear_color_stop(cx.theme().chart_1.opacity(0.4), 1.),
                    linear_color_stop(cx.theme().background.opacity(0.3), 0.),
                ))
                .y(|d| d.mobile)
                .stroke(cx.theme().chart_2)
                .fill(linear_gradient(
                    0.,
                    linear_color_stop(cx.theme().chart_2.opacity(0.4), 1.),
                    linear_color_stop(cx.theme().background.opacity(0.3), 0.),
                ))
                .y(|d| d.tablet)
                .stroke(cx.theme().chart_3)
                .fill(linear_gradient(
                    0.,
                    linear_color_stop(cx.theme().chart_3.opacity(0.4), 1.),
                    linear_color_stop(cx.theme().background.opacity(0.3), 0.),
                ))
                .tick_margin(3),
            false,
            cx,
        )
        .into_any_element()
    }

    fn financial_chart(&self, cx: &mut Context<Self>) -> AnyElement {
        let chart_1 = cx.theme().chart_1;
        let muted_foreground = cx.theme().muted_foreground;

        let ohlc_data = vec![
            StockOHLC {
                date: "2023-01".to_string(),
                open: 100.0,
                high: 110.0,
                low: 95.0,
                close: 105.0,
            },
            StockOHLC {
                date: "2023-02".to_string(),
                open: 105.0,
                high: 115.0,
                low: 100.0,
                close: 112.0,
            },
            StockOHLC {
                date: "2023-03".to_string(),
                open: 112.0,
                high: 120.0,
                low: 108.0,
                close: 115.0,
            },
        ];

        let price_data = vec![
            StockData {
                date: "2023-01".to_string(),
                price: 105.0,
                volume: 1500000,
            },
            StockData {
                date: "2023-02".to_string(),
                price: 112.0,
                volume: 2000000,
            },
            StockData {
                date: "2023-03".to_string(),
                price: 115.0,
                volume: 1800000,
            },
        ];

        v_flex()
            .gap_4()
            .child(chart_container(
                "Stock Price - Candlestick",
                CandlestickChart::new(ohlc_data.clone())
                    .x(|d| d.date.clone())
                    .open(|d| d.open)
                    .high(|d| d.high)
                    .low(|d| d.low)
                    .close(|d| d.close)
                    .tick_margin(3),
                false,
                cx,
            ))
            .child(chart_container(
                "Stock Price - Line",
                LineChart::new(price_data.clone())
                    .x(|d| d.date.clone())
                    .y(|d| d.price)
                    .stroke(chart_1)
                    .linear()
                    .tick_margin(5),
                false,
                cx,
            ))
            .child(chart_container(
                "Trading Volume",
                BarChart::new(price_data)
                    .x(|d| d.date.clone())
                    .y(|d| d.volume as f64)
                    .fill(move |d| {
                        if d.volume > 1000000 {
                            chart_1
                        } else {
                            muted_foreground.opacity(0.6)
                        }
                    })
                    .tick_margin(5),
                false,
                cx,
            ))
            .into_any_element()
    }

    fn color_schemes(&self, cx: &mut Context<Self>) -> AnyElement {
        v_flex()
            .gap_4()
            .child({
                let data = vec![
                    DataPoint {
                        x: "Jan".to_string(),
                        y: 100.0,
                    },
                    DataPoint {
                        x: "Feb".to_string(),
                        y: 150.0,
                    },
                    DataPoint {
                        x: "Mar".to_string(),
                        y: 120.0,
                    },
                ];
                // Theme-based colors (recommended)
                LineChart::new(data)
                    .x(|d| d.x.clone())
                    .y(|d| d.y)
                    .stroke(cx.theme().chart_1)
            })
            .child({
                let data = vec![
                    CategoryValue {
                        category: "A".to_string(),
                        value: 30.0,
                        color: Hsla::default(),
                        category_index: 0,
                    },
                    CategoryValue {
                        category: "B".to_string(),
                        value: 80.0,
                        color: Hsla::default(),
                        category_index: 1,
                    },
                    CategoryValue {
                        category: "C".to_string(),
                        value: 45.0,
                        color: Hsla::default(),
                        category_index: 2,
                    },
                ];
                let colors = [
                    cx.theme().success,
                    cx.theme().warning,
                    cx.theme().info,
                    cx.theme().chart_1,
                ];
                // Custom color palette
                BarChart::new(data)
                    .x(|d| d.category.clone())
                    .y(|d| d.value)
                    .fill(move |d| colors[d.category_index % colors.len()])
            })
            .into_any_element()
    }

    fn responsive_design(&self) -> AnyElement {
        // Example of a chart that adapts to container size
        let data = vec![
            DataPoint {
                x: "Jan".to_string(),
                y: 100.0,
            },
            DataPoint {
                x: "Feb".to_string(),
                y: 150.0,
            },
            DataPoint {
                x: "Mar".to_string(),
                y: 120.0,
            },
        ];

        // Container with responsive sizing
        div()
            .flex_1()
            .min_h(px(300.))
            .max_h(px(600.))
            .w_full()
            .child(LineChart::new(data).x(|d| d.x.clone()).y(|d| d.y))
            .into_any_element()
    }

    fn large_datasets(&self) -> AnyElement {
        let mut data = Vec::new();
        for i in 0..1000 {
            data.push(DateValue {
                date: format!("2023-{:03}", i + 1),
                value: (i as f64).sin() * 50.0 + 100.0,
            });
        }
        // For large datasets, consider data sampling
        let sampled_data: Vec<_> = data
            .iter()
            .step_by(5) // Show every 5th point
            .cloned()
            .collect();

        LineChart::new(sampled_data)
            .x(|d| d.date.clone())
            .y(|d| d.value)
            .tick_margin(3) // Reduce tick density
            .into_any_element()
    }

    fn memory_optimization(&self) -> AnyElement {
        let mut data = Vec::new();
        for i in 0..1000 {
            data.push(DateValue {
                date: format!("2023-{:05}", i + 1),
                value: (i as f64).cos() * 30.0 + 70.0,
            });
        }
        // Use efficient data accessors
        LineChart::new(data)
            .x(|d| d.date.clone()) // Clone only when necessary
            .y(|d| d.value) // Direct field access
            .into_any_element()
    }

    fn with_state_management(&self) -> AnyElement {
        match self.chart_type {
            ChartType::Line => LineChart::new(self.filtered_data())
                .x(|d| d.date.clone())
                .y(|d| d.value)
                .into_any_element(),
            ChartType::Bar => BarChart::new(self.filtered_data())
                .x(|d| d.date.clone())
                .y(|d| d.value)
                .into_any_element(),
            ChartType::Area => AreaChart::new(self.filtered_data())
                .x(|d| d.date.clone())
                .y(|d| d.value)
                .into_any_element(),
        }
    }

    fn real_time_updates(&mut self, cx: &mut Context<Self>) -> AnyElement {
        // Auto-start the update loop on first render (when counter is still at initial value)
        if self.counter == 3 {
            self.start_auto_update(cx);
        }

        // Simple chart display with auto-updating data
        div()
            .h(px(200.))
            .w_full()
            .child(
                LineChart::new(self.timestamp_data.clone())
                    .x(|d| d.timestamp.clone())
                    .y(|d| d.value)
                    .linear()
                    .dot()
                    .stroke(cx.theme().chart_1),
            )
            .into_any_element()
    }
}

#[derive(Clone)]
struct DataPoint {
    x: String,
    y: f64,
}

#[derive(Clone)]
struct MonthValue {
    month: String,
    value: f64,
}

#[derive(Clone)]
struct CategoryValue {
    category: String,
    value: f64,
    color: Hsla,
    category_index: usize,
}

#[derive(Clone)]
struct TimeValue {
    time: String,
    value: f64,
}

#[derive(Clone)]
struct DailyDevice {
    pub date: String,
    pub desktop: f64,
    pub mobile: f64,
}

#[derive(Clone)]
struct AmountValue {
    amount: f32,
    color: Hsla,
}

#[derive(Clone)]
#[allow(dead_code)]
struct StockPrice {
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
}

#[derive(Clone)]
#[allow(dead_code)]
struct MonthlyDevice {
    pub month: String,
    pub desktop: f64,
    pub color_alpha: f32,
}

#[allow(dead_code)]
impl MonthlyDevice {
    pub fn color(&self, base_color: Hsla) -> Hsla {
        base_color.alpha(self.color_alpha)
    }
}

#[derive(Clone)]
struct DateValue {
    date: String,
    value: f64,
}

#[derive(Clone)]
struct SalesData {
    month: String,
    revenue: f64,
    profit: f64,
    region: String,
}

#[derive(Clone)]
struct DeviceUsage {
    date: String,
    desktop: f64,
    mobile: f64,
    tablet: f64,
}

#[derive(Clone)]
struct StockData {
    date: String,
    price: f64,
    volume: u64,
}

#[derive(Clone)]
struct StockOHLC {
    date: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
}

#[derive(Clone, Copy, PartialEq)]
#[allow(dead_code)]
enum ChartType {
    Line,
    Bar,
    Area,
}

#[derive(Clone)]
struct TimeRange {
    start: String,
    end: String,
}

impl TimeRange {
    fn contains(&self, date: &str) -> bool {
        date >= self.start.as_str() && date <= self.end.as_str()
    }
}

#[derive(Clone)]
struct TimestampValue {
    timestamp: String,
    value: f64,
}

fn chart_container(
    title: &str,
    chart: impl IntoElement,
    center: bool,
    cx: &mut Context<ChartComponentView>,
) -> impl IntoElement {
    v_flex()
        .flex_1()
        .h_full()
        .border_1()
        .border_color(cx.theme().border)
        .rounded_lg()
        .p_4()
        .child(
            div()
                .when(center, |this| this.text_center())
                .font_semibold()
                .child(title.to_string()),
        )
        .child(
            div()
                .when(center, |this| this.text_center())
                .text_color(cx.theme().muted_foreground)
                .text_sm()
                .child("Data period label"),
        )
        .child(div().flex_1().py_4().child(chart))
        .child(
            div()
                .when(center, |this| this.text_center())
                .font_semibold()
                .text_sm()
                .child("Summary statistic"),
        )
        .child(
            div()
                .when(center, |this| this.text_center())
                .text_color(cx.theme().muted_foreground)
                .text_sm()
                .child("Additional context"),
        )
}
