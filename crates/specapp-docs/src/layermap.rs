//! The layer map: blocks as cards in rows by layer (the highest layer on top), placed on a column
//! grid so that dependencies run as straight as they can, and the dependency arrows routed
//! between the cards. Lifted from agenticspec's generator and made generic: the lowest layer is
//! drawn as one band only when it holds one block that every other block uses; a row of many
//! blocks is ordered by barycentre sweeps instead of trying every placement.

use std::collections::BTreeMap;

const MIN_WIDTH: f64 = 980.0;
const MIN_CARD_W: f64 = 150.0;
const LABEL: f64 = 104.0;
const GUTTER: f64 = 30.0;
const CARD_H: f64 = 92.0;
const BAND_H: f64 = 64.0;
const ROW_GAP: f64 = 50.0;
const TOP: f64 = 6.0;
const BOTTOM: f64 = 6.0;
/// The most placements of one row that are tried one by one.
const MAX_PLACEMENTS: usize = 5040;

/// One block as the map places it: its index in the input and its row rank (0 is the lowest).
#[derive(Debug, Clone)]
pub struct Node {
    pub rank: usize,
    pub row_name: String,
    pub row_number: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct Card {
    pub index: usize,
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    pub band: bool,
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub count: usize,
    pub path: String,
}

#[derive(Debug, Clone)]
pub struct Row {
    pub number: Option<u32>,
    pub name: String,
    pub y: f64,
    pub h: f64,
}

#[derive(Debug, Clone)]
pub struct Map {
    pub width: f64,
    pub height: f64,
    pub rows: Vec<Row>,
    pub cards: Vec<Card>,
    pub edges: Vec<Edge>,
    /// Whether the lowest row is drawn as one band whose arrows are left out.
    pub band: bool,
}

fn r1(x: f64) -> f64 {
    let r = (x * 10.0).round() / 10.0;
    if r == 0.0 { 0.0 } else { r }
}

fn count_placements(k: usize, n: usize) -> usize {
    (0..k).fold(1usize, |acc, i| acc.saturating_mul(n - i))
}

/// Every injective placement of `k` items on `n` slots, in lexicographic order.
fn placements(k: usize, n: usize) -> Vec<Vec<usize>> {
    fn go(k: usize, n: usize, cur: &mut Vec<usize>, out: &mut Vec<Vec<usize>>) {
        if cur.len() == k {
            out.push(cur.clone());
            return;
        }
        for s in 0..n {
            if !cur.contains(&s) {
                cur.push(s);
                go(k, n, cur, out);
                cur.pop();
            }
        }
    }
    let mut out = Vec::new();
    go(k, n, &mut Vec::new(), &mut out);
    out
}

/// Lays out the map. `nodes` are the blocks in their input order; `edges` are (from, to,
/// count) between their indexes.
pub fn layout(nodes: &[Node], edges: &[(usize, usize, usize)]) -> Map {
    let top = nodes.iter().map(|n| n.rank).max().unwrap_or(0);
    let mut by_rank: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for (i, n) in nodes.iter().enumerate() {
        by_rank.entry(n.rank).or_default().push(i);
    }
    // The band: the lowest row holds one block, there are others, and each of them uses it.
    let band = match by_rank.get(&0) {
        Some(lowest) if lowest.len() == 1 && nodes.len() > 1 && top > 0 => {
            let b = lowest[0];
            (0..nodes.len())
                .filter(|i| *i != b)
                .all(|i| edges.iter().any(|(f, t, _)| *f == i && *t == b))
        }
        _ => false,
    };
    let in_band = |i: usize| band && nodes[i].rank == 0;
    let columns = by_rank
        .iter()
        .filter(|(r, _)| !(band && **r == 0))
        .map(|(_, v)| v.len())
        .max()
        .unwrap_or(1)
        .max(1);
    let width = MIN_WIDTH.max(LABEL + GUTTER + columns as f64 * (MIN_CARD_W + GUTTER));
    let card_w = (width - LABEL - GUTTER * (columns as f64 + 1.0)) / columns as f64;
    let drawn: Vec<(usize, usize, usize)> = edges
        .iter()
        .copied()
        .filter(|(f, t, _)| !in_band(*f) && !in_band(*t))
        .collect();

    // Columns: start centred, then sweep the rows, each taking the placement that keeps its
    // dependencies straightest against every other row.
    let mut slot = vec![0usize; nodes.len()];
    for (rank, members) in &by_rank {
        if band && *rank == 0 {
            continue;
        }
        let start = (columns - members.len()) / 2;
        for (k, i) in members.iter().enumerate() {
            slot[*i] = start + k;
        }
    }
    let ranks: Vec<usize> = by_rank
        .keys()
        .copied()
        .filter(|r| !(band && *r == 0))
        .collect();
    let cost_of = |members: &[usize], p: &[usize], slot: &[usize]| -> i64 {
        let at = |i: usize| {
            members
                .iter()
                .position(|m| *m == i)
                .map_or(slot[i], |k| p[k])
        };
        let mut cost: i64 = p
            .iter()
            .map(|s| (2 * *s as i64 - (columns as i64 - 1)).abs())
            .sum();
        for (f, t, _) in &drawn {
            let (f, t) = (*f, *t);
            if !members.contains(&f) && !members.contains(&t) {
                continue;
            }
            let d = (at(f) as i64 - at(t) as i64).abs();
            if nodes[f].rank == nodes[t].rank {
                cost += 16 * (d - 1).max(0);
            } else {
                let span = nodes[f].rank.abs_diff(nodes[t].rank) as i64;
                cost += 4 * d * if span == 1 { 3 } else { 1 };
            }
        }
        cost
    };
    for sweep in 0..8 {
        let order: Vec<usize> = if sweep % 2 == 0 {
            ranks.iter().rev().copied().collect()
        } else {
            ranks.clone()
        };
        for rank in order {
            let members = &by_rank[&rank];
            if count_placements(members.len(), columns) <= MAX_PLACEMENTS {
                let mut best: Option<(i64, Vec<usize>)> = None;
                for p in placements(members.len(), columns) {
                    let cost = cost_of(members, &p, &slot);
                    if best.as_ref().is_none_or(|(c, _)| cost < *c) {
                        best = Some((cost, p));
                    }
                }
                if let Some((_, p)) = best {
                    for (k, i) in members.iter().enumerate() {
                        slot[*i] = p[k];
                    }
                }
            } else {
                // Barycentre of each member's neighbours, then the slots in that order.
                let mut keyed: Vec<(f64, usize)> = members
                    .iter()
                    .map(|&i| {
                        let near: Vec<f64> = drawn
                            .iter()
                            .filter_map(|(f, t, _)| {
                                if *f == i {
                                    Some(slot[*t] as f64)
                                } else if *t == i {
                                    Some(slot[*f] as f64)
                                } else {
                                    None
                                }
                            })
                            .collect();
                        let key = if near.is_empty() {
                            slot[i] as f64
                        } else {
                            near.iter().sum::<f64>() / near.len() as f64
                        };
                        (key, i)
                    })
                    .collect();
                keyed.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
                let start = (columns - members.len()) / 2;
                for (k, (_, i)) in keyed.into_iter().enumerate() {
                    slot[i] = start + k;
                }
            }
        }
    }

    let row_y = |rank: usize| TOP + (top - rank) as f64 * (CARD_H + ROW_GAP);
    let col_x = |c: usize| LABEL + GUTTER + c as f64 * (card_w + GUTTER);
    let gutter_x = |g: usize| LABEL + GUTTER / 2.0 + g as f64 * (card_w + GUTTER);
    let mut cards = Vec::new();
    for (i, n) in nodes.iter().enumerate() {
        if in_band(i) {
            cards.push(Card {
                index: i,
                x: r1(col_x(0)),
                y: r1(row_y(0)),
                w: r1(columns as f64 * card_w + (columns as f64 - 1.0) * GUTTER),
                h: BAND_H,
                band: true,
            });
        } else {
            cards.push(Card {
                index: i,
                x: r1(col_x(slot[i])),
                y: r1(row_y(n.rank)),
                w: r1(card_w),
                h: CARD_H,
                band: false,
            });
        }
    }
    let card = |i: usize| &cards[i];
    let mut rows: Vec<Row> = by_rank
        .iter()
        .map(|(rank, members)| Row {
            number: nodes[members[0]].row_number,
            name: nodes[members[0]].row_name.clone(),
            y: r1(row_y(*rank)),
            h: if band && *rank == 0 { BAND_H } else { CARD_H },
        })
        .collect();
    rows.reverse();

    // Arrows in a fixed order: from the top row down, left to right.
    let mut order: Vec<(usize, usize, usize)> = drawn.clone();
    order.sort_by_key(|(f, t, _)| {
        (
            std::cmp::Reverse(nodes[*f].rank),
            slot[*f],
            std::cmp::Reverse(nodes[*t].rank),
            slot[*t],
            *f,
            *t,
        )
    });
    // A long arrow runs down a gutter between two columns; pick the least used one between them.
    let mut gutter_of: BTreeMap<(usize, usize), usize> = BTreeMap::new();
    let mut load = vec![0usize; columns + 1];
    for (f, t, _) in &order {
        let (rf, rt) = (nodes[*f].rank, nodes[*t].rank);
        if rf == rt || rf.abs_diff(rt) == 1 {
            continue;
        }
        let (a, b) = (slot[*f].min(slot[*t]), slot[*f].max(slot[*t]));
        let candidates: Vec<usize> = if a == b {
            vec![a, a + 1]
        } else {
            (a + 1..=b).collect()
        };
        let g = *candidates
            .iter()
            .min_by_key(|g| (load[**g], g.abs_diff(slot[*t])))
            .expect("a gutter");
        load[g] += 1;
        gutter_of.insert((*f, *t), g);
    }
    // Lanes inside each gutter.
    let mut lane: BTreeMap<(usize, usize), f64> = BTreeMap::new();
    for g in 0..=columns {
        let mut users: Vec<(usize, usize)> = gutter_of
            .iter()
            .filter(|(_, x)| **x == g)
            .map(|(k, _)| *k)
            .collect();
        users.sort_by_key(|(f, t)| (std::cmp::Reverse(nodes[*f].rank), nodes[*t].rank, *f, *t));
        let n = users.len();
        let step = if n > 1 {
            ((GUTTER - 10.0) / (n as f64 - 1.0)).min(6.0)
        } else {
            0.0
        };
        for (k, e) in users.into_iter().enumerate() {
            lane.insert(e, (k as f64 - (n as f64 - 1.0) / 2.0) * step);
        }
    }
    // The x a card's arrow leaves from or arrives at, spread over its edge by the other end's x.
    let toward = |f: usize, t: usize, at_source: bool| -> f64 {
        match gutter_of.get(&(f, t)) {
            Some(g) => gutter_x(*g) + lane[&(f, t)],
            None => {
                let other = if at_source { card(t) } else { card(f) };
                other.x + other.w / 2.0
            }
        }
    };
    let mut ports: BTreeMap<(usize, usize, bool), f64> = BTreeMap::new();
    for i in 0..nodes.len() {
        for at_source in [true, false] {
            let mut mine: Vec<(f64, usize, usize)> = order
                .iter()
                .filter(|(f, t, _)| nodes[*f].rank != nodes[*t].rank)
                .filter(|(f, t, _)| if at_source { *f == i } else { *t == i })
                .map(|(f, t, _)| (toward(*f, *t, at_source), *f, *t))
                .collect();
            mine.sort_by(|a, b| a.0.total_cmp(&b.0).then((a.1, a.2).cmp(&(b.1, b.2))));
            let c = card(i);
            let n = mine.len() as f64;
            let span = (c.w - 48.0).min(14.0 * n);
            for (k, (_, f, t)) in mine.into_iter().enumerate() {
                let x = c.x + c.w / 2.0 - span / 2.0 + span * (k as f64 + 0.5) / n;
                ports.insert((f, t, at_source), x);
            }
        }
    }
    let mut out = Vec::new();
    for (f, t, count) in order {
        let (cf, ct) = (card(f), card(t));
        let path = if nodes[f].rank == nodes[t].rank {
            let y = cf.y + cf.h / 2.0;
            if cf.x < ct.x {
                format!("M{} {} L{} {}", r1(cf.x + cf.w), r1(y), r1(ct.x), r1(y))
            } else {
                format!("M{} {} L{} {}", r1(cf.x), r1(y), r1(ct.x + ct.w), r1(y))
            }
        } else {
            // An arrow upward (a dependency on a higher row) leaves the top of its card.
            let up = nodes[f].rank < nodes[t].rank;
            let sx = ports[&(f, t, true)];
            let tx = ports[&(f, t, false)];
            let (sy, ty) = if up {
                (cf.y, ct.y + ct.h)
            } else {
                (cf.y + cf.h, ct.y)
            };
            match gutter_of.get(&(f, t)) {
                None => {
                    let m = (ty - sy) / 2.0;
                    format!(
                        "M{} {} C{} {} {} {} {} {}",
                        r1(sx),
                        r1(sy),
                        r1(sx),
                        r1(sy + m),
                        r1(tx),
                        r1(ty - m),
                        r1(tx),
                        r1(ty)
                    )
                }
                Some(g) => {
                    let gx = gutter_x(*g) + lane[&(f, t)];
                    let h = if up { -ROW_GAP / 2.0 } else { ROW_GAP / 2.0 };
                    format!(
                        "M{} {} C{} {} {} {} {} {} L{} {} C{} {} {} {} {} {}",
                        r1(sx),
                        r1(sy),
                        r1(sx),
                        r1(sy + h * 0.7),
                        r1(gx),
                        r1(sy + h * 0.3),
                        r1(gx),
                        r1(sy + h),
                        r1(gx),
                        r1(ty - h),
                        r1(gx),
                        r1(ty - h * 0.3),
                        r1(tx),
                        r1(ty - h * 0.7),
                        r1(tx),
                        r1(ty)
                    )
                }
            }
        };
        out.push(Edge {
            from: f,
            to: t,
            count,
            path,
        });
    }
    let lowest_h = if band { BAND_H } else { CARD_H };
    let height = TOP + top as f64 * (CARD_H + ROW_GAP) + lowest_h + BOTTOM;
    Map {
        width: r1(width),
        height: r1(height),
        rows,
        cards,
        edges: out,
        band,
    }
}
