use rand::prelude::*;
use smiles_with_selectors::definitions::bond::BondType;
use smiles_with_selectors::petgraph::graph::NodeIndex;
use smiles_with_selectors::workspace::Workspace;

pub trait SmilesStackfulSelector {
    fn find_with_selector(&self, root: NodeIndex, target_selector: &str) -> Option<NodeIndex>;
    fn filter_with_selector(&self, root: NodeIndex, target_selector: &str) -> Vec<NodeIndex>;
    fn remove_selector(&mut self, target: NodeIndex, target_selector: &str) -> Option<&String>;
    fn add_selectors(&mut self, target: NodeIndex, selector: &[&str]) -> Option<&String>;
}

impl SmilesStackfulSelector for Workspace {
    fn find_with_selector(&self, root: NodeIndex, target_selector: &str) -> Option<NodeIndex> {
        self.find_node_in_structure(root, |atom| {
            if let Some(selector) = &atom.selector {
                let selectors = selector.split(";").collect::<Vec<&str>>();
                selectors.contains(&target_selector)
            } else {
                false
            }
        })
    }

    fn filter_with_selector(&self, root: NodeIndex, target_selector: &str) -> Vec<NodeIndex> {
        self.filter_nodes_in_structure(root, |atom| {
            if let Some(selector) = &atom.selector {
                let selectors = selector.split(";").collect::<Vec<&str>>();
                selectors.contains(&target_selector)
            } else {
                false
            }
        })
        .unwrap()
    }

    fn remove_selector(&mut self, target: NodeIndex, target_selector: &str) -> Option<&String> {
        self.get_atom_mut(target).and_then(|atom| {
            if let Some(selectors) = &atom.selector {
                let selectors = selectors.split(";").collect::<Vec<_>>();
                let outgoing_index = selectors
                    .iter()
                    .position(|selector| selector == &target_selector)
                    .unwrap();
                let before = &selectors[0..outgoing_index].join(";");
                let after = &selectors[outgoing_index + 1..].join(";");
                let mut updated_selectors = String::new();
                updated_selectors.push_str(before);
                updated_selectors.push_str(";");
                updated_selectors.push_str(after);
                if updated_selectors == ";" {
                    atom.selector = None
                } else {
                    atom.selector = Some(updated_selectors);
                }
                atom.selector.as_ref()
            } else {
                None
            }
        })
    }

    fn add_selectors(&mut self, target: NodeIndex, added_selectors: &[&str]) -> Option<&String> {
        self.get_atom_mut(target).and_then(|atom| {
            if let Some(selectors) = &atom.selector {
                let mut added = added_selectors.join(";");
                added.push_str(";");
                added.push_str(selectors);
                atom.selector = Some(added);
                atom.selector.as_ref()
            } else {
                None
            }
        })
    }
}

fn random_take_one<'a, E>(collection: &'a Vec<E>) -> &'a E {
    let size = collection.len();
    let mut rng = thread_rng();
    let rand_value = rng.gen_range(0..size);
    &collection[rand_value]
}

pub fn random_generate_structure(
    start: &str,
    replacers: Vec<(isize, usize, Vec<&str>, &str, &str, &str)>, // select n replacer, replace n times each replacer, avaliable replacers, replacer incoming token, replacer outgoing token, bond type token
) -> Option<String> {
    let mut ws = Workspace::new();
    let start_point = ws.add_structure(start).unwrap();
    for (amount, times, r_sources, incoming_token, outgoing_token, bond_token) in replacers {
        let bond_type = BondType::new(bond_token).unwrap();
        if r_sources.len() == 0 {
            loop {
                let outgoing = ws.find_with_selector(start_point, outgoing_token);
                let incoming = ws.find_with_selector(start_point, incoming_token);
                if let Some(outgoing) = outgoing {
                    if let Some(incoming) = incoming {
                        ws.connect(outgoing, incoming, bond_type);
                        ws.remove_selector(outgoing, outgoing_token);
                        ws.remove_selector(incoming, incoming_token);
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
        if amount >= 0 {
            for _ in 0..amount {
                let replacer_fragment = random_take_one(&r_sources);
                let replacer_root = ws.add_structure(replacer_fragment).unwrap();
                for _ in 0..times {
                    let outgoing = ws.find_with_selector(start_point, outgoing_token);
                    let incoming = ws.find_with_selector(replacer_root, incoming_token);
                    if let Some(outgoing) = outgoing {
                        if let Some(incoming) = incoming {
                            ws.connect(outgoing, incoming, bond_type);
                            ws.remove_selector(outgoing, outgoing_token);
                            ws.remove_selector(incoming, incoming_token);
                        }
                    }
                }
            }
        } else {
            while let Some(outgoing) = ws.find_with_selector(start_point, outgoing_token) {
                let replacer_fragment = random_take_one(&r_sources);
                let replacer_root = ws.add_structure(replacer_fragment).unwrap();
                let incoming = ws
                    .find_with_selector(replacer_root, incoming_token)
                    .unwrap();
                ws.connect(outgoing, incoming, bond_type);
                ws.remove_selector(outgoing, outgoing_token);
                ws.remove_selector(incoming, incoming_token);
            }
        }
    }
    ws.to_sws(start_point)
}

#[test]
fn with_metal() {
    let ligands = random_generate_structure(
        "[P{R;R;Out;L}]",
        vec![
            (
                1,
                1,
                vec![
                    "[c{In}]1cccc[c{Out}]1",
                    "[c{In}]1ccc[c{Out}]c1",
                    "[C{R;In;Out}]",
                ],
                "In",
                "Out",
                "-",
            ),
            (1, 1, vec!["[P{R;R;In;L}]"], "In", "Out", "-"),
            (
                -1,
                1,
                vec![
                    "[H{RIn}]",
                    "[C{RIn}]",
                    "[OH{RIn}]",
                    "[c{RIn}]1ccccc1",
                ],
                "RIn",
                "R",
                "-",
            ),
        ],
    );

    let complex = random_generate_structure("[Fe+2{LIn;LIn;LIn;LIn}]", vec![
        (2,2,vec![&ligands.unwrap()], "L", "LIn", "-")
    ]).unwrap();
    println!("{}", complex)
}
