use ijson::{IString, IValue};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{self, cmp::Ordering, collections::HashMap, error::Error, fs};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default, Clone)]
pub struct OptionData {
    pub default: Option<IValue>,
    pub description: IValue,
    #[serde(alias = "readOnly")]
    pub read_only: bool,
    #[serde(alias = "type")]
    pub op_type: IString,
    pub declarations: Vec<IString>,
    pub example: Option<IValue>,
}

#[derive(Default, Debug, PartialEq)]
pub struct AttrTree {
    pub attributes: HashMap<String, AttrTree>,
    pub options: Vec<String>,
}

pub fn read(file: &str) -> Result<(HashMap<String, OptionData>, AttrTree), Box<dyn Error>> {
    let f = fs::read_to_string(file)?;
    let data: HashMap<String, OptionData> = serde_json::from_str(&f)?;
    let ops = data.keys().map(|x| x.as_str()).collect::<Vec<_>>();
    let tree = buildtree(ops)?;
    Ok((data, tree))
}

pub fn attrloc(tree: &AttrTree, pos: Vec<String>) -> Option<&AttrTree> {
    match pos.len().cmp(&1) {
        Ordering::Greater => match tree.attributes.get(&pos[0]) {
            Some(x) => attrloc(x, pos[1..].to_vec()),
            None => None,
        },
        Ordering::Equal => tree.attributes.get(&pos[0]),
        Ordering::Less => Some(tree),
    }
}

fn buildtree(ops: Vec<&str>) -> Result<AttrTree, Box<dyn Error>> {
    let split = ops
        .into_iter()
        .map(|x| x.split('.').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut tree = AttrTree {
        attributes: HashMap::new(),
        options: vec![],
    };
    for attr in split {
        match attr.len().cmp(&1) {
            Ordering::Greater => {
                if tree.attributes.get(attr[0]).is_none() {
                    tree.attributes.insert(
                        attr[0].to_string(),
                        AttrTree {
                            attributes: HashMap::new(),
                            options: vec![],
                        },
                    );
                }
                buildtree_child(
                    tree.attributes.get_mut(attr[0]).unwrap(),
                    attr[1..].to_vec(),
                )?;
            }
            Ordering::Equal => tree.options.push(attr[0].to_string()),
            Ordering::Less => {}
        }
    }
    Ok(tree)
}

fn buildtree_child(tree: &mut AttrTree, attr: Vec<&str>) -> Result<(), Box<dyn Error>> {
    match attr.len().cmp(&1) {
        Ordering::Greater => {
            if tree.attributes.get(attr[0]).is_none() {
                tree.attributes.insert(
                    attr[0].to_string(),
                    AttrTree {
                        attributes: HashMap::new(),
                        options: vec![],
                    },
                );
            }
            buildtree_child(
                tree.attributes.get_mut(attr[0]).unwrap(),
                attr[1..].to_vec(),
            )
        }
        Ordering::Equal => {
            tree.options.push(attr[0].to_string());
            Ok(())
        }
        Ordering::Less => Ok(()),
    }
}
