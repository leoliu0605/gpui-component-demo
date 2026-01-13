use chrono::{Datelike, Days, Duration, Local, NaiveDate, Utc, Weekday};
use gpui::*;
use gpui_component::*;
use gpui_component::{
    calendar::Date,
    date_picker::{DatePicker, DatePickerEvent, DatePickerState, DateRangePreset},
};

use crate::models::{ComponentMeta, subtitle};

pub struct DatePickerComponentView;

impl ComponentMeta for DatePickerComponentView {
    const DESCRIPTION: &'static str = "A flexible date picker component with calendar interface that supports single date selection, \ndate range selection, custom date formatting, disabled dates, and preset ranges.";
    const LINK: &'static str =
        "https://longbridge.github.io/gpui-component/docs/components/date-picker";
}

impl Render for DatePickerComponentView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .w_full()
            .max_w_96()
            .child(subtitle("Basic Date Picker"))
            .child(self.basic_date_picker(_window, _cx))
            .child(subtitle("With Initial Date"))
            .child(self.with_initial_date(_window, _cx))
            .child(subtitle("Date Range Picker"))
            .child(self.date_range_picker(_window, _cx))
            .child(subtitle("With Custom Date Format"))
            .child(self.with_custom_date_format(_window, _cx))
            .child(subtitle("With Placeholder"))
            .child(self.with_placeholder(_window, _cx))
            .child(subtitle("Cleanable Date Picker"))
            .child(self.cleanable_date_picker(_window, _cx))
            .child(subtitle("Different Sizes"))
            .child(self.different_sizes(_window, _cx))
            .child(subtitle("Disabled State"))
            .child(self.disabled_state(_window, _cx))
            .child(subtitle("Custom Appearance"))
            .child(self.custom_appearance(_window, _cx))
            .child(subtitle("Disabled Weekends"))
            .child(self.disabled_weekends(_window, _cx))
            .child(subtitle("Disabled Date Range"))
            .child(self.disabled_date_range(_window, _cx))
            .child(subtitle("Disabled Date Interval"))
            .child(self.disabled_date_interval(_window, _cx))
            .child(subtitle("Custom Disabled Dates"))
            .child(self.custom_disabled_dates(_window, _cx))
            .child(subtitle("Single Date Presets"))
            .child(self.single_date_presets(_window, _cx))
            .child(subtitle("Date Range Presets"))
            .child(self.date_range_presets(_window, _cx))
            .child(subtitle("Handle Date Selection Events"))
            .child(self.handle_date_selection_events(_window, _cx))
            .child(subtitle("Multiple Months Display"))
            .child(self.multiple_months_display(_window, _cx))
            .child(subtitle("Business Days Only"))
            .child(self.business_days_only(_window, _cx))
            .child(subtitle("Date Range with Max Duration"))
            .child(self.date_range_with_max_duration(_window, _cx))
            .child(subtitle("Quarter Presets"))
            .child(self.quarter_presets(_window, _cx))
            .child(subtitle("Event Date Picker"))
            .child(self.event_date_picker(_window, _cx))
            .child(subtitle("Booking System Date Range"))
            .child(self.booking_system_date_range(_window, _cx))
            .child(subtitle("Financial Period Selector"))
            .child(self.financial_period_selector(_window, _cx))
    }
}

impl DatePickerComponentView {
    /// Example code for the Date Picker component

    fn basic_date_picker(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let date_picker = cx.new(|cx| DatePickerState::new(window, cx));

        DatePicker::new(&date_picker).into_any_element()
    }

    fn with_initial_date(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let date_picker = cx.new(|cx| {
            let mut picker = DatePickerState::new(window, cx);
            picker.set_date(Local::now().naive_local().date(), window, cx);
            picker
        });

        DatePicker::new(&date_picker).into_any_element()
    }

