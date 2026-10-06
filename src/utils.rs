// SPDX-License-Identifier: MPIT

//! Utility functions for handling data in this library.

use crate::reexports::iced::core::{
    layout::{Limits, Node},
    Point, Size,
};
use cosmic::anim::lerp;
use cosmic::iced::core::{Background, Color};

/// Collect iterator into static array without panicking or collecting into a Vec.
///
/// Initializes with `T::default()`, then takes `SIZE` values from the iterator.
#[allow(dead_code)]
pub fn static_array_from_iter<T: Copy + Default, const SIZE: usize>(
    iter: impl Iterator<Item = T>,
) -> [T; SIZE] {
    let mut array = [T::default(); SIZE];

    for (id, value) in iter.take(SIZE).enumerate() {
        array[id] = value;
    }

    array
}

/// Produces a [`Node`] with two children nodes one right next to each other.
pub fn next_to_each_other(
    limits: &Limits,
    spacing: f32,
    left: impl FnOnce(&Limits) -> Node,
    right: impl FnOnce(&Limits) -> Node,
) -> Node {
    let mut right_node = right(limits);
    let right_size = right_node.size();

    let left_limits = limits.shrink(Size::new(right_size.width + spacing, 0.0));
    let mut left_node = left(&left_limits);
    let left_size = left_node.size();

    let (left_y, right_y) = if left_size.height > right_size.height {
        (0.0, (left_size.height - right_size.height) / 2.0)
    } else {
        ((right_size.height - left_size.height) / 2.0, 0.0)
    };

    left_node = left_node.move_to(Point::new(0.0, left_y));
    right_node = right_node.move_to(Point::new(left_size.width + spacing, right_y));

    Node::with_children(
        Size::new(
            left_size.width + spacing + right_size.width,
            left_size.height.max(right_size.height),
        ),
        vec![left_node, right_node],
    )
}

pub fn blend_background(first: Background, other: Background, percent: f32) -> Background {
    match (first, other) {
        (Background::Color(c1), Background::Color(c2)) => {
            let [r1, g1, b1, a1] = c1.into_linear();
            let [r2, g2, b2, a2] = c2.into_linear();

            let blended = Color::from_linear_rgba(
                lerp(r1, r2, percent),
                lerp(g1, g2, percent),
                lerp(b1, b2, percent),
                lerp(a1, a2, percent),
            );

            Background::Color(blended)
        }
        (first, other) => {
            if percent < 0.5 {
                first
            } else {
                other
            }
        }
    }
}
