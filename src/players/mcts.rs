use std::fmt::Display;

use crate::games::GameState;

use super::Player;

pub struct MCTSPlayer;

impl Player for MCTSPlayer
{
    fn choose_move<Game>(&self, game_state: &Game) -> Game::Move
    where
        Game: GameState,
        Game::Move: Display,
    {
        todo!()
    }
}

mod arena_tree
{
    use rand::{seq::SliceRandom, thread_rng};

    use self::arena_vec::{ArenaVec, NodeRef};

    pub struct ArenaTree<T>
    {
        nodes: ArenaVec<Node<T>>,
        root: NodeRef,
    }
    impl<T: Default + From<usize>> ArenaTree<T>
    {
        pub fn new(root_data: T, capacity: usize) -> ArenaTree<T>
        {
            let node = Node {
                parent: None,
                children: vec![],
                data: root_data,
                expanded: false,
            };
            let mut nodes = ArenaVec::new(capacity);
            let root = nodes.insert(node);

            ArenaTree { nodes, root }
        }

        pub fn root(&mut self) -> &mut Node<T>
        {
            self.nodes.get_mut(&self.root)
        }
    }

    pub struct Node<T>
    {
        parent: Option<NodeRef>,
        children: Vec<NodeRef>,
        pub data: T,
        expanded: bool,
    }

    mod arena_vec
    {

        pub struct ArenaVec<T>
        {
            data: Vec<Option<T>>,
        }

        impl<T> ArenaVec<T>
        {
            pub fn data(&self) -> &Vec<Option<T>>
            {
                &self.data
            }
        }

        #[derive(Clone, Copy)]
        pub struct NodeRef(usize);

        impl<T> ArenaVec<T>
        {
            pub fn new(capacity: usize) -> ArenaVec<T>
            {
                ArenaVec {
                    data: (0..capacity).map(|_| None).collect(),
                }
            }

            pub fn insert(&mut self, value: T) -> NodeRef
            {
                let mut index_to_insert = None;
                for i in 0..self.data.len()
                {
                    if self.data[i].is_none()
                    {
                        index_to_insert = Some(i);
                        break;
                    }
                }

                let index_to_insert = match index_to_insert
                {
                    Some(i) => i,
                    None =>
                    {
                        self.data.push(None);
                        self.data.len() - 1
                    },
                };

                self.data[index_to_insert] = Some(value);

                NodeRef(index_to_insert)
            }

            pub fn get(&self, index: &NodeRef) -> &T
            {
                self.data[index.0].as_ref().expect("invalid NodeRef")
            }

            pub fn get_mut(&mut self, index: &NodeRef) -> &mut T
            {
                self.data[index.0].as_mut().expect("invalid NodeRef")
            }
        }
    }
}