    fn date_range_picker(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        v_flex()
            .gap_4()
            .child({
                // Range mode picker
                let range_picker = cx.new(|cx| DatePickerState::range(window, cx));

                DatePicker::new(&range_picker).number_of_months(2) // Show 2 months for easier range selection
            })
            .child({
                // With initial range
                let range_picker = cx.new(|cx| {
                    let now = Local::now().naive_local().date();
                    let mut picker = DatePickerState::new(window, cx);
                    picker.set_date(
                        (now, now.checked_add_days(Days::new(7)).unwrap()),
                        window,
                        cx,
                    );
                    picker
                });

                DatePicker::new(&range_picker).number_of_months(2)
            })
            .into_any_element()
    }

    fn with_custom_date_format(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let date_picker = cx.new(|cx| {
            DatePickerState::new(window, cx).date_format("%Y-%m-%d") // ISO format
        });

        // Other format examples:
        // "%m/%d/%Y" -> 12/25/2023
        // "%B %d, %Y" -> December 25, 2023
        // "%d %b %Y" -> 25 Dec 2023

        DatePicker::new(&date_picker).into_any_element()
    }

    fn with_placeholder(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let date_picker = cx.new(|cx| DatePickerState::new(window, cx));

        DatePicker::new(&date_picker)
            .placeholder("Select a date...")
            .into_any_element()
    }

    fn cleanable_date_picker(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let date_picker = cx.new(|cx| DatePickerState::new(window, cx));

        DatePicker::new(&date_picker)
            .cleanable(true) // Show clear button when date is selected
            .into_any_element()
    }

    fn different_sizes(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let date_picker = cx.new(|cx| DatePickerState::new(window, cx));

        v_flex()
            .gap_4()
            .child(DatePicker::new(&date_picker).large())
            .child(
                DatePicker::new(&date_picker), // medium (default)
            )
            .child(DatePicker::new(&date_picker).small())
            .into_any_element()
    }

    fn disabled_state(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let date_picker = cx.new(|cx| DatePickerState::new(window, cx));

        DatePicker::new(&date_picker)
            .disabled(true)
            .into_any_element()
    }

    fn custom_appearance(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let date_picker = cx.new(|cx| DatePickerState::new(window, cx));

        v_flex()
            .gap_4()
            .child({
                // Without default styling
                DatePicker::new(&date_picker).appearance(false)
            })
            .child({
                // Use in custom container
                div()
                    .border_b_2()
                    .px_6()
                    .py_3()
                    .border_color(cx.theme().border)
                    .bg(cx.theme().secondary)
                    .child(DatePicker::new(&date_picker).appearance(false))
            })
            .into_any_element()
    }

    fn disabled_weekends(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let date_picker = cx.new(|cx| {
            DatePickerState::new(window, cx).disabled_matcher(vec![0, 6]) // Sunday=0, Saturday=6
        });

        DatePicker::new(&date_picker).into_any_element()
    }

    fn disabled_date_range(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let now = Local::now().naive_local().date();

        let date_picker = cx.new(|cx| {
            DatePickerState::new(window, cx).disabled_matcher(calendar::Matcher::range(
                Some(now),
                now.checked_add_days(Days::new(7)),
            ))
        });

        DatePicker::new(&date_picker).into_any_element()
    }

    fn disabled_date_interval(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let now = Local::now().naive_local().date();
        let date_picker = cx.new(|cx| {
            DatePickerState::new(window, cx).disabled_matcher(calendar::Matcher::interval(
                Some(now),
                now.checked_add_days(Days::new(5)),
            ))
        });

        DatePicker::new(&date_picker).into_any_element()
    }

