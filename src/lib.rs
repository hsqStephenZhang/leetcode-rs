#[allow(dead_code)]

pub mod leetcode {
    pub mod leetcode1004;
    pub mod leetcode1143;
    pub mod leetcode122;
    pub mod leetcode1423;
    pub mod leetcode165;
    pub mod leetcode1658;
    pub mod leetcode1871;
    pub mod leetcode207;
    pub mod leetcode209;
    pub mod leetcode210;
    pub mod leetcode219;
    pub mod leetcode220;
    pub mod leetcode29;
    pub mod leetcode300;
    pub mod leetcode304;
    pub mod leetcode306;
    pub mod leetcode316;
    pub mod leetcode321;
    pub mod leetcode329;
    pub mod leetcode402;
    pub mod leetcode416;
    pub mod leetcode494;
    pub mod leetcode523;
    pub mod leetcode567;
    pub mod leetcode581;
    pub mod leetcode713;
    pub mod leetcode718;
    pub mod leetcode739;
    pub mod leetcode802;
    pub mod leetcode91;
    pub mod leetcode901;
    pub mod leetcode912;
    pub mod leetcode93;
    pub mod leetcode930;
}

#[allow(dead_code)]
pub mod offer {
    pub mod offer013;
}

pub mod kinds {

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
}
