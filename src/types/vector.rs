// Copyright 2013 The CGMath Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::num;
use std::num::{Zero, zero};
use std::num::{sqrt, atan2};

use traits::alg::*;
use traits::ext::*;
use traits::util::*;

#[deriving(Eq, Zero, Clone)] pub struct Vec1<S> { x: S }
#[deriving(Eq, Zero, Clone)] pub struct Vec2<S> { x: S, y: S }
#[deriving(Eq, Zero, Clone)] pub struct Vec3<S> { x: S, y: S, z: S }
#[deriving(Eq, Zero, Clone)] pub struct Vec4<S> { x: S, y: S, z: S, w: S }
#[deriving(Eq, Zero, Clone)] pub struct Vec5<S> { x: S, y: S, z: S, w: S, a: S }
#[deriving(Eq, Zero, Clone)] pub struct Vec6<S> { x: S, y: S, z: S, w: S, a: S, b: S }

macro_rules! impl_vec(
    ($Self:ident <$S:ident> { $($field:ident),+ }) => (
        impl<$S: Field> $Self<$S> {
            #[inline]
            pub fn new($($field: $S),+) -> $Self<$S> {
                $Self { $($field: $field),+ }
            }

            /// The additive identity of the vector.
            #[inline]
            pub fn zero() -> $Self<$S> { zero() }
        }
    )
)

impl_vec!(Vec1<S> { x })
impl_vec!(Vec2<S> { x, y })
impl_vec!(Vec3<S> { x, y, z })
impl_vec!(Vec4<S> { x, y, z, w })
impl_vec!(Vec5<S> { x, y, z, w, a })
impl_vec!(Vec6<S> { x, y, z, w, a, b })

macro_rules! impl_vec_clonable(
    ($Self:ident <$S:ident>) => (
        impl<$S:Clone + Field> $Self<$S> {
            /// Construct a vector from a single value.
            #[inline]
            pub fn from_value(value: $S) -> $Self<$S> {
                Indexable::build(|_| value.clone())
            }
        }
    )
)

impl_vec_clonable!(Vec1<S>)
impl_vec_clonable!(Vec2<S>)
impl_vec_clonable!(Vec3<S>)
impl_vec_clonable!(Vec4<S>)
impl_vec_clonable!(Vec5<S>)
impl_vec_clonable!(Vec6<S>)

// Operator impls

macro_rules! impl_vec_ops(
    ($vec_ops_mod:ident, $Self:ident <$S:ident>) => (
        pub mod $vec_ops_mod {
            use super::*;
            use super::super::super::traits::alg::*;

            impl_scalar_binop!($Self<$S>, Mul, mul)
            impl_scalar_binop!($Self<$S>, Div, div)
            impl_scalar_binop!($Self<$S>, Rem, rem)
            impl_coordinate_binop!($Self<$S>, $Self<$S>, $Self<$S>, Add, add)
            impl_coordinate_binop!($Self<$S>, $Self<$S>, $Self<$S>, Sub, sub)
            impl_coordinate_op!($Self<$S>, $Self<$S>, Neg, neg)
        }
    )
)

impl_vec_ops!(vec1_ops, Vec1<S>)
impl_vec_ops!(vec2_ops, Vec2<S>)
impl_vec_ops!(vec3_ops, Vec3<S>)
impl_vec_ops!(vec4_ops, Vec4<S>)
impl_vec_ops!(vec5_ops, Vec5<S>)
impl_vec_ops!(vec6_ops, Vec6<S>)

/// Operations specific to two-dimensional vectors.
impl<S: Field> Vec2<S> {
    /// The perpendicular dot product of the vector and `other`.
    pub fn perp_dot(&self, other: &Vec2<S>) -> S {
        (self.x * other.y) - (self.y * other.x)
    }
}

/// Operations specific to three-dimensional vectors.
impl<S: Field> Vec3<S> {
    /// Returns the cross product of the vector and `other`.
    pub fn cross(&self, other: &Vec3<S>) -> Vec3<S> {
        Vec3::new((self.y * other.z) - (self.z * other.y),
                  (self.z * other.x) - (self.x * other.z),
                  (self.x * other.y) - (self.y * other.x))
    }
}

// Trait impls

impl_indexable!(Vec1<T>, [T, ..1])
impl_indexable!(Vec2<T>, [T, ..2])
impl_indexable!(Vec3<T>, [T, ..3])
impl_indexable!(Vec4<T>, [T, ..4])
impl_indexable!(Vec5<T>, [T, ..5])
impl_indexable!(Vec6<T>, [T, ..6])

