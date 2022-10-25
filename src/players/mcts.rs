use std::{
    f64::consts::{E, SQRT_2},
    fmt::Display,
};

use rand::{seq::SliceRandom, thread_rng};

use crate::games::{GameResult, GameState};

use self::arena_tree::{ArenaTree, NodeRef};

use super::{random::RandomPlayer, GamePlayer};

pub struct MCTSPlayer
{
    iterations: usize,
}

impl MCTSPlayer
{
    pub fn new(iterations: usize) -> MCTSPlayer
    {
        MCTSPlayer { iterations }
    }
}

impl GamePlayer for MCTSPlayer
{
    fn choose_move<Game>(&self, game_state: &Game) -> Game::Move
    where
        Game: GameState,
        Game::Move: Display,
    {
        let mut tree = ArenaTree::new(game_state.clone(), self.iterations);

        for _ in 1..self.iterations
        {
            let leaf = tree.select_leaf_node();

            if !tree.is_node_termnial(&leaf)
            {
                tree.create_children_for(&leaf);

                //println!("{:?}", tree.get(&leaf));

                let node_to_simulate = *tree.children_of(&leaf).choose(&mut thread_rng()).unwrap();

                let result = tree.simulate_node(&node_to_simulate);
                tree.backprop_result(&leaf, result)
            }
            else
            {
                tree.backprop_result(&leaf, tree.get(&leaf).data.check_win());
            }
        }

        let best_state_ref = tree
            .children_of(&tree.root_ref())
            .iter()
            .max_by(|n1, n2| tree.get_ucb_value(n1).total_cmp(&tree.get_ucb_value(n2)))
            .unwrap();
        let best_state = tree.get(best_state_ref).data.clone();

        best_state.last_move().expect("state had no last move")
    }
}

trait GameStateTree
{
    fn select_leaf_node(&self) -> NodeRef;
    fn create_children_for(&mut self, node: &NodeRef);
    fn is_node_termnial(&self, node: &NodeRef) -> bool;
    fn simulate_node(&mut self, node: &NodeRef) -> GameResult;
    fn backprop_result(&mut self, node: &NodeRef, result: GameResult);

    fn get_ucb_value(&self, node: &NodeRef) -> f64;
}

impl<T> GameStateTree for ArenaTree<T>
where
    T: GameState,
{
    fn select_leaf_node(&self) -> NodeRef
    {
        let mut node = self.root_ref();

        while !self.children_of(&node).is_empty()
        {
            // TODO: Get max child by ucb value
            // TEMP: Get random child
            //node = *self.children_of(&node).choose(&mut thread_rng()).unwrap();
            node = *self
                .children_of(&node)
                .iter()
                .max_by(|n1, n2| self.get_ucb_value(n1).total_cmp(&self.get_ucb_value(n2)))
                .unwrap();
        }

        node
    }

    fn create_children_for(&mut self, node: &NodeRef)
    {
        if self.get(node).expanded
        {
            panic!("node already expanded");
        }
        if !self.children_of(node).is_empty()
        {
            panic!("tried to expand expanded node");
        }

        let moves = self.get(node).data.get_valid_moves();
        for m in moves
        {
            let mut state = self.get(node).data.clone();
            state.do_move(m);
            let child = self.insert(state, node);
            self.get_mut(node).children.push(child);
        }
    }

    fn is_node_termnial(&self, node: &NodeRef) -> bool
    {
        self.get(node).data.check_win() != GameResult::InProgress
    }

    fn simulate_node(&mut self, node: &NodeRef) -> GameResult
    {
        let player = RandomPlayer;
        self.get(node).data.clone().play(&player, &player, false)
    }

    fn backprop_result(&mut self, node: &NodeRef, result: GameResult)
    {
        let mut node = *node;
        loop
        {
            let n = self.get_mut(&node);
            n.num_plays += 1;

            n.score += match &result
            {
                GameResult::InProgress => panic!("game should be finished"),
                GameResult::Draw => 0.5,
                GameResult::Win(winner) =>
                {
                    if *winner != n.data.player_to_move()
                    {
                        1.0
                    }
                    else
                    {
                        0.0
                    }
                },
            };

            node = match n.parent
            {
                Some(parent) => parent,
                None => break,
            };
        }
    }

    fn get_ucb_value(&self, node: &NodeRef) -> f64
    {
        let node = self.get(node);
        let parent = self.get(
            &node
                .parent
                .expect("tried to get ucb of node with no parent"),
        );

        if node.num_plays == 0
        {
            return f64::INFINITY;
        }

        let value = node.score / node.num_plays as f64;
        let c = SQRT_2 / 2.0;
        let exploration = ((parent.num_plays as f64).log(E) / node.num_plays as f64).sqrt();

        value + c * exploration
    }
}

mod arena_tree
{
    use self::arena_vec::ArenaVec;

    pub use arena_vec::NodeRef;

    pub struct ArenaTree<T>
    {
        nodes: ArenaVec<Node<T>>,
        root: NodeRef,
    }
    impl<T> ArenaTree<T>
    {
        pub fn new(root_data: T, capacity: usize) -> ArenaTree<T>
        {
            let node = Node {
                parent: None,
                children: vec![],
                data: root_data,
                expanded: false,
                num_plays: 0,
                score: 0.0,
            };
            let mut nodes = ArenaVec::new(capacity);
            let root = nodes.insert(node);

            ArenaTree { nodes, root }
        }

        pub fn insert(&mut self, value: T, parent: &NodeRef) -> NodeRef
        {
            self.nodes.insert(Node {
                parent: Some(*parent),
                children: vec![],
                data: value,
                expanded: false,
                num_plays: 0,
                score: 0.0,
            })
        }

        pub fn get(&self, index: &NodeRef) -> &Node<T>
        {
            self.nodes.get(index)
        }

        pub fn get_mut(&mut self, index: &NodeRef) -> &mut Node<T>
        {
            self.nodes.get_mut(index)
        }

        pub fn children_of(&self, index: &NodeRef) -> &Vec<NodeRef>
        {
            &self.get(index).children
        }

        pub fn root_ref(&self) -> NodeRef
        {
            self.root
        }

        pub fn root(&self) -> &Node<T>
        {
            self.nodes.get(&self.root)
        }

        pub fn root_mut(&mut self) -> &mut Node<T>
        {
            self.nodes.get_mut(&self.root)
        }
    }

    #[derive(Debug)]
    pub struct Node<T>
    {
        pub parent: Option<NodeRef>,
        pub children: Vec<NodeRef>,
        pub data: T,
        pub num_plays: usize,
        pub score: f64,
        pub expanded: bool,
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

        #[derive(Clone, Copy, Debug)]
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