    fn custom_disabled_dates(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        v_flex()
            .gap_4()
            .child({
                // Disable first 5 days of each month
                let date_picker = cx.new(|cx| {
                    DatePickerState::new(window, cx)
                        .disabled_matcher(calendar::Matcher::custom(|date| date.day0() < 5))
                });
                DatePicker::new(&date_picker)
            })
            .child({
                // Disable all Mondays
                let date_picker = cx.new(|cx| {
                    DatePickerState::new(window, cx).disabled_matcher(calendar::Matcher::custom(
                        |date| date.weekday() == chrono::Weekday::Mon,
                    ))
                });

                DatePicker::new(&date_picker)
            })
            .into_any_element()
    }

    fn single_date_presets(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let date_picker = cx.new(|cx| DatePickerState::new(window, cx));
        let presets = vec![
            DateRangePreset::single(
                "Yesterday",
                (Utc::now() - Duration::days(1)).naive_local().date(),
            ),
            DateRangePreset::single(
                "Last Week",
                (Utc::now() - Duration::weeks(1)).naive_local().date(),
            ),
            DateRangePreset::single(
                "Last Month",
                (Utc::now() - Duration::days(30)).naive_local().date(),
            ),
        ];

        DatePicker::new(&date_picker)
            .presets(presets)
            .into_any_element()
    }

    fn date_range_presets(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let date_picker = cx.new(|cx| DatePickerState::range(window, cx));
        let range_presets = vec![
            DateRangePreset::range(
                "Last 7 Days",
                (Utc::now() - Duration::days(7)).naive_local().date(),
                Utc::now().naive_local().date(),
            ),
            DateRangePreset::range(
                "Last 30 Days",
                (Utc::now() - Duration::days(30)).naive_local().date(),
                Utc::now().naive_local().date(),
            ),
            DateRangePreset::range(
                "Last 90 Days",
                (Utc::now() - Duration::days(90)).naive_local().date(),
                Utc::now().naive_local().date(),
            ),
        ];

        DatePicker::new(&date_picker)
            .number_of_months(2)
            .presets(range_presets)
            .into_any_element()
    }

    fn handle_date_selection_events(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let date_picker = cx.new(|cx| DatePickerState::new(window, cx));

        let _ = cx.subscribe(&date_picker, |_view, _, event, _| match event {
            DatePickerEvent::Change(date) => match date {
                Date::Single(Some(selected_date)) => {
                    println!("Single date selected: {}", selected_date);
                }
                Date::Range(Some(start), Some(end)) => {
                    println!("Date range selected: {} to {}", start, end);
                }
                Date::Range(Some(start), None) => {
                    println!("Range start selected: {}", start);
                }
                _ => {
                    println!("Date cleared");
                }
            },
        });

        DatePicker::new(&date_picker).into_any_element()
    }

    fn multiple_months_display(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let date_picker = cx.new(|cx| DatePickerState::new(window, cx));

        v_flex()
            .gap_4()
            .child({
                // Show 2 months side by side (useful for date ranges)
                DatePicker::new(&date_picker).number_of_months(2)
            })
            .child({
                // Show 3 months
                DatePicker::new(&date_picker).number_of_months(3)
            })
            .into_any_element()
    }

    fn business_days_only(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let business_days_picker = cx.new(|cx| {
            DatePickerState::new(window, cx).disabled_matcher(calendar::Matcher::custom(|date| {
                matches!(date.weekday(), Weekday::Sat | Weekday::Sun)
            }))
        });

        DatePicker::new(&business_days_picker)
            .placeholder("Select business day")
            .into_any_element()
    }

    fn date_range_with_max_duration(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let max_30_days_picker = cx.new(|cx| DatePickerState::range(window, cx));

        // cx.subscribe(&max_30_days_picker, |view, picker, event, _| {
        //     match event {
        //         DatePickerEvent::Change(Date::Range(Some(start), Some(end))) => {
        //             let duration = end.signed_duration_since(*start).num_days();
        //             if duration > 30 {
        //                 // Reset to start date only if range exceeds 30 days
        //                 picker.update(cx, |state, cx| {
        //                     state.set_date(Date::Range(Some(*start), None), window, cx);
        //                 });
        //             }
        //         }
        //         _ => {}
        //     }
        // });

        DatePicker::new(&max_30_days_picker)
            .number_of_months(2)
            .placeholder("Select up to 30 days")
            .into_any_element()
    }

