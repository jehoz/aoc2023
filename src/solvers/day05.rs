use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, newline, space1},
    combinator::{map_opt, map_res},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

use crate::solvers::Solution;

pub fn solve(input: &str) -> Solution {
    let (input, seeds) = parse_seeds(input).unwrap();
    let (_, mappings) =
        separated_list1(tuple((newline, newline)), parse_mapping)(input.trim()).unwrap();

    let part1 = {
        let single_seeds = CategoryMap::init(seeds.iter().map(|s| Range::singleton(s)).collect());
        let res = mappings.iter().fold(single_seeds, |m1, m2| m1.compose(&m2));
        res.range_maps
            .iter()
            .map(|r| r.output_start)
            .reduce(u64::min)
            .unwrap()
            .to_string()
    };

    let part2 = {
        let seed_ranges = CategoryMap::init(expand_seed_ranges(seeds));
        let res = mappings.iter().fold(seed_ranges, |m1, m2| m1.compose(&m2));
        res.range_maps
            .iter()
            .map(|r| r.output_start)
            .reduce(u64::min)
            .unwrap()
            .to_string()
    };

    Solution { part1, part2 }
}

struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn make(start: u64, end: u64) -> Option<Self> {
        if start < end {
            Some(Range { start, end })
        } else {
            None
        }
    }

    fn singleton(n: &u64) -> Self {
        Range {
            start: n.clone(),
            end: n + 1,
        }
    }

    fn intersect(self, r2: Range) -> Option<Range> {
        Range::make(u64::max(self.start, r2.start), u64::min(self.end, r2.end))
    }
}

#[derive(Clone)]
struct RangeMapping {
    input_start: u64,
    output_start: u64,
    length: u64,
}

impl RangeMapping {
    fn lift(r: &Range) -> Self {
        RangeMapping {
            input_start: r.start,
            output_start: r.start,
            length: r.end - r.start,
        }
    }

    fn input_range(&self) -> Range {
        Range {
            start: self.input_start,
            end: self.input_start + self.length,
        }
    }

    fn output_range(&self) -> Range {
        Range {
            start: self.output_start,
            end: self.output_start + self.length,
        }
    }

    fn compose_intersection(
        &self,
        next: &RangeMapping,
    ) -> (Option<RangeMapping>, Vec<RangeMapping>) {
        match self.output_range().intersect(next.input_range()) {
            Some(inter) => {
                let composed = {
                    let in_offset = inter.start - self.output_start;
                    let out_offset = inter.start - next.input_start;
                    RangeMapping {
                        input_start: self.input_start + in_offset,
                        output_start: next.output_start + out_offset,
                        length: inter.end - inter.start,
                    }
                };
                let rest = vec![
                    Range::make(self.output_start, inter.start),
                    Range::make(inter.end, self.output_start + self.length),
                ]
                .iter()
                .flatten()
                .map(|r| {
                    let offset = r.start - self.output_start;
                    RangeMapping {
                        input_start: self.input_start + offset,
                        output_start: self.output_start + offset,
                        length: r.end - r.start,
                    }
                })
                .collect();

                return (Some(composed), rest);
            }

            None => return (None, vec![self.clone()]),
        }
    }
}

struct CategoryMap {
    range_maps: Vec<RangeMapping>,
}

impl CategoryMap {
    fn init(ranges: Vec<Range>) -> Self {
        CategoryMap {
            range_maps: ranges.iter().map(RangeMapping::lift).collect(),
        }
    }

    fn compose(&self, cmap: &CategoryMap) -> CategoryMap {
        let mut unmapped = self.range_maps.clone();
        let mut mapped = Vec::new();
        for rmap_out in &cmap.range_maps {
            unmapped = unmapped
                .iter()
                .map(|rmap| {
                    let (composed, rest) = rmap.compose_intersection(&rmap_out);
                    if let Some(c) = composed {
                        mapped.push(c);
                    }
                    rest
                })
                .flatten()
                .collect()
        }

        mapped.append(&mut unmapped);
        mapped.sort_by(|a, b| a.input_start.cmp(&b.input_start));
        CategoryMap { range_maps: mapped }
    }
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(
        tag("seeds: "),
        separated_list1(space1, map_res(digit1, |n: &str| n.parse::<u64>())),
    )(input)
}

fn parse_range(input: &str) -> IResult<&str, RangeMapping> {
    map_opt(
        separated_list1(space1, map_res(digit1, |s: &str| s.parse::<u64>())),
        |ns| {
            Some(RangeMapping {
                output_start: ns.get(0).cloned()?,
                input_start: ns.get(1).cloned()?,
                length: ns.get(2).cloned()?,
            })
        },
    )(input)
}

fn parse_mapping(input: &str) -> IResult<&str, CategoryMap> {
    let (input, _) = tuple((alpha1, tag("-to-"), alpha1, tag(" map:\n")))(input)?;
    map_opt(separated_list1(newline, parse_range), |ranges| {
        Some(CategoryMap { range_maps: ranges })
    })(input)
}

fn expand_seed_ranges(seed_nums: Vec<u64>) -> Vec<Range> {
    let mut ranges = Vec::new();

    for i in (0..seed_nums.len()).step_by(2) {
        let start = seed_nums[i];
        let end = start + seed_nums[i + 1];
        ranges.push(Range { start, end });
    }

    ranges
}
