use rue_syntax::{SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Program(SyntaxNode);

impl Program {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        (node.kind() == SyntaxKind::Program).then(|| Self(node))
    }

    pub fn items(&self) -> Vec<Item> {
        self.0.children().filter_map(Item::cast).collect()
    }

    pub fn expr(&self) -> Option<Expr> {
        self.0.children_with_tokens().find_map(Expr::cast)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Block(SyntaxNode);

impl Block {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        (node.kind() == SyntaxKind::Block).then(|| Self(node))
    }

    pub fn items(&self) -> Vec<Item> {
        self.0.children().filter_map(Item::cast).collect()
    }

    pub fn expr(&self) -> Option<Expr> {
        self.0.children_with_tokens().find_map(Expr::cast)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Item {
    FnDf(FnDef),
}

impl Item {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        if let Some(value) = FnDef::cast(node) {
            Some(Self::FnDf(value))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnDef(SyntaxNode);

impl FnDef {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        (node.kind() == SyntaxKind::FnDef).then(|| Self(node))
    }

    pub fn name(&self) -> Option<SyntaxToken> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find(|token| token.kind() == SyntaxKind::Ident)
    }

    pub fn param_list(&self) -> Option<FnParamList> {
        self.0.children().find_map(FnParamList::cast)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnParamList(SyntaxNode);

impl FnParamList {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        (node.kind() == SyntaxKind::FnParamList).then(|| Self(node))
    }

    pub fn params(&self) -> Vec<FnParam> {
        self.0.children().filter_map(FnParam::cast).collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnParam(SyntaxNode);

impl FnParam {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        (node.kind() == SyntaxKind::FnParam).then(|| Self(node))
    }

    pub fn name(&self) -> Option<SyntaxToken> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find(|token| token.kind() == SyntaxKind::Ident)
    }

    pub fn ty(&self) -> Option<Type> {
        self.0.children_with_tokens().find_map(Type::cast)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    Integer(SyntaxToken),
}

impl Expr {
    pub fn cast(node: SyntaxElement) -> Option<Self> {
        match node.kind() {
            SyntaxKind::Integer => Some(Self::Integer(node.into_token()?)),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Named(SyntaxToken),
}

impl Type {
    pub fn cast(node: SyntaxElement) -> Option<Self> {
        match node.kind() {
            SyntaxKind::Ident => Some(Self::Named(node.into_token()?)),
            _ => None,
        }
    }
}
