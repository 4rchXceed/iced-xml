use std::collections::HashMap;

// Hash types that aren't natively hashable
#[derive(Debug, Clone)]
pub struct HashableF32(f32);

impl HashableF32 {
    pub fn new(value: f32) -> Self {
        HashableF32(value)
    }

    pub fn value(&self) -> f32 {
        return self.0;
    }
}

impl std::hash::Hash for HashableF32 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

#[derive(Debug, Clone)]
pub struct HashableGridTarget(iced::widget::pane_grid::Target);

impl HashableGridTarget {
    pub fn new(value: iced::widget::pane_grid::Target) -> Self {
        HashableGridTarget(value)
    }
    pub fn value(&self) -> iced::widget::pane_grid::Target {
        return self.0;
    }
    fn hash_edge(edge: &iced::widget::pane_grid::Edge) -> u64 {
        match edge {
            iced::widget::pane_grid::Edge::Bottom => 0,
            iced::widget::pane_grid::Edge::Left => 1,
            iced::widget::pane_grid::Edge::Right => 2,
            iced::widget::pane_grid::Edge::Top => 3,
        }
    }
}

impl std::hash::Hash for HashableGridTarget {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let first = match self.0 {
            iced::widget::pane_grid::Target::Edge(edge) => {
                let edge_hash = HashableGridTarget::hash_edge(&edge);
                1 + edge_hash
            }
            iced::widget::pane_grid::Target::Pane(pane, region) => {
                pane.hash(state);
                let region_hash = match region {
                    iced::widget::pane_grid::Region::Center => 0,
                    iced::widget::pane_grid::Region::Edge(edge) => {
                        HashableGridTarget::hash_edge(&edge)
                    }
                };
                4 + region_hash
            }
        };
        first.hash(state);
    }
}

#[derive(Debug, Clone)]
pub struct HashableHashMap<K, V>(pub HashMap<K, V>);

impl<K, V> HashableHashMap<K, V> {
    pub fn new(map: HashMap<K, V>) -> Self {
        Self(map)
    }
    pub fn value(&self) -> &HashMap<K, V> {
        return &self.0;
    }
}

impl<K: std::hash::Hash, V: std::hash::Hash> std::hash::Hash for HashableHashMap<K, V> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let mut items: Vec<(&K, &V)> = self.0.iter().collect();
        items.sort_by(|a, b| a.0.hash(state).cmp(&b.0.hash(state)));
        for (k, v) in items {
            k.hash(state);
            v.hash(state);
        }
    }
}

#[derive(Debug, Clone, Hash)]
pub struct ScrollState {
    pub scroll_left: HashableF32,
    pub scroll_top: HashableF32,
    pub scroll_width: HashableF32,
    pub scroll_height: HashableF32,
}
