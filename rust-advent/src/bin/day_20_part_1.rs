#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::fs;

use anyhow::Context;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Element {
    value: i16,
    initial_position: usize,
}

#[derive(Debug)]
struct MovedElement {
    #[allow(dead_code)]
    element: Element,
    current_position: usize,
    new_position: usize,
}

impl MovedElement {
    fn new(initial_position: usize, values: &[Element]) -> anyhow::Result<Self> {
        let current_position = values
            .iter()
            .position(|e| e.initial_position == initial_position)
            .with_context(|| {
                format!("Failed to find element with initial position {initial_position}.")
            })?;
        let element = values
            .get(current_position)
            .with_context(|| format!("Retrieving element at index {current_position} failed"))?;

        let length = values.len();
        let length_i16 = i16::try_from(length)?;
        let offset = usize::try_from(element.value.rem_euclid(length_i16 - 1))?;

        let new_position = if current_position + offset < length {
            current_position + offset
        } else {
            current_position + offset - (length - 1)
        };

        Ok(Self {
            element: *element,
            current_position,
            new_position,
        })
    }

    #[allow(dead_code)]
    const fn value(&self) -> i16 {
        self.element.value
    }
}

fn mix(values: &mut Vec<Element>) -> anyhow::Result<()> {
    for i in 0..values.len() {
        move_element(values, i)?;
        // let vals: Vec<i16> = values.iter().map(|e| e.value).collect();
        // println!("Current vals = {vals:?}");
    }
    Ok(())
}

/*
 * If value is non-negative and adding value and the position < length,
 *    then we rotate_left with the slice [pos..pos+val]
 * If value is negative and adding val and pos < 0,
 *    then we rotate_left with the slice [pos..pos+val+len]
 *
 * If value is non-negative and adding value and position >= length,
 *    then we rotate_right with the slice [pos+val-len..pos]
 * If value is negative and adding val and pos >= 0,
 *    then we rotate_right with the slice [pos+val..pos]
 *
 * Make sure handle value = 0.
 */
fn move_element(values: &mut [Element], i: usize) -> anyhow::Result<()> {
    let moved_element = MovedElement::new(i, values)?;

    let current_position = moved_element.current_position;
    let new_position = moved_element.new_position;

    match current_position.cmp(&new_position) {
        std::cmp::Ordering::Equal => {}
        std::cmp::Ordering::Less => values[current_position..=new_position].rotate_left(1),
        std::cmp::Ordering::Greater => values[new_position..=current_position].rotate_right(1),
    }
    Ok(())
}

fn compute_result(values: &Vec<Element>) -> anyhow::Result<i16> {
    let length = values.len();
    let zero_position = values
        .iter()
        .position(|e| e.value == 0)
        .with_context(|| "Failed to find element with value 0.")?;
    println!(
        "Position of zero is {zero_position}, with element {:?}",
        values[zero_position]
    );

    Ok([1000, 2000, 3000]
        .iter()
        .map(|offset| {
            let i = (zero_position + offset) % length;
            println!(
                "Offset {offset} with position {i} and value {}",
                values[i].value
            );
            values[i].value
        })
        .sum())
}

static INPUT_FILE: &str = "../inputs/day_20.input";

