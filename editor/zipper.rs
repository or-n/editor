#[derive(Debug)]
pub struct WentDown<Item, Kid> {
    item: Item,
    before: Vec<Kid>,
    after: Vec<Kid>,
}

#[derive(Debug)]
pub struct Zipper<Item, Kid> {
    pub node: Node<Item, Kid>,
    pub went: Vec<WentDown<Item, Kid>>,
}

#[derive(Debug, Clone)]
pub struct Node<Item, Kid> {
    pub item: Item,
    pub kids: Vec<Kid>,
}

impl<Item, Kid> Zipper<Item, Kid>
where
    Item: Copy,
    Kid: Clone + TryFrom<Node<Item, Kid>> + TryInto<Node<Item, Kid>>,
{
    pub fn up(&mut self) -> Option<()> {
        let tree = Kid::try_from(self.node.clone()).ok()?;
        let last = self.went.pop()?;
        self.node.kids = last.before;
        self.node.kids.push(tree);
        self.node.kids.extend(last.after);
        self.node.item = last.item;
        Some(())
    }

    pub fn down(&mut self, i: usize) -> Option<()> {
        if i >= self.node.kids.len() {
            return None;
        }
        self.went.push(WentDown {
            item: self.node.item,
            before: self.node.kids[0..i].to_vec(),
            after: self.node.kids[i + 1..].to_vec(),
        });
        self.node = self.node.kids[i].clone().try_into().ok()?;
        Some(())
    }
}
