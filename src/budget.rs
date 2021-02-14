use itertools::Itertools;
use crate::duration::Duration;
use crate::models;


#[derive(Debug)]
pub struct Budget {
    pub duration: Duration,
    pub items: Vec<models::Item>,
}

impl Budget {
    pub fn total(&self) -> f32 {
        let days_in_month: f32 = (52_f32/12_f32)*7_f32; // 30.333333

        self.items.iter().fold(0.0, |acc, item| {
            let multiple = match (self.duration, item.duration) {
                (Duration::Day(b_days), Duration::Day(i_days)) => b_days as f32 / i_days as f32,
                (Duration::Day(b_days), Duration::Month(i_months)) => b_days as f32 / (i_months as f32 * days_in_month),
                (Duration::Month(b_months), Duration::Day(i_days)) => b_months as f32 / (i_days as f32 / days_in_month),
                (Duration::Month(b_months), Duration::Month(i_months)) => b_months as f32 / i_months as f32,
            };

            acc + (item.cost * multiple)
        })
    }

    pub fn split_by_currency(&self) -> Vec<(String, Budget)> {
        let grouped_items = self.items
            .clone()
            .into_iter()
            .sorted_by_key(|item| item.currency.clone())
            .group_by(|item| item.currency.clone());

        grouped_items
            .into_iter()
            .map(|(key, group)| {
                (key, Budget { duration: self.duration.clone(), items: group.collect() })
            })
            .collect()
    }
}