fn main() -> anyhow::Result<()> {
    let mut values: Vec<Element> = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?
        .trim()
        .lines()
        .map(str::parse)
        .enumerate()
        .map(|(i, r)| {
            Ok(Element {
                value: r?,
                initial_position: i,
            })
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    mix(&mut values)?;

    println!("After mixing: {values:?}");

    let result = compute_result(&values);

    println!("The result is {result:?}");

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{Element, MovedElement};

    #[test]
    #[allow(clippy::unwrap_used)]
    fn zero_does_not_move() {
        let (vec, index) = (
            [
                Element {
                    value: 0,
                    initial_position: 1,
                },
                Element {
                    value: -1,
                    initial_position: 0,
                },
            ],
            1,
        );
        let moved_element = MovedElement::new(index, &vec).unwrap();
        assert_eq!(moved_element.new_position, 0);
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn two_element_list_does_not_change() {
        let (vec, index) = (
            [
                Element {
                    value: 0,
                    initial_position: 1,
                },
                Element {
                    value: -1,
                    initial_position: 0,
                },
            ],
            0,
        );
        let moved_element = MovedElement::new(index, &vec).unwrap();
        assert_eq!(moved_element.new_position, 1);
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn two_wraps_back_same_place() {
        let (vec, index) = (
            [
                Element {
                    value: 2,
                    initial_position: 0,
                },
                Element {
                    value: 0,
                    initial_position: 1,
                },
            ],
            0,
        );
        let moved_element = MovedElement::new(index, &vec).unwrap();
        assert_eq!(moved_element.new_position, 0);
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn two_maps_to_one() {
        let (vec, index) = (
            [
                Element {
                    value: 0,
                    initial_position: 0,
                },
                Element {
                    value: 1,
                    initial_position: 1,
                },
                Element {
                    value: 2,
                    initial_position: 2,
                },
                Element {
                    value: 3,
                    initial_position: 3,
                },
            ],
            2,
        );
        let moved_element = MovedElement::new(index, &vec).unwrap();
        assert_eq!(moved_element.new_position, 1);
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn simple_wrapping() {
        let vec = [
            Element {
                value: 1,
                initial_position: 0,
            },
            Element {
                value: 2,
                initial_position: 1,
            },
            Element {
                value: 7,
                initial_position: 2,
            },
            Element {
                value: -2,
                initial_position: 3,
            },
            Element {
                value: 0,
                initial_position: 4,
            },
            Element {
                value: 4,
                initial_position: 5,
            },
        ];
        let index = 2;
        let moved_element = MovedElement::new(index, &vec).unwrap();
        assert_eq!(moved_element.new_position, 4);
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn moves_to_right_end() {
        let vec = [
            Element {
                value: 1,
                initial_position: 0,
            },
            Element {
                value: 2,
                initial_position: 1,
            },
            Element {
                value: 3,
                initial_position: 2,
            },
            Element {
                value: -2,
                initial_position: 3,
            },
            Element {
                value: 0,
                initial_position: 4,
            },
            Element {
                value: 4,
                initial_position: 5,
            },
        ];
        let index = 2;
        let moved_element = MovedElement::new(index, &vec).unwrap();
        assert_eq!(moved_element.new_position, 5);
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn wrapping_moves_to_the_left() {
        let vec = [
            Element {
                value: 1,
                initial_position: 0,
            },
            Element {
                value: 2,
                initial_position: 1,
            },
            Element {
                value: -3,
                initial_position: 2,
            },
            Element {
                value: -2,
                initial_position: 3,
            },
            Element {
                value: 8,
                initial_position: 4,
            },
            Element {
                value: 4,
                initial_position: 5,
            },
        ];
        let index = 4;
        let moved_element = MovedElement::new(index, &vec).unwrap();
        assert_eq!(moved_element.new_position, 2);
    }
}

#[cfg(test)]
mod proptest_tests {
    use std::iter::once;

    use itertools::Itertools;
    use proptest::test_runner::Config;
    use proptest::{prelude::*, prop_compose, strategy::Just};

    use crate::{Element, MovedElement};

    prop_compose! {
        fn vec_of_elements()(elements in prop::collection::vec(-100i16..100i16, 2..50)
            .prop_map(|values| {
                values.into_iter()
                    .chain(once(0))
                    .unique()
                    .enumerate()
                    .map(|(initial_position, value)| Element { value, initial_position })
                    .collect::<Vec<Element>>()
            }).prop_shuffle()
            .prop_filter("Lists must have at least two elements", |elements| elements.len() > 1)
        ) -> Vec<Element> {
            // prop_assume!(elements.len() > 1);
            elements
        }
    }

    prop_compose! {
        fn vec_and_index()(vec in vec_of_elements())
            (index in 0..vec.len(), vec in Just(vec)) -> (Vec<Element>, usize) {
                (vec, index)
            }
    }

    proptest! {
        #![proptest_config(Config { max_shrink_iters: 10_000, ..Config::default() })]

        #[test]
        fn moved_element_new_does_not_fail((vec, index) in vec_and_index()) {
            let _ = MovedElement::new(index, &vec);
        }

        #[test]
        #[allow(clippy::unwrap_used)]
        fn new_position_has_correct_element((vec, index) in vec_and_index()) {
            let moved_element = MovedElement::new(index, &vec).unwrap();

            prop_assert_eq!(
                vec[moved_element.current_position],
                moved_element.element
            );
        }

        #[test]
        #[allow(clippy::unwrap_used)]
        fn new_position_is_correct((vec, index) in vec_and_index()) {
            let moved_element = MovedElement::new(index, &vec).unwrap();

            let current_position = i16::try_from(moved_element.current_position)?;
            let num_elements_i16 = i16::try_from(vec.len())?;

            let offset = moved_element.value().rem_euclid(num_elements_i16 - 1);
            let mut new_position = current_position + offset;
            if new_position >= num_elements_i16 {
                new_position -= num_elements_i16 - 1;
            }

            prop_assert_eq!(
                new_position,
                i16::try_from(moved_element.new_position)?
            );
        }
    }
}
