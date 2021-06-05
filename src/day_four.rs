use std::{clone::Clone, cmp::Ord, cmp::PartialOrd, collections::HashMap, usize, vec};
use regex::Regex;

use crate::utils::read_lines;

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
struct Day {
    year: usize,
    month: usize,
    day: usize
}
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Date {
    day: Day,
    hour: usize,
    minute: usize
}
#[derive(Clone)]
struct GuardSchedule {
    guard_id: usize,
    days: HashMap<Day, Vec<(usize, usize)>>
}

enum GuardEvent {
    Begin {
        date: Date,
        guard_id: usize
    },
    FallAsleep {
        date: Date
    },
    WakeUp {
        date: Date
    }
}

impl Date {
    fn from_string(string: &str) -> Option<Date> {
        let regex = Regex::new(r"\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\]").ok()?;
        let capture = regex.captures(string)?;
        let year = capture[1].parse::<usize>().ok()?;
        let month = capture[2].parse::<usize>().ok()?;
        let day = capture[3].parse::<usize>().ok()?;
        let hour = capture[4].parse::<usize>().ok()?;
        let minute = capture[5].parse::<usize>().ok()?;
        let day = Day {
            year,
            month,
            day
        };
        Some(
            Date {
                day,
                hour, 
                minute
            }
        )
    }
}

impl GuardEvent {
    fn from_string(string: &str) -> Option<GuardEvent> {
        let regex_begin_shift = Regex::new(r"(.*) Guard #(\d+) begins shift").ok()?;
        let regex_falls_asleep = Regex::new(r"(.*) falls asleep").ok()?;
        let regex_wakes_up = Regex::new(r"(.*) wakes up").ok()?;
        if let Some(captures) = regex_begin_shift.captures(string) {
            let date = Date::from_string(&captures[1])?;
            let guard_id = captures[2].parse::<usize>().ok()?;
            Some(
                GuardEvent::Begin {
                    date,
                    guard_id
                }
            )
        } else if let Some(captures) = regex_falls_asleep.captures(string) {
            let date = Date::from_string(&captures[1])?;
            Some(
                GuardEvent::FallAsleep {
                    date
                }
            )
        } else if let Some(captures) = regex_wakes_up.captures(string) {
            let date = Date::from_string(&captures[1])?;
            Some(
                GuardEvent::WakeUp {
                    date
                }
            )
        } else {
            None
        }
    }

    fn get_date(&self) -> &Date {
        match self {
            GuardEvent::Begin {date, guard_id: _} => date,
            GuardEvent::FallAsleep {date} => date,
            GuardEvent::WakeUp {date} => date
        } 
    }
}

impl GuardSchedule {
    fn get_minutes_asleep(&self) -> usize {
        let mut res: usize = 0;
        for (_, intervals) in &self.days {
            for &(start, end) in intervals {
                res += end - start + 1;
            }
        }
        res
    }
}

fn get_guards_events(input: &Vec<String>) -> Vec<GuardEvent> {
    let mut guard_events: Vec<_> = input.iter().filter_map(|string| GuardEvent::from_string(string))
        .collect();
    guard_events.sort_by(|first_event, second_event| first_event.get_date().cmp(second_event.get_date()));
    guard_events
}

fn get_guards_schedules(guards_events: &Vec<GuardEvent>) -> HashMap<usize, GuardSchedule> {
    let mut guards_schedules: HashMap<usize, GuardSchedule> = HashMap::new();
    let mut current_guard_id: usize = 0;
    let mut first_asleep_minute: usize = 0;
    for guard_event in guards_events {
        match guard_event {
            GuardEvent::Begin {date: _, guard_id} => current_guard_id = *guard_id,
            GuardEvent::FallAsleep {date} => first_asleep_minute = date.minute,
            GuardEvent::WakeUp {date} => {
                let wake_minute = date.minute;
                let interval = (first_asleep_minute, wake_minute - 1);
                match guards_schedules.get_mut(&current_guard_id) {
                    Some(guard_schedule) => {
                        match guard_schedule.days.get_mut(&date.day) {
                            Some(intervals) => intervals.push(interval),
                            None => { guard_schedule.days.insert(date.day.clone(), vec![interval]); }
                        }
                    }
                    None => {
                        let mut days: HashMap<Day, Vec<(usize, usize)>> = HashMap::new();
                        days.insert(date.day.clone(), vec![interval]);
                        guards_schedules.insert(
                            current_guard_id,
                            GuardSchedule {
                                guard_id: current_guard_id,
                                days
                            }
                        );
                    }
                }
            }
        }
    }
    guards_schedules
}

fn get_guard_with_max_sleep_time<'a>(guards_schedules: &'a Vec<GuardSchedule>) -> &'a GuardSchedule {
    guards_schedules.iter().max_by(|first, second| first.get_minutes_asleep().cmp(&second.get_minutes_asleep()))
        .unwrap()
}

fn get_guard_most_asleep_minute(guard_schedule: &GuardSchedule) -> usize {
    let mut minutes: [usize; 60] = [0; 60];
    for (_, intervals) in &guard_schedule.days {
        for &(start, end) in intervals {
            for index in start..=end {
                minutes[index] += 1;
            }
        }
    }
    minutes.iter().enumerate()
        .max_by(|(_, first) , (_, second)| first.cmp(second))
        .map(|(index, _)| index)
        .unwrap()
}

fn get_guard_most_asleep_minute_with_frequency(guard_schedule: &GuardSchedule) -> (usize, usize) {
    let mut minutes: [usize; 60] = [0; 60];
    for (_, intervals) in &guard_schedule.days {
        for &(start, end) in intervals {
            for index in start..=end {
                minutes[index] += 1;
            }
        }
    }
    minutes.iter().enumerate()
        .max_by(|(_, first) , (_, second)| first.cmp(second))
        .map(|(index, &frequency)| (index, frequency))
        .unwrap()
}

pub fn solve_part_one() {
    let input = read_lines("day_four.txt");
    let guards_events = get_guards_events(&input);
    let guards_schedules: Vec<_> = get_guards_schedules(&guards_events).values().cloned().collect();
    let max_sleep_guard_schedule = get_guard_with_max_sleep_time(&guards_schedules);
    let max_minute = get_guard_most_asleep_minute(max_sleep_guard_schedule);
    let answer = max_sleep_guard_schedule.guard_id * max_minute;
    println!("{}", answer);
}

pub fn solve_part_two() {
    let input = read_lines("day_four.txt");
    let guards_events = get_guards_events(&input);
    let guards_schedules: Vec<_> = get_guards_schedules(&guards_events).values().cloned().collect();
    let answer = guards_schedules.iter()
        .map(|guard_schedule| (guard_schedule.guard_id, get_guard_most_asleep_minute_with_frequency(guard_schedule)))
        .max_by_key(|(_, (_, frequency))| *frequency)
        .map(|(guard_id, (minute, _))| guard_id * minute)
        .unwrap();
    println!("{}", answer);
}