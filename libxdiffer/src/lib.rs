use std::collections::HashMap;

use wasm_bindgen::prelude::*;

use x_diff_rs::{
    diff::{self, Edit},
    tree::{XNode, XTree},
};

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct DiffTree {
    kind: DiffTreeKind,
    root: Option<DiffNode>,
}

#[derive(Debug, Clone, Copy)]
#[wasm_bindgen]
pub enum DiffTreeKind {
    Same,
    TotalDiff,
    PartialDiff,
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct DiffNode {
    name: String,
    range1: Option<Range>,
    range2: Option<Range>,
    kind: DiffNodeKind,
    children: Vec<DiffNode>,
}

#[derive(Debug, Clone, Copy)]
#[wasm_bindgen]
pub enum DiffNodeKind {
    NoDiff,
    DeletedNode,
    DeletedSubNode,
    AddedNode,
    AddedSubNode,
    UpdatedNode,
}

#[derive(Debug, Clone, Copy)]
#[wasm_bindgen]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

impl From<std::ops::Range<usize>> for Range {
    fn from(value: std::ops::Range<usize>) -> Self {
        Self {
            start: value.start,
            end: value.end,
        }
    }
}

#[wasm_bindgen]
impl DiffTree {
    #[wasm_bindgen]
    pub fn kind(&self) -> DiffTreeKind {
        self.kind
    }

    #[wasm_bindgen]
    pub fn root(&self) -> Option<DiffNode> {
        self.root.clone()
    }
}

#[wasm_bindgen]
impl DiffNode {
    #[wasm_bindgen]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen]
    pub fn range1(&self) -> Option<Range> {
        self.range1
    }

    #[wasm_bindgen]
    pub fn range2(&self) -> Option<Range> {
        self.range2
    }

    #[wasm_bindgen]
    pub fn kind(&self) -> DiffNodeKind {
        self.kind
    }

    #[wasm_bindgen]
    pub fn children(&self) -> Vec<DiffNode> {
        self.children.clone()
    }
}

#[wasm_bindgen]
pub struct SplittedText {
    head: String,
    middle: String,
    tail: String,
}

#[wasm_bindgen]
impl SplittedText {
    #[wasm_bindgen]
    pub fn head(&self) -> String {
        self.head.clone()
    }

    #[wasm_bindgen]
    pub fn middle(&self) -> String {
        self.middle.clone()
    }

    #[wasm_bindgen]
    pub fn tail(&self) -> String {
        self.tail.clone()
    }
}

#[wasm_bindgen]
pub fn split_by_range(text: &str, range: Option<Range>) -> SplittedText {
    if let Some(r) = range {
        let start = r.start;
        let end = r.end;
        let head = &text[0..start];
        let middle = &text[start..end];
        let tail = &text[end..text.len()];
        SplittedText {
            head: head.to_string(),
            middle: middle.to_string(),
            tail: tail.to_string(),
        }
    } else {
        SplittedText {
            head: text.to_string(),
            middle: "".to_string(),
            tail: "".to_string(),
        }
    }
}

#[wasm_bindgen]
pub fn build_diff_tree(xml1: &str, xml2: &str) -> Result<DiffTree, String> {
    let tree1 = XTree::parse(&xml1).map_err(|e| format!("Cannot parse tree1. {e:?}"))?;
    let tree2 = XTree::parse(&xml2).map_err(|e| format!("Cannot parse tree2. {e:?}"))?;
    let edits = diff::diff(&tree1, &tree2);
    if edits.is_empty() {
        return Ok(DiffTree {
            kind: DiffTreeKind::Same,
            root: None,
        });
    }
    if matches!(edits[0], Edit::ReplaceRoot) {
        return Ok(DiffTree {
            kind: DiffTreeKind::TotalDiff,
            root: None,
        });
    }
    let mut changed_nodes = HashMap::new();
    for e in edits {
        let key = match e {
            Edit::Insert {
                child_node: _,
                to_node,
            } => to_node.id().to_string(),
            Edit::Delete(xnode) => xnode.id().to_string(),
            Edit::Update { old, new: _ } => old.id().to_string(),
            Edit::ReplaceRoot => unreachable!(),
        };
        changed_nodes.entry(key).or_insert(Vec::new()).push(e);
    }
    Ok(DiffTree {
        kind: DiffTreeKind::PartialDiff,
        root: Some(build_diff_node(
            tree1.root(),
            &changed_nodes,
            DiffNodeKind::NoDiff,
        )),
    })
}

fn build_diff_node(
    node: XNode,
    changed_nodes: &HashMap<String, Vec<Edit>>,
    kind: DiffNodeKind,
) -> DiffNode {
    let name = match node.name() {
        x_diff_rs::tree::XNodeName::TagName(expanded_name) => format!("<{}>", expanded_name.name()),
        x_diff_rs::tree::XNodeName::AttributeName(attribute) => format!("[{}]", attribute.name()),
        x_diff_rs::tree::XNodeName::Text => "TEXT".to_string(),
    };
    if let Some(edits) = changed_nodes.get(&node.id().to_string()) {
        // If the node is added (i.e it is in tree2) then even if its id exists in changed_nodes, that's just coincidence
        if !(matches!(kind, DiffNodeKind::AddedNode) || matches!(kind, DiffNodeKind::AddedSubNode))
        {
            let mut inserted_nodes = Vec::new();
            for e in edits {
                match e {
                    Edit::Insert {
                        child_node,
                        to_node: _,
                    } => {
                        inserted_nodes.push(*child_node);
                    }
                    Edit::Delete(xnode) => {
                        let mut subnodes = Vec::new();
                        for c in xnode.children() {
                            subnodes.push(build_diff_node(
                                c,
                                changed_nodes,
                                DiffNodeKind::DeletedSubNode,
                            ));
                        }
                        return DiffNode {
                            name,
                            range1: Some(Range::from(xnode.range())),
                            range2: None,
                            kind: DiffNodeKind::DeletedNode,
                            children: subnodes,
                        };
                    }
                    Edit::Update { old, new } => {
                        return DiffNode {
                            name,
                            range1: Some(Range::from(old.range())),
                            range2: Some(Range::from(new.range())),
                            kind: DiffNodeKind::UpdatedNode,
                            children: Vec::new(),
                        };
                    }
                    Edit::ReplaceRoot => unreachable!(),
                }
            }
            let mut subnodes = Vec::new();
            for c in node.children() {
                subnodes.push(build_diff_node(c, changed_nodes, kind));
            }
            for n in inserted_nodes {
                subnodes.push(build_diff_node(n, changed_nodes, DiffNodeKind::AddedNode));
            }
            return DiffNode {
                name,
                range1: None,
                range2: None,
                kind,
                children: subnodes,
            };
        }
    }
    let (range1, range2, subnode_kind) = match kind {
        DiffNodeKind::NoDiff => (None, None, kind),
        DiffNodeKind::DeletedSubNode => (Some(Range::from(node.range())), None, kind),
        DiffNodeKind::AddedSubNode => (None, Some(Range::from(node.range())), kind),
        DiffNodeKind::AddedNode => (
            None,
            Some(Range::from(node.range())),
            DiffNodeKind::AddedSubNode,
        ),
        _ => unreachable!(),
    };
    let mut children = Vec::new();
    for c in node.children() {
        children.push(build_diff_node(c, changed_nodes, subnode_kind))
    }
    DiffNode {
        name,
        range1,
        range2,
        kind,
        children,
    }
}
