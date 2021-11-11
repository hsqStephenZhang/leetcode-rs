#![feature(map_first_last)]

pub mod utils;

#[allow(
    dead_code,
    unused_variables,
    unused_mut,
    unreachable_code,
    unused_imports
)]
pub mod leetcode;

#[allow(
    dead_code,
    unused_variables,
    unused_mut,
    unreachable_code,
    unused_imports
)]
pub mod offer;

#[allow(
    dead_code,
    unused_variables,
    unused_mut,
    unreachable_code,
    unused_imports
)]
pub mod offer2;

pub mod categories {
    pub mod topology {
        pub use crate::leetcode::leetcode207 as curriculum1;
        pub use crate::leetcode::leetcode210 as curriculum2;
        pub use crate::leetcode::leetcode802 as curriculum3;
    }

    pub mod dynamic_programming {}

    pub mod prefix {}

    pub mod string {}

    pub mod hash {}

    pub mod stack {}

    pub mod backtrace {
        pub use crate::leetcode::leetcode131;
        pub use crate::leetcode::leetcode216;
        pub use crate::leetcode::leetcode39;
        pub use crate::leetcode::leetcode40;
        pub use crate::leetcode::leetcode46;
        pub use crate::leetcode::leetcode47;
        pub use crate::leetcode::leetcode51;
        pub use crate::leetcode::leetcode77;
        pub use crate::leetcode::leetcode78;
        pub use crate::leetcode::leetcode90;
        pub use crate::leetcode::leetcode93;
    }

    pub mod greedy {
        pub use crate::leetcode::leetcode45;
        pub use crate::leetcode::leetcode53;
        pub use crate::leetcode::leetcode55;
    }
}
