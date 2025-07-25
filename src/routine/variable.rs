use std::time::Duration;

pub type RoutineVariableTime = RangeValue<Duration>;
pub type RoutineVariablePosition = (i32, i32);
pub type RoutineVariableFilterList = Vec<RoutineVariableFilter>;

#[derive(Clone, Debug, Default)]
pub struct RoutineVariableFilter {
    pub text: String,
    pub depth: u32,
    pub index: Option<usize>,
}

impl RoutineVariableTime {
    pub const LOOP_INTERVAL_TIME_DEFAULT: RoutineVariableTime =
        Self::from_range(LOOP_INTERVAL_TIME);
    pub const MOUSE_MOVE_TIME_DEFAULT: RoutineVariableTime = Self::from_range(MOUSE_MOVE_TIME);
    pub const FIND_TIME_DEFAULT: RoutineVariableTime = Self::from_range(FIND_TIME_TIME);

    const fn from_range(value_range: ValueRange<Duration>) -> Self {
        Self {
            value: value_range.default,
            range: value_range,
        }
    }

    pub fn as_millis_value(&self) -> u64 {
        self.value.as_millis() as u64
    }

    pub fn to_second_string(&self) -> String {
        let millis = self.as_millis_value();
        let second = millis as f32 / 1000.;
        format!("{:.2}s", second).to_string()
    }
}

const LOOP_INTERVAL_TIME: ValueRange<Duration> = ValueRange::<Duration> {
    default: Duration::from_millis(10),
    min: Duration::from_millis(10),
    max: Duration::from_millis(10),
};

const MOUSE_MOVE_TIME: ValueRange<Duration> = ValueRange::<Duration> {
    default: Duration::from_millis(10),
    min: Duration::from_millis(10),
    max: Duration::from_millis(10),
};

const FIND_TIME_TIME: ValueRange<Duration> = ValueRange::<Duration> {
    default: Duration::from_millis(10),
    min: Duration::from_millis(10),
    max: Duration::from_millis(10),
};

#[derive(Clone, Debug, Default)]
pub struct RangeValue<T> {
    pub value: T,
    pub range: ValueRange<T>,
}

#[derive(Clone, Debug, Default)]
pub struct ValueRange<T> {
    pub default: T,
    pub min: T,
    pub max: T,
}
