use std::collections::HashSet;

use chrono::{Datelike, Days, Local, NaiveDate, Weekday};
use gpui::*;
use gpui_component::calendar::{Calendar, CalendarEvent, CalendarState, Date, Matcher};
use gpui_component::*;

use crate::models::ComponentRenderer;

pub struct CalendarComponent;

impl ComponentRenderer for CalendarComponent {
    fn show(&self, _window: &mut Window, _cx: &mut App) -> AnyElement {
        v_flex()
            .gap_2()
            .w_full()
            .max_w_96()
            .child(self.add_subtitle("Basic Calendar"))
            .child(self.basic_calendar(_window, _cx))
            .child(self.add_subtitle("Calendar with Initial Date"))
            .child(self.calendar_with_initial_date(_window, _cx))
            .child(self.add_subtitle("Date Range Calendar"))
            .child(self.date_range_calendar(_window, _cx))
            .child(self.add_subtitle("Multiple Months Display"))
            .child(self.multiple_months_display(_window, _cx))
            .child(self.add_subtitle("Calendar Sizes"))
            .child(self.calendar_sizes(_window, _cx))
            .child(self.add_subtitle("Disabled Weekends"))
            .child(self.disabled_weekends(_window, _cx))
            .child(self.add_subtitle("Disabled Specific Weekdays"))
            .child(self.disabled_specific_weekdays(_window, _cx))
            .child(self.add_subtitle("Disabled Date Range"))
            .child(self.disabled_date_range(_window, _cx))
            .child(self.add_subtitle("Disabled Date Interval"))
            .child(self.disabled_date_interval(_window, _cx))
            .child(self.add_subtitle("Custom Disabled Dates"))
            .child(self.custom_disabled_dates(_window, _cx))
            .child(self.add_subtitle("Custom Year Range"))
            .child(self.custom_year_range(_window, _cx))
            .child(self.add_subtitle("Handle Selection Events"))
            .child(self.handle_selection_events(_window, _cx))
            .child(self.add_subtitle("Business Days Only Calendar"))
            .child(self.business_days_only_calendar(_window, _cx))
            .child(self.add_subtitle("Holiday Calendar"))
            .child(self.holiday_calendar(_window, _cx))
            .child(self.add_subtitle("Multi-Month Range Selector"))
            .child(self.multi_month_range_selector(_window, _cx))
            .child(self.add_subtitle("Quarterly View Calendar"))
            .child(self.quarterly_view_calendar(_window, _cx))
            .child(self.add_subtitle("Custom Styling"))
            .child(self.custom_styling(_window, _cx))
            .child(self.add_subtitle("Event Planning Calendar"))
            .child(self.event_planning_calendar(_window, _cx))
            .child(self.add_subtitle("Vacation Booking Calendar"))
            .child(self.vacation_booking_calendar(_window, _cx))
            .child(self.add_subtitle("Report Date Range Selector"))
            .child(self.report_date_range_selector(_window, _cx))
            .child(self.add_subtitle("Availability Calendar"))
            .child(self.availability_calendar(_window, _cx))
            .into_any_element()
    }

    fn description(&self) -> &'static str {
        "A standalone calendar component that provides a rich interface for date selection and navigation. \nThe Calendar component supports single date selection, date range selection, multiple month views, \ncustom disabled dates, and comprehensive keyboard navigation."
    }

    fn link(&self) -> &'static str {
        "https://longbridge.github.io/gpui-component/docs/components/calendar"
    }
}

impl CalendarComponent {
    /// Example code for the Calendar component

    fn basic_calendar(&self, window: &mut Window, cx: &mut App) -> AnyElement {
        h_flex()
            .gap_4()
            .child({
                let state = cx.new(|cx| CalendarState::new(window, cx));
                Calendar::new(&state)
            })
            .into_any_element()
    }

    fn calendar_with_initial_date(&self, window: &mut Window, cx: &mut App) -> AnyElement {
        h_flex()
            .gap_4()
            .child({
                let state = cx.new(|cx| {
                    let mut state = CalendarState::new(window, cx);
                    state.set_date(Local::now().naive_local().date(), window, cx);
                    state
                });

                Calendar::new(&state)
            })
            .into_any_element()
    }

    fn date_range_calendar(&self, window: &mut Window, cx: &mut App) -> AnyElement {
        h_flex()
            .gap_4()
            .child({
                let state = cx.new(|cx| {
                    let mut state = CalendarState::new(window, cx);
                    let now = Local::now().naive_local().date();
                    state.set_date(
                        Date::Range(Some(now), now.checked_add_days(Days::new(7))),
                        window,
                        cx,
                    );
                    state
                });

                Calendar::new(&state)
            })
            .into_any_element()
    }

    fn multiple_months_display(&self, window: &mut Window, cx: &mut App) -> AnyElement {
        let state = cx.new(|cx| CalendarState::new(window, cx));

        v_flex()
            .gap_4()
            .child({
                h_flex().gap_4().child(
                    // Show 2 months side by side
                    Calendar::new(&state).number_of_months(2),
                )
            })
            .child({
                h_flex().gap_4().child(
                    // Show 3 months
                    Calendar::new(&state).number_of_months(3),
                )
            })
            .into_any_element()
    }

