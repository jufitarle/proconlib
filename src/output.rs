    pub mod output {
        pub trait Echo {
            /// ### 改行あり出力  
            /// ``` rust
            /// let abc = "abc";  
            /// abc.echo();  
            /// // expected:
            /// // abc
            ///
            /// let v = vec![1, 2, 3];
            /// v.echo();
            /// // expected:
            /// // 1
            /// // 2
            /// // 3
            /// ```  
            fn echo(&self);

            /// ### 各要素の間に何か挟み、最後に改行をつけて出力  
            /// ``` rust
            /// let abc = "abc";  
            /// abc.echo_with("d");  
            /// // expected: abcd
            ///
            /// let v = vec![1, 2, 3];
            /// v.echo_with(" ");
            /// // expected:
            /// // 1 2 3
            /// ```
            fn echo_with(&self, s: &str);
        }

        impl<T> Echo for T
        where
            T: std::fmt::Display,
        {
            fn echo(&self) {
                println!("{}", self);
            }

            fn echo_with(&self, s: &str) {
                print!("{}{}", self, s);
            }
        }

        impl<T> Echo for [T]
        where
            T: std::fmt::Display,
        {
            fn echo(&self) {
                use itertools::Itertools;
                println!("{}", self.iter().format("\n"));
            }

            fn echo_with(&self, s: &str) {
                use itertools::Itertools;
                println!("{}", self.iter().format(s));
            }
        }
    }
