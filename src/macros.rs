use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "full")] {
        
        #[macro_export]
        macro_rules! vector {
            //no arguments case
            () => {
                $crate::vectors::Vector::from(vec![])
            };

            //repeat some elements some n times
            ($x:expr; $n:expr) => {
                $crate::vectors::Vector::from(vec![$x; $n])
            };
            
            //match each comma-separated argument
            //and allow the last comma to be ignored
            ($($x:expr),*) => {
                $crate::vectors::Vector::from(vec![$($x),*])
            };

            //match each comma-separated argument
            //but an unneccesary comma was used at the end
            ($($x:expr,)*) => {
                $crate::vectors::Vector::from(vec![$($x),*])
            }
        }

    } else if #[cfg(feature = "no_std" )] {
        #[macro_export]
        macro_rules! vector {
            //no arguments case
            () => {
                $crate::vectors::VectorSlice::from([].as_slice())
            };
    
            //repeat some elements some n times
            ($x:expr; $n:expr) => {
                $crate::vectors::VectorSlice::from([$x; $n].as_slice())
            };
            
            //match each comma-separated argument
            //and allow the last comma to be ignored
            ($($x:expr),*) => {
                $crate::vectors::VectorSlice::from([$($x),*].as_slice())
            };
    
            //match each comma-separated argument
            //but an unneccesary comma was used at the end
            ($($x:expr,)*) => {
                $crate::vectors::VectorSlice::from([$($x),*].as_slice())
            }
        }
    }
}