use rowan::ast::AstNode;
use rue_syntax::{RueLang, SyntaxKind, SyntaxNode};

macro_rules! ast_node {
    ($name:ident, $kind:path) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $name {
            node: SyntaxNode,
        }

        impl AstNode for $name {
            type Language = RueLang;

            fn cast(node: SyntaxNode) -> Option<Self> {
                Self::can_cast(node.kind()).then(|| Self { node })
            }

            fn syntax(&self) -> &SyntaxNode {
                &self.node
            }

            fn can_cast(kind: SyntaxKind) -> bool {
                kind == $kind
            }
        }
    };
}

macro_rules! ast_enum {
    ($name:ident $(, $node:ident)+) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub enum $name {
            $($node($node)),+
        }

        impl AstNode for $name {
            type Language = RueLang;

            fn cast(node: SyntaxNode) -> Option<Self> {
                $( if let Some(value) = $node::cast(node) {
                    return Some(Self::$node(value));
                } )+
                None
            }

            fn syntax(&self) -> &SyntaxNode {
                match self { $(
                    Self::$node(value) => value.syntax(),
                )+ }
            }

            fn can_cast(kind: SyntaxKind) -> bool {
                $($node::can_cast(kind))|+
            }
        }
    };
}

ast_node!(Program, SyntaxKind::Program);

impl Program {
    pub fn items(&self) -> Vec<Item> {
        self.node.children().filter_map(Item::cast).collect()
    }

    pub fn expr(&self) -> Option<Expr> {
        self.node.children_with_tokens().find_map()
    }
}

ast_enum!(Item, FnDef);
ast_node!(FnDef, SyntaxKind::FnDef);

impl FnDef {
    pub fn name(&self) -> Option<String> {
        self.node
            .children_with_tokens()
            .find(|node| node.kind() == SyntaxKind::Ident)
            .and_then(|node| node.as_token().map(|node| node.text().to_string()))
    }
}

ast_enum!(Expr, Integer);
ast_node!(Integer, SyntaxKind::Integer);
