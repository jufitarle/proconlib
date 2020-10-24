    pub mod macros {
        #[macro_export]
        /// Clone trait is required
        macro_rules! swap {
            ($a: expr, $b: expr) => {
                let t = $a.clone();
                $a = $b.clone();
                $b = t;
            };
        }

        #[macro_export]
        macro_rules! min {
            ($a: expr) => {
                $a
            };

            ($a: expr, $b: expr) => {
                std::cmp::min($a, $b)
            };

            ($a: expr, $($rest: expr),+) => {
                std::cmp::min($a, min!($($rest),+))
            };
        }

        #[macro_export]
        macro_rules! max {
            ($a: expr) => {
                $a
            };

            ($a: expr, $b: expr) => {
                std::cmp::max($a, $b)
            };

            ($a: expr, $($rest: expr),+) => {
                std::cmp::max($a, max!($($rest),+))
            };
    }

        #[macro_export]
        macro_rules! chmax {
            ($target: expr , $($other: expr),+) => {
                let max = max!($($other),*);
                if $target < max{
                    $target = max;
                    true
                } else {
                    false
                }
            };
        }

        #[macro_export]
        macro_rules! chmin {
            ($target: expr , $($other: expr),+) => {
                let min = min!($($other),*);
                if min < $target {
                    $target = min;
                    true
                } else {
                    false
                }
            };

        }
    }