    fn calendar_sizes(&self, window: &mut Window, cx: &mut App) -> AnyElement {
        let state = cx.new(|cx| CalendarState::new(window, cx));

        v_flex()
            .gap_4()
            .child(h_flex().gap_4().child(Calendar::new(&state).large()))
            .child(h_flex().gap_4().child(
                Calendar::new(&state), // medium (default)
            ))
            .child(h_flex().gap_4().child(Calendar::new(&state).small()))
            .into_any_element()
    }

    fn disabled_weekends(&self, window: &mut Window, cx: &mut App) -> AnyElement {
        h_flex()
            .gap_4()
            .child({
                let state = cx.new(|cx| {
                    CalendarState::new(window, cx).disabled_matcher(vec![0, 6]) // Sunday=0, Saturday=6
                });

                Calendar::new(&state)
            })
            .into_any_element()
    }

    fn disabled_specific_weekdays(&self, window: &mut Window, cx: &mut App) -> AnyElement {
        h_flex()
            .gap_4()
            .child({
                // Disable Sundays, Wednesdays, and Saturdays
                let state =
                    cx.new(|cx| CalendarState::new(window, cx).disabled_matcher(vec![0, 3, 6]));

                Calendar::new(&state)
            })
            .into_any_element()
    }

    fn disabled_date_range(&self, window: &mut Window, cx: &mut App) -> AnyElement {
        h_flex()
            .gap_4()
            .child({
                let now = Local::now().naive_local().date();

                let state = cx.new(|cx| {
                    CalendarState::new(window, cx).disabled_matcher(Matcher::range(
                        Some(now),
                        now.checked_add_days(Days::new(7)),
                    ))
                });

                Calendar::new(&state)
            })
            .into_any_element()
    }

    fn disabled_date_interval(&self, window: &mut Window, cx: &mut App) -> AnyElement {
        h_flex()
            .gap_4()
            .child({
                let now = Local::now().naive_local().date();

                // Disable dates outside the interval (before/after specified dates)
                let state = cx.new(|cx| {
                    CalendarState::new(window, cx).disabled_matcher(Matcher::interval(
                        Some(now.checked_sub_days(Days::new(30)).unwrap()),
                        now.checked_add_days(Days::new(30)),
                    ))
                });

                Calendar::new(&state)
            })
            .into_any_element()
    }

    fn custom_disabled_dates(&self, window: &mut Window, cx: &mut App) -> AnyElement {
        v_flex()
            .gap_4()
            .child(h_flex().gap_4().child({
                // Disable first 5 days of each month
                let state = cx.new(|cx| {
                    CalendarState::new(window, cx).disabled_matcher(Matcher::custom(|date| {
                        date.day0() < 5 // day0() returns 0-based day
                    }))
                });

                Calendar::new(&state)
            }))
            .child(h_flex().gap_4().child({
                // Disable all Mondays
                let state = cx.new(|cx| {
                    CalendarState::new(window, cx).disabled_matcher(Matcher::custom(|date| {
                        date.weekday() == chrono::Weekday::Mon
                    }))
                });

                Calendar::new(&state)
            }))
            .child(h_flex().gap_4().child({
                // Disable past dates
                let state = cx.new(|cx| {
                    CalendarState::new(window, cx).disabled_matcher(Matcher::custom(|date| {
                        *date < Local::now().naive_local().date()
                    }))
                });

                Calendar::new(&state)
            }))
            .into_any_element()
    }

    fn custom_year_range(&self, window: &mut Window, cx: &mut App) -> AnyElement {
        h_flex()
            .gap_4()
            .child({
                let state = cx.new(|cx| {
                    CalendarState::new(window, cx).year_range((2020, 2030)) // Limit to specific year range
                });

                Calendar::new(&state)
            })
            .into_any_element()
    }

    fn handle_selection_events(&self, window: &mut Window, cx: &mut App) -> AnyElement {
        h_flex()
            .gap_4()
            .child({
                let state = cx.new(|cx| CalendarState::new(window, cx));

                let _ = cx.subscribe(&state, |_, event, _| match event {
                    CalendarEvent::Selected(date) => match date {
                        Date::Single(Some(selected_date)) => {
                            println!("Date selected: {}", selected_date);
                        }
                        Date::Range(Some(start), Some(end)) => {
                            println!("Range selected: {} to {}", start, end);
                        }
                        Date::Range(Some(start), None) => {
                            println!("Range start: {}", start);
                        }
                        _ => {
                            println!("Selection cleared");
                        }
                    },
                });

                Calendar::new(&state)
            })
            .into_any_element()
    }