impl<S: Clone + Field> Swappable<S, [S, ..1]> for Vec1<S>;
impl<S: Clone + Field> Swappable<S, [S, ..2]> for Vec2<S>;
impl<S: Clone + Field> Swappable<S, [S, ..3]> for Vec3<S>;
impl<S: Clone + Field> Swappable<S, [S, ..4]> for Vec4<S>;
impl<S: Clone + Field> Swappable<S, [S, ..5]> for Vec5<S>;
impl<S: Clone + Field> Swappable<S, [S, ..6]> for Vec6<S>;

impl<S: Clone + Field> Coordinate<S, [S, ..1]> for Vec1<S>;
impl<S: Clone + Field> Coordinate<S, [S, ..2]> for Vec2<S>;
impl<S: Clone + Field> Coordinate<S, [S, ..3]> for Vec3<S>;
impl<S: Clone + Field> Coordinate<S, [S, ..4]> for Vec4<S>;
impl<S: Clone + Field> Coordinate<S, [S, ..5]> for Vec5<S>;
impl<S: Clone + Field> Coordinate<S, [S, ..6]> for Vec6<S>;

impl<S: Field> ScalarMul<S> for Vec1<S>;
impl<S: Field> ScalarMul<S> for Vec2<S>;
impl<S: Field> ScalarMul<S> for Vec3<S>;
impl<S: Field> ScalarMul<S> for Vec4<S>;
impl<S: Field> ScalarMul<S> for Vec5<S>;
impl<S: Field> ScalarMul<S> for Vec6<S>;

impl<S: Field> Module<S> for Vec1<S>;
impl<S: Field> Module<S> for Vec2<S>;
impl<S: Field> Module<S> for Vec3<S>;
impl<S: Field> Module<S> for Vec4<S>;
impl<S: Field> Module<S> for Vec5<S>;
impl<S: Field> Module<S> for Vec6<S>;

impl<S: Field> VectorSpace<S> for Vec1<S>;
impl<S: Field> VectorSpace<S> for Vec2<S>;
impl<S: Field> VectorSpace<S> for Vec3<S>;
impl<S: Field> VectorSpace<S> for Vec4<S>;
impl<S: Field> VectorSpace<S> for Vec5<S>;
impl<S: Field> VectorSpace<S> for Vec6<S>;

macro_rules! impl_vec_inner_product(
    ($Self:ident <$S:ident>) => (
        impl<$S:Real + Field + ApproxEq<$S>> InnerProductSpace<$S> for $Self<$S> {
            fn norm(&self) -> $S {
                num::sqrt(self.inner(self))
            }

            fn inner(&self, other: &$Self<$S>) -> $S {
                let comp_sum: $Self<$S> = self.bimap(other, |a, b| a.mul(b));
                comp_sum.fold(num::zero::<$S>(), |a, b| a.add(b))
            }

            fn is_orthogonal(&self, other: &$Self<$S>) -> bool {
                self.inner(other).approx_eq(&num::zero())
            }
        }
    )
)

impl_vec_inner_product!(Vec1<S>)
impl_vec_inner_product!(Vec2<S>)
impl_vec_inner_product!(Vec3<S>)
impl_vec_inner_product!(Vec4<S>)
impl_vec_inner_product!(Vec5<S>)
impl_vec_inner_product!(Vec6<S>)

// Euclidean spaces only really make sense for 2D and 3D vector spaces

impl<S:Real + Field + ApproxEq<S>> EuclideanSpace<S> for Vec2<S> {
    fn angle(&self, other: &Vec2<S>) -> S {
        atan2(self.perp_dot(other), self.dot(other))
    }
}

impl<S:Real + Field + ApproxEq<S>> EuclideanSpace<S> for Vec3<S> {
    fn angle(&self, other: &Vec3<S>) -> S {
        atan2(self.cross(other).length(), self.dot(other))
    }
}

impl<S: Clone + Field> VectorExt<S, [S, ..1]> for Vec1<S>;
impl<S: Clone + Field> VectorExt<S, [S, ..2]> for Vec2<S>;
impl<S: Clone + Field> VectorExt<S, [S, ..3]> for Vec3<S>;
impl<S: Clone + Field> VectorExt<S, [S, ..4]> for Vec4<S>;
impl<S: Clone + Field> VectorExt<S, [S, ..5]> for Vec5<S>;
impl<S: Clone + Field> VectorExt<S, [S, ..6]> for Vec6<S>;