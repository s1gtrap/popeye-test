use std::collections::{HashMap, HashSet, VecDeque};
use std::iter::once;

use petgraph::graph::UnGraph;

use petgraph::visit::IntoNodeReferences;
use yew::prelude::*;

use crate::data::{Exercise, EXERCISES};
use crate::set::Set;

/// Welsh-Powell
fn optimize(sets: Vec<Vec<Exercise>>) -> Vec<Vec<Exercise>> {
    let (mut g, _) = sets.iter().flatten().fold(
        (
            petgraph::Graph::<_, (), _>::new_undirected(),
            HashMap::new(),
        ),
        |(mut g, mut t), e| {
            let n1 = g.add_node(e.id);
            let es = t.entry(e.muscle).or_insert(vec![]);
            for n2 in es.iter() {
                g.add_edge(n1, *n2, ());
            }
            es.push(n1);
            (g, t)
        },
    );
    log::info!("{:?}", petgraph::dot::Dot::new(&g));

    let mut nodes: Vec<_> = g.node_indices().collect();
    nodes.sort_by_key(|n| -(g.neighbors(*n).count() as isize));
    //log::info!("{:?}", nodes);

    let mut g1: UnGraph<(), ()> = UnGraph::new_undirected();
    let mut nodes = VecDeque::from(nodes);
    let mut c = 0;
    let mut s = vec![];
    while let Some(n) = g.node_indices().max_by_key(|n| g.neighbors(*n).count()) {
        let mut neighbors = g.neighbors(n).collect::<HashSet<_>>();
        log::info!("coloring {n:?} = {c}");
        s.push(vec![EXERCISES[&g.node_weight(n).unwrap()[..]]]);
        let mut v = vec![n];
        for (n0, _) in g.node_references() {
            if n == n0 || neighbors.contains(&n0) {
                continue;
            }
            log::info!("also coloring {n0:?} = {c}");
            v.push(n0);
            s[c].push(EXERCISES[&g.node_weight(n0).unwrap()[..]]);
            neighbors.extend(g.neighbors(n0));
            g.neighbors(n).collect::<HashSet<_>>();
        }
        for n in v {
            g.remove_node(n);
        }
        c += 1;
    }

    s
}

#[test]
fn test_optimize() {
    assert_eq!(optimize(vec![]), Vec::<Vec<Exercise>>::new());
    assert_eq!(
        optimize(vec![vec![EXERCISES["DumbbellCurl"]]]),
        vec![vec![EXERCISES["DumbbellCurl"]]],
    );
    assert_eq!(
        optimize(vec![vec![
            EXERCISES["DumbbellCurl"],
            EXERCISES["DumbbellCurl"],
        ]]),
        vec![
            vec![EXERCISES["DumbbellCurl"]],
            vec![EXERCISES["DumbbellCurl"]],
        ],
    );
    assert_eq!(
        optimize(vec![vec![
            EXERCISES["DumbbellCurl"],
            EXERCISES["BarbellCurl"],
        ]]),
        vec![
            vec![EXERCISES["BarbellCurl"]],
            vec![EXERCISES["DumbbellCurl"]],
        ],
    );
    assert_eq!(
        optimize(vec![
            vec![EXERCISES["DumbbellCurl"]],
            vec![EXERCISES["DumbbellCurl"]],
        ]),
        vec![
            vec![EXERCISES["DumbbellCurl"]],
            vec![EXERCISES["DumbbellCurl"]],
        ],
    );
    assert_eq!(
        optimize(vec![
            vec![EXERCISES["DumbbellCurl"]],
            vec![EXERCISES["BarbellCurl"]],
        ]),
        vec![
            vec![EXERCISES["BarbellCurl"]],
            vec![EXERCISES["DumbbellCurl"]],
        ],
    );
    assert_eq!(
        optimize(vec![
            vec![EXERCISES["BenchPress"]],
            vec![EXERCISES["BarbellCurl"]]
        ]),
        vec![vec![EXERCISES["BarbellCurl"], EXERCISES["BenchPress"]]],
    );
    assert_eq!(
        optimize(vec![
            vec![
                EXERCISES["InclineBenchPress"],
                EXERCISES["BarbellCurl"],
                EXERCISES["InclineBenchPress"],
            ],
            vec![EXERCISES["BarbellCurl"]],
        ]),
        vec![
            vec![EXERCISES["BarbellCurl"], EXERCISES["InclineBenchPress"]],
            vec![EXERCISES["BarbellCurl"], EXERCISES["InclineBenchPress"]],
        ],
    );
}

#[function_component(App)]
pub fn app() -> Html {
    let sets = use_state(|| Vec::<Vec<Exercise>>::new());

    {
        let sets = sets.clone();
        use_effect_with_deps(
            move |s| {
                let (g, _) = (*s).iter().flatten().fold(
                    (
                        petgraph::Graph::<_, (), _>::new_undirected(),
                        HashMap::new(),
                    ),
                    |(mut g, mut t), e| {
                        let n1 = g.add_node(e.id);
                        let es = t.entry(e.muscle).or_insert(vec![]);
                        for n2 in es.iter() {
                            g.add_edge(n1, *n2, ());
                        }
                        es.push(n1);
                        (g, t)
                    },
                );
            },
            sets,
        );
    }

    let on_add = {
        let sets = sets.clone();
        Callback::from(move |o: String| {
            sets.set(
                (*sets)
                    .clone()
                    .into_iter()
                    .chain(once(vec![EXERCISES[&o[..]]]))
                    .collect(),
            );
        })
    };

    let onclick = {
        let sets = sets.clone();
        Callback::from(move |_| {
            sets.set(optimize((*sets).clone()));
        })
    };

    html! {
        <main>
            {
                sets
                    .iter()
                    .enumerate()
                    .map(|(i, e)| {
                        let on_add = {
                            let sets = sets.clone();
                            Callback::from(move |o: String| {
                                let mut s = (*sets).clone();
                                s[i].push(EXERCISES[&o[..]]);
                                sets.set(s);
                            })
                        };
                        html!(<Set index={i} exercises={e.clone()} on_add={on_add} />)
                    })
                    .collect::<Html>()
            }
            <Set index={sets.len()} exercises={vec![]} on_add={on_add} />
            <button onclick={onclick}>{ "Optimize!" }</button>
        </main>
    }
}
