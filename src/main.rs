trait Filter<T> {
    fn is_match(&self, item: &T) -> bool;
}

struct FilterCondition<T> {
    desired_value: T,
}

impl<T: PartialEq> Filter<T> for FilterCondition<T> {
    fn is_match(&self, item: &T) -> bool {
        *item == self.desired_value
    }
}

impl<T, F> Filter<T> for &F
where
    F: Filter<T>,
{
    fn is_match(&self, item: &T) -> bool {
        (*self).is_match(item)
    }
}

fn custom_filter<T, F>(collection: Vec<T>, filter: F) -> Vec<T>
where
    F: Filter<T>,
    T: Clone,
{
    collection
        .into_iter()
        .filter(|item| filter.is_match(item))
        .collect()
}

fn main() {
    let data = vec![10, 20, 30, 40, 50];
    let condition = FilterCondition { desired_value: 30 };

    let filtered_data = custom_filter(data, &condition);

    for item in &filtered_data {
        println!("{}", item);
    }
}
