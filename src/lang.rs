use num_traits::{FromPrimitive, ToPrimitive};

use crate::syntax_kind::SyntaxKind;

pub type SyntaxNode = rowan::SyntaxNode<Rue>;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rue {}

impl rowan::Language for Rue {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        SyntaxKind::from_u16(raw.0).unwrap()
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.to_u16().unwrap())
    }
}
