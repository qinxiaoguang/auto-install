// 自用macros

// future相关macros
pub mod futures {
    // 等待所有task
    #[macro_export]
    macro_rules! join_all {
        ( $tasks:expr ) => {
            for t in $tasks {
                t.await;
            }
        };
    }
}
