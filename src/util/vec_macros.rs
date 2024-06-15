#[macro_use]
pub mod vec_macros {
    #[macro_export]
    macro_rules! impl_binary_ops {
        ($type:ident, $trait:ident, $fn:ident, $op:tt $(, $field:ident)*) => {
            impl std::ops::$trait<$type> for $type {
                type Output = $type;

                fn $fn(self, _rhs: $type) -> $type {
                    $type {
                        $(
                            $field: self.$field $op _rhs.$field,
                        )*
                    }
                }
            }
        };
    }

    #[macro_export]
    macro_rules! impl_binary_assign_ops {
        ($type:ident, $trait:ident, $fn:ident, $op:tt $(, $field:ident)*) => {
            impl std::ops::$trait<$type> for $type {
                fn $fn(&mut self, other: $type) {
                    $(self.$field $op other.$field;)*
                }
            }
        };
    }

    #[macro_export]
    macro_rules! impl_neg {
        ($type:ident $(, $field:ident)*) => {
            impl std::ops::Neg for $type {
                type Output = $type;

                fn neg(self) -> $type {
                    $type {
                        $(
                            $field: -self.$field,
                        )*
                    }
                }
            }
        };
    }
    #[macro_export]
    macro_rules! impl_unary {
        ($fn:ident, $type:ident $(, $field:ident)*) => {
            impl $type {
                pub fn $fn(self) -> $type {
                    $type {
                        $(
                            $field: self.$field.$fn(),
                        )*
                    }
                }
            }
        };
    }
    #[macro_export]
    macro_rules! impl_partial_eq {
        ($type:ident $(, $field:ident)*) => {
            impl PartialEq for $type {
                fn eq(&self, other: &Self) -> bool {
                    $(
                        // Check equality of each field
                        if self.$field != other.$field {
                            return false;
                        }
                    )*
                    true
                }
            }
        };
    }
    #[macro_export]
    macro_rules! impl_partial_ord {
        ($type:ident $(, $field:ident)*) => {
            impl std::cmp::PartialOrd for $type {
                fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                    let mut ordering = std::cmp::Ordering::Equal;

                    $(
                        // Compare each field and update the ordering if necessary
                        if self.$field > other.$field {
                            if ordering == std::cmp::Ordering::Less {
                                return None; // Inconsistent ordering
                            }
                            ordering = std::cmp::Ordering::Greater;
                        } else if self.$field < other.$field {
                            if ordering == std::cmp::Ordering::Greater {
                                return None; // Inconsistent ordering
                            }
                            ordering = std::cmp::Ordering::Less;
                        }
                    )*

                    Some(ordering)
                }
            }
        };
    }
    #[macro_export]
    macro_rules! impl_vec_by_scalar_ops {
        ($vector_type:ident, $scalar_type:ident, $trait:ident, $fn:ident, $op:tt $(, $field:ident)*) => {
            impl std::ops::$trait<$scalar_type> for $vector_type {
                type Output = $vector_type;

                fn $fn(self, scalar: $scalar_type) -> $vector_type {
                    $vector_type {
                        $(
                            $field: self.$field $op scalar,
                        )*
                    }
                }
            }
        };
    }
    #[macro_export]
    macro_rules! impl_scalar_by_vec_ops {
        ($vector_type:ident, $scalar_type:ident, $trait:ident, $fn:ident, $op:tt $(, $field:ident)*) => {
            impl std::ops::$trait<$vector_type> for $scalar_type {
                type Output = $vector_type;

                fn $fn(self, vector: $vector_type) -> $vector_type {
                    $vector_type {
                        $(
                            $field: self $op vector.$field,
                        )*
                    }
                }
            }
        };
    }
    #[macro_export]
    macro_rules! impl_lerp {
        ($type:ident, $scalar_type:ident $(, $field:ident)*) => {
            impl super::lerp::Lerp for $type {
                fn lerp(self, other: Self, t: f32) -> Self {
                    $type {
                        $(
                            $field: (self.$field as f32 * (1.0 - t) + other.$field as f32 * t) as $scalar_type,
                        )*
                    }
                }
            }
        };
    }
    #[macro_export]
    // macro_rules! impl_dot_product {
    //     ($type:ident, $scalar_type:ident, $( $field:ident ),*) => {
    //         impl $type {
    //             pub fn dot(&self, other: &$type) -> $scalar_type {
    //                 $(
    //                     self.$field * other.$field +
    //                 )*
    //                 0
    //             }
    //         }
    //     };
    // }

    #[macro_export]
    macro_rules! impl_dot_product_inner {
        ($self:ident, $other:ident, $field:ident, $($fields:ident),*) => {
            $self.$field * $other.$field + $crate::impl_dot_product_inner!($self, $other, $($fields),*)
        };
        ($self:ident, $other:ident, $field:ident) => {
            $self.$field * $other.$field
        };
    }
    #[macro_export]
    macro_rules! impl_dot_product {
        ($type:ident, $scalar_type:ident, $( $field:ident ),*) => {
            impl $type {
                pub fn dot(&self, other: &$type) -> $scalar_type {
                    $crate::impl_dot_product_inner!(self, other, $( $field ),*)
                }
            }
        };
    }
    #[macro_export]
    macro_rules! impl_norm {
        ($type:ident, $scalar_type:ident $(, $field:ident)*) => {
            impl $type {
                pub fn norm(&self) -> $scalar_type {
                    (self.$($field.powi(2) + )* 0).sqrt()
                }
            }
        };
    }
    #[macro_export]
    macro_rules! impl_vec {
        ($type:ident, $scalar_type:ident, $( $field:ident ),* ) => {
            $crate::impl_binary_ops!($type, Add, add, + $(, $field)*);
            $crate::impl_binary_ops!($type, Sub, sub, - $(, $field)*);
            $crate::impl_binary_ops!($type, Mul, mul, * $(, $field)*);
            $crate::impl_binary_ops!($type, Div, div, / $(, $field)*);
            $crate::impl_binary_ops!($type, Rem, rem, % $(, $field)*);
            $crate::impl_binary_assign_ops!($type, AddAssign, add_assign, += $(, $field)*);
            $crate::impl_binary_assign_ops!($type, SubAssign, sub_assign, -= $(, $field)*);
            $crate::impl_binary_assign_ops!($type, MulAssign, mul_assign, *= $(, $field)*);
            $crate::impl_binary_assign_ops!($type, DivAssign, div_assign, /= $(, $field)*);
            $crate::impl_binary_assign_ops!($type, RemAssign, rem_assign, %= $(, $field)*);
            $crate::impl_neg!($type $(, $field)*);
            // $crate::impl_unary!(fract, $type $(, $field)*);
            $crate::impl_unary!(abs, $type $(, $field)*);
            $crate::impl_partial_eq!($type $(, $field)*);
            $crate::impl_partial_ord!($type $(, $field)*);
            $crate::impl_scalar_by_vec_ops!($type, $scalar_type, Mul, mul, * $(, $field)*);
            $crate::impl_vec_by_scalar_ops!($type, $scalar_type, Mul, mul, * $(, $field)*);
            $crate::impl_vec_by_scalar_ops!($type, $scalar_type, Div, div, / $(, $field)*);
            $crate::impl_lerp!($type, $scalar_type $(, $field)*);
            $crate::impl_dot_product!($type, $scalar_type, $( $field ),*);
        };
    }
}

// needs to like impl lerp etc
