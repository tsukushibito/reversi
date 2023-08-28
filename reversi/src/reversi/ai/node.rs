use crate::Action;

pub trait Node: Sized {
    fn children(&self) -> &[Self];
    fn children_mut(&mut self) -> &mut Vec<Self>;
    fn value(&self) -> &Option<i32>;
    fn value_mut(&mut self) -> &mut Option<i32>;
    fn last_action(&self) -> &Action;

    fn expand(&mut self);

    fn node_count(&self) -> usize {
        self.children()
            .iter()
            .fold(1, |acc, child| acc + child.node_count())
    }

    fn searched_nodes(&self) -> usize {
        self.children()
            .iter()
            .filter(|child| child.value().is_some())
            .fold(1, |acc, child| acc + child.searched_nodes())
    }

    fn candidate(&self) -> Option<Vec<Action>> {
        if self.children().is_empty() {
            None
        } else {
            let mut children = self
                .children()
                .iter()
                .map(|child| (child.value(), child.last_action()))
                .collect::<Vec<_>>();
            children.sort_by(|a, b| b.0.cmp(a.0));
            Some(
                children
                    .iter()
                    .map(|(_, &action)| action)
                    .collect::<Vec<_>>(),
            )
        }
    }
}
