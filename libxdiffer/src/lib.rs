use std::collections::HashMap;

use wasm_bindgen::prelude::*;

use x_diff_rs::{
    diff::{self, Edit},
    tree::{XNode, XTree},
};

#[cfg(debug_assertions)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(text: &str);
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct DiffTree {
    kind: DiffTreeKind,
    root: Option<DiffNode>,
    diff_count: usize,
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
    insert_pos: Option<usize>,
    children: Vec<DiffNode>,
    is_attribute: bool,
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

    #[wasm_bindgen]
    pub fn diff_count(&self) -> usize {
        self.diff_count
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

    #[wasm_bindgen]
    pub fn insert_pos(&self) -> Option<usize> {
        self.insert_pos
    }

    #[wasm_bindgen]
    pub fn is_attribute(&self) -> bool {
        self.is_attribute
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct Change {
    pub range1: Option<Range>,
    pub range2: Option<Range>,
    pub insert_pos: Option<usize>,
    pub is_attribute: bool,
}

#[wasm_bindgen]
impl Change {
    #[wasm_bindgen(constructor)]
    pub fn new(
        range1: Option<Range>,
        range2: Option<Range>,
        insert_pos: Option<usize>,
        is_attribute: bool,
    ) -> Self {
        Self {
            range1,
            range2,
            insert_pos,
            is_attribute,
        }
    }

    #[wasm_bindgen]
    pub fn copy(&self) -> Self {
        *self
    }
}

#[wasm_bindgen]
pub fn apply_changes(text1: &str, text2: &str, mut changes: Vec<Change>) -> String {
    // Adjust insert position for attribute nodes
    for change in &mut changes {
        if change.range1.is_none() && change.range2.is_some() && change.is_attribute {
            let pos = change.insert_pos.unwrap();
            let mut new_insert_pos = pos + text1[pos..text1.len()].find('>').unwrap();
            if &text1[new_insert_pos - 1..new_insert_pos] == "/" {
                new_insert_pos -= 1;
            }
            change.insert_pos = Some(new_insert_pos);
        }
    }
    changes.sort_by_key(|change| {
        std::cmp::Reverse(
            change
                .range1
                .map(|r| r.start)
                .or(change.insert_pos)
                .unwrap_or(0),
        )
    });
    let mut final_text = text1.to_string();
    for change in &changes {
        final_text = match (change.range1, change.range2, change.insert_pos) {
            (None, Some(r2), Some(pos)) => {
                let prefix_space = if change.is_attribute
                    && (text1[pos - 1..pos].find([' ', '\t', '\r', '\n']).is_none())
                {
                    " "
                } else {
                    ""
                };
                let suffix_space = if change.is_attribute && (&text1[pos..pos + 1] == "/") {
                    " "
                } else {
                    ""
                };
                format!(
                    "{head}{prefix_space}{insert}{suffix_space}{tail}",
                    head=&final_text[0..pos],
                    insert=&text2[r2.start..r2.end].trim(),
                    tail=&final_text[pos..final_text.len()]
                )
            }
            (Some(r1), None, None) => format!(
                "{}{}",
                &final_text[0..r1.start],
                &final_text[r1.end..final_text.len()]
            ),
            (Some(r1), Some(r2), None) => format!(
                "{}{}{}",
                &final_text[0..r1.start],
                &text2[r2.start..r2.end],
                &final_text[r1.end..final_text.len()]
            ),
            _ => unreachable!(),
        };
    }
    final_text
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
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
            diff_count: 0,
        });
    }
    if matches!(edits[0], Edit::ReplaceRoot) {
        return Ok(DiffTree {
            kind: DiffTreeKind::TotalDiff,
            root: None,
            diff_count: 1,
        });
    }
    let diff_count = edits.len();
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
            None,
        )),
        diff_count,
    })
}

fn build_diff_node(
    node: XNode,
    changed_nodes: &HashMap<String, Vec<Edit>>,
    kind: DiffNodeKind,
    insert_pos: Option<usize>,
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
                                None,
                            ));
                        }
                        return DiffNode {
                            name,
                            range1: Some(Range::from(xnode.range())),
                            range2: None,
                            kind: DiffNodeKind::DeletedNode,
                            insert_pos: None,
                            children: subnodes,
                            is_attribute: xnode.is_attribute(),
                        };
                    }
                    Edit::Update { old, new } => {
                        return DiffNode {
                            name,
                            range1: Some(Range::from(old.range())),
                            range2: Some(Range::from(new.range())),
                            kind: DiffNodeKind::UpdatedNode,
                            insert_pos: None,
                            children: Vec::new(),
                            is_attribute: old.is_attribute(),
                        };
                    }
                    Edit::ReplaceRoot => unreachable!(),
                }
            }
            let mut subnodes = Vec::new();
            let mut insert_pos = 0;
            for c in node.children() {
                subnodes.push(build_diff_node(c, changed_nodes, kind, None));
                if c.range().end > insert_pos {
                    insert_pos = c.range().end;
                }
            }
            for n in inserted_nodes {
                if n.is_attribute() {
                    insert_pos = node.range().start;
                }
                subnodes.push(build_diff_node(
                    n,
                    changed_nodes,
                    DiffNodeKind::AddedNode,
                    Some(insert_pos),
                ));
            }
            return DiffNode {
                name,
                range1: None,
                range2: None,
                kind,
                children: subnodes,
                insert_pos: None,
                is_attribute: node.is_attribute(),
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
        children.push(build_diff_node(c, changed_nodes, subnode_kind, None))
    }
    DiffNode {
        name,
        range1,
        range2,
        kind,
        children,
        insert_pos,
        is_attribute: node.is_attribute(),
    }
}