    fn quarter_presets(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let date_picker = cx.new(|cx| DatePickerState::range(window, cx));
        let year = Local::now().year();
        let quarterly_presets = vec![
            DateRangePreset::range("Q1", quarter_start(year, 1), quarter_end(year, 1)),
            DateRangePreset::range("Q2", quarter_start(year, 2), quarter_end(year, 2)),
            DateRangePreset::range("Q3", quarter_start(year, 3), quarter_end(year, 3)),
            DateRangePreset::range("Q4", quarter_start(year, 4), quarter_end(year, 4)),
        ];

        DatePicker::new(&date_picker)
            .presets(quarterly_presets)
            .number_of_months(2)
            .into_any_element()
    }

    fn event_date_picker(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let event_date = cx.new(|cx| {
            let picker = DatePickerState::new(window, cx)
                .date_format("%B %d, %Y")
                .disabled_matcher(gpui_component::calendar::Matcher::custom(|date| {
                    // Disable past dates
                    *date < Local::now().naive_local().date()
                }));
            picker
        });

        DatePicker::new(&event_date)
            .placeholder("Choose event date")
            .cleanable(true)
            .into_any_element()
    }

    fn booking_system_date_range(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let booking_range = cx.new(|cx| DatePickerState::range(window, cx));

        let today = Local::now().naive_local().date();
        let this_month_start = NaiveDate::from_ymd_opt(today.year(), today.month(), 1).unwrap();
        let this_month_end = quarter_end(today.year(), ((today.month() as u32 - 1) / 3) + 1);
        let days_to_weekend = (5 - today.weekday().number_from_monday() as i64 + 7) % 7;
        let this_weekend_start = if days_to_weekend == 0 {
            today
        } else {
            today + Duration::days(days_to_weekend)
        };
        let this_weekend_end = this_weekend_start + Duration::days(1);
        let next_week_start = this_weekend_end + Duration::days(1);
        let next_week_end = next_week_start + Duration::days(6);

        let booking_presets = vec![
            DateRangePreset::range("This Weekend", this_weekend_start, this_weekend_end),
            DateRangePreset::range("Next Week", next_week_start, next_week_end),
            DateRangePreset::range("This Month", this_month_start, this_month_end),
        ];

        DatePicker::new(&booking_range)
            .number_of_months(2)
            .presets(booking_presets)
            .placeholder("Select check-in and check-out dates")
            .into_any_element()
    }

    fn financial_period_selector(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let financial_period =
            cx.new(|cx| DatePickerState::range(window, cx).date_format("%Y-%m-%d"));

        let year = Local::now().year();
        let quarterly_presets = vec![
            DateRangePreset::range("Q1", quarter_start(year, 1), quarter_end(year, 1)),
            DateRangePreset::range("Q2", quarter_start(year, 2), quarter_end(year, 2)),
            DateRangePreset::range("Q3", quarter_start(year, 3), quarter_end(year, 3)),
            DateRangePreset::range("Q4", quarter_start(year, 4), quarter_end(year, 4)),
        ];

        DatePicker::new(&financial_period)
            .number_of_months(3)
            .presets(quarterly_presets)
            .placeholder("Select reporting period")
            .into_any_element()
    }
}

fn quarter_start(year: i32, quarter: u32) -> NaiveDate {
    let month = (quarter - 1) * 3 + 1;
    NaiveDate::from_ymd_opt(year, month, 1).unwrap()
}

fn quarter_end(year: i32, quarter: u32) -> NaiveDate {
    let month = quarter * 3;
    let start = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    NaiveDate::from_ymd_opt(year, month, start.num_days_in_month() as u32).unwrap()
}
