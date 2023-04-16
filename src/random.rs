use rand::{thread_rng, Rng};

#[derive(Debug, Clone)]
pub struct Randomizer<T> {
    pub items: Vec<RItem<T>>,
}

impl<T> Randomizer<T>
where
    T: Clone + std::fmt::Debug + std::cmp::PartialEq,
{
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    #[allow(dead_code)]
    fn get_test() -> Randomizer<String> {
        let mut r = Randomizer::new();
        r.add("Should be MOOOOOOOST POSIBLE".into(), 99.9);
        r.add("Den".into(), 33.3);
        r.add("Vlad".into(), 70.0);
        r.add("Roma".into(), 0.5);
        r.add("Nobody".into(), 50.0);
        r.add("SUPER ULTRA RAREE 0.00001".into(), 0.00001);
        r.add("IT CAN BEEEEEEEEEEEEEEE!!!!!!!!!!!!!!!!!!".into(), 0.0);
        r
    }

    /// [chance] - should be from 0 to 99.9
    pub fn add(&mut self, item: T, chance: f64) {
        if !(0.0..=99.9).contains(&chance) {
            panic!("parameter 'chance' should be in range 0..99.9");
        }
        self.items.push(RItem { item, chance });
    }

    pub fn get_formated_chance(&self, item_obj: &T) -> f64 {
        self.items
            .clone()
            .iter()
            .find(|item| &item.item == item_obj)
            .unwrap()
            .chance
    }

    fn recalc_chances(&mut self) {
        let total_weight = self.items.iter().map(|r| r.chance).sum::<f64>();
        self.items.iter_mut().for_each(|i| {
            let y = 10i32.pow(5) as f64;
            let x = i.chance / total_weight;
            i.chance = ((x * y).round() / y) * 100.0;
        });
    }

    pub fn get_random_item(&mut self) -> T {
        let mut rng = thread_rng();
        // Calculate the total weight of all items
        let total_weight = self.items.iter().map(|r| r.chance).sum::<f64>();

        if total_weight == 0.0 {
            panic!("Items are empty!");
        }

        // Check if the total weight is greater than 100.0
        if total_weight > 100.0 {
            self.recalc_chances();
        }

        // Create a vector of weighted intervals
        let mut intervals = vec![(0.0, &self.items[0].item)];
        let mut last_weight = self.items[0].chance;
        for i in 1..self.items.len() {
            let next_interval = (last_weight, &self.items[i].item);
            intervals.push(next_interval);
            last_weight += self.items[i].chance;
        }

        // Generate a random number and find the corresponding interval
        let random_num = rng.gen_range(0.0..100.0);
        let selected = intervals
            .binary_search_by(|interval| {
                if interval.0 > random_num {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                }
            })
            .unwrap_or_else(|i| i - 1);

        intervals[selected].1.clone()
    }
}

#[derive(Debug, Clone)]
pub struct RItem<T> {
    pub item: T,
    pub chance: f64,
}
