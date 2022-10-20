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

mod tree
{
    use std::ops::Index;

    #[derive(Clone, Copy)]
    struct NodeRef(usize);
    struct Node<T>
    {
        parent: Option<NodeRef>,
        children: Vec<NodeRef>,
        data: T,
    }

    struct Tree<T>
    {
        nodes: Vec<Node<T>>,
        root: NodeRef,
    }

    impl<T> Index<NodeRef> for Tree<T>
    {
        type Output = T;

        fn index(&self, index: NodeRef) -> &Self::Output
        {
            &self.nodes[index.0].data
        }
    }

    impl<T> Tree<T>
    {
        fn new(root_data: T) -> Tree<T>
        {
            let root = Node {
                parent: None,
                children: vec![],
                data: root_data,
            };
            let nodes = vec![root];

            Tree {
                nodes: vec![],
                root: NodeRef(0),
            }
        }

        fn get_children(&self, node: NodeRef) -> &Vec<NodeRef>
        {
            &self.nodes[node.0].children
        }

        fn root(&self) -> NodeRef
        {
            self.root
        }
    }
}
