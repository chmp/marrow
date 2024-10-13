// check the layout of the interval types

#[test]
fn interval_layout_day_time() {
    assert_eq!(
        std::mem::size_of::<arrow_array::types::IntervalDayTime>(),
        std::mem::size_of::<marrow::types::DayTimeInterval>(),
    );
    assert_eq!(
        std::mem::align_of::<arrow_array::types::IntervalDayTime>(),
        std::mem::align_of::<marrow::types::DayTimeInterval>(),
    );
    assert_eq!(
        std::mem::offset_of!(arrow_array::types::IntervalDayTime, days),
        std::mem::offset_of!(marrow::types::DayTimeInterval, days),
    );
    assert_eq!(
        std::mem::offset_of!(arrow_array::types::IntervalDayTime, milliseconds),
        std::mem::offset_of!(marrow::types::DayTimeInterval, milliseconds),
    );
}

#[test]
fn interval_layout_month_day_nano() {
    assert_eq!(
        std::mem::size_of::<arrow_array::types::IntervalMonthDayNano>(),
        std::mem::size_of::<marrow::types::MonthDayNanoInterval>(),
    );
    assert_eq!(
        std::mem::align_of::<arrow_array::types::IntervalMonthDayNano>(),
        std::mem::align_of::<marrow::types::MonthDayNanoInterval>(),
    );
    assert_eq!(
        std::mem::offset_of!(arrow_array::types::IntervalMonthDayNano, months),
        std::mem::offset_of!(marrow::types::MonthDayNanoInterval, months),
    );
    assert_eq!(
        std::mem::offset_of!(arrow_array::types::IntervalMonthDayNano, days),
        std::mem::offset_of!(marrow::types::MonthDayNanoInterval, days),
    );
    assert_eq!(
        std::mem::offset_of!(arrow_array::types::IntervalMonthDayNano, nanoseconds),
        std::mem::offset_of!(marrow::types::MonthDayNanoInterval, nanoseconds),
    );
}