    fn business_days_only_calendar(&self, window: &mut Window, cx: &mut App) -> AnyElement {
        h_flex()
            .gap_4()
            .child({
                let state = cx.new(|cx| {
                    CalendarState::new(window, cx).disabled_matcher(Matcher::custom(|date| {
                        matches!(date.weekday(), Weekday::Sat | Weekday::Sun)
                    }))
                });

                Calendar::new(&state)
            })
            .into_any_element()
    }

    fn holiday_calendar(&self, window: &mut Window, cx: &mut App) -> AnyElement {
        h_flex()
            .gap_4()
            .child({
                // Define holidays
                let holidays: HashSet<NaiveDate> = [
                    NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(), // New Year
                    NaiveDate::from_ymd_opt(2024, 7, 4).unwrap(), // Independence Day
                    NaiveDate::from_ymd_opt(2024, 12, 25).unwrap(), // Christmas
                ]
                .into_iter()
                .collect();

                let state = cx.new(|cx| {
                    CalendarState::new(window, cx)
                        .disabled_matcher(Matcher::custom(move |date| holidays.contains(date)))
                });

                Calendar::new(&state)
            })
            .into_any_element()
    }

    fn multi_month_range_selector(&self, window: &mut Window, cx: &mut App) -> AnyElement {
        h_flex()
            .gap_4()
            .child({
                let state = cx.new(|cx| {
                    let mut state = CalendarState::new(window, cx);
                    state.set_date(Date::Range(None, None), window, cx); // Range mode
                    state
                });

                Calendar::new(&state).number_of_months(3) // Show 3 months for easier range selection
            })
            .into_any_element()
    }

    fn quarterly_view_calendar(&self, window: &mut Window, cx: &mut App) -> AnyElement {
        h_flex()
            .gap_4()
            .child({
                let state = cx.new(|cx| CalendarState::new(window, cx));

                // Update to show current quarter's months
                Calendar::new(&state).number_of_months(3)
            })
            .into_any_element()
    }

    fn custom_styling(&self, window: &mut Window, cx: &mut App) -> AnyElement {
        h_flex()
            .gap_4()
            .child({
                let calendar = cx.new(|cx| CalendarState::new(window, cx));

                Calendar::new(&calendar)
                    .p_4() // Custom padding
                    .bg(cx.theme().secondary) // Custom background
                    .border_2() // Custom border
                    .border_color(cx.theme().primary) // Custom border color
                    .rounded(px(12.)) // Custom border radius
                    .w(px(400.)) // Custom width
                    .h(px(350.)) // Custom height
            })
            .into_any_element()
    }

    fn event_planning_calendar(&self, window: &mut Window, cx: &mut App) -> AnyElement {
        h_flex()
            .gap_4()
            .child({
                let event_calendar = cx.new(|cx| {
                    let mut state = CalendarState::new(window, cx);
                    // Disable past dates and weekends
                    state = state.disabled_matcher(Matcher::custom(|date| {
                        let now = Local::now().naive_local().date();
                        *date < now || matches!(date.weekday(), Weekday::Sat | Weekday::Sun)
                    }));
                    state
                });

                Calendar::new(&event_calendar).large() // Easier to see and interact with
            })
            .into_any_element()
    }

    fn vacation_booking_calendar(&self, window: &mut Window, cx: &mut App) -> AnyElement {
        h_flex()
            .gap_4()
            .child({
                let vacation_calendar = cx.new(|cx| {
                    let mut state = CalendarState::new(window, cx);
                    state.set_date(Date::Range(None, None), window, cx); // Range mode
                    state
                });

                Calendar::new(&vacation_calendar).number_of_months(2) // Show 2 months for range selection
            })
            .into_any_element()
    }

    fn report_date_range_selector(&self, window: &mut Window, cx: &mut App) -> AnyElement {
        h_flex()
            .gap_4()
            .child({
                let report_calendar = cx.new(|cx| {
                    let mut state = CalendarState::new(window, cx).year_range((2020, 2025)); // Limit to business years

                    state.set_date(Date::Range(None, None), window, cx);
                    state
                });

                Calendar::new(&report_calendar).number_of_months(3).small() // Compact for dashboard use
            })
            .into_any_element()
    }

    fn availability_calendar(&self, window: &mut Window, cx: &mut App) -> AnyElement {
        h_flex()
            .gap_4()
            .child({
                // Example unavailable dates - in a real app, these would come from a database or API
                let unavailable_dates: HashSet<NaiveDate> = [
                    Local::now()
                        .naive_local()
                        .date()
                        .checked_add_days(Days::new(2))
                        .unwrap(),
                    Local::now()
                        .naive_local()
                        .date()
                        .checked_add_days(Days::new(5))
                        .unwrap(),
                    Local::now()
                        .naive_local()
                        .date()
                        .checked_add_days(Days::new(10))
                        .unwrap(),
                ]
                .into_iter()
                .collect();

                let availability_calendar = cx.new(|cx| {
                    CalendarState::new(window, cx).disabled_matcher(Matcher::custom(move |date| {
                        unavailable_dates.contains(date)
                    }))
                });

                Calendar::new(&availability_calendar).number_of_months(2)
            })
            .into_any_element()
    }
}
