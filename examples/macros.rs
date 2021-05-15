// macros.rs --- Macro Example 


// block
// expr 用于表达式
// ident 用于变量名或函数名
// item
// pat (模式 pattern)
// path
// stmt (语句 statement)
// tt (标记树 token tree)
// ty (类型 type)


macro_rules! say_h {
    () => (
        println!("H!");
    )
}

macro_rules! crt_fn {
    ($func: ident) => {
        fn $func() { 
            println!("call func {:?}", stringify!($func));
        }
    }
}

macro_rules! crt_expr {
    ($expr:expr) => (
        println!("call expr {:?}", stringify!($expr));
    )
}

// 根据你调用它的方式，`test!` 将以不同的方式来比较 `$left` 和 `$right`。
macro_rules! compose {
    // 参数不需要使用逗号隔开。
    // 参数可以任意组合！
    ($left:expr; and $right:expr) => (
        println!("{:?} and {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left && $right)
    );
    // ^ 每个分支都必须以分号结束。
    ($left:expr; or $right:expr) => (
        println!("{:?} or {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left || $right)
    );
}

// `min!` 将求出任意数量的参数的最小值。
macro_rules! min {
    // 基本情形：
    ($x:expr) => ($x);
    // `$x` 后面跟着至少一个 `$y,`
    ($x:expr, $($y:expr),+) => (
        // 对 `$x` 后面的 `$y` 们调用 `min!` 
        std::cmp::min($x, min!($($y),+))
    )
}

macro_rules! calc {
    (eval $e:expr) => {{
         let val: usize = $e;
         println!("{} = {}", stringify!{$e}, val);
    }};

    (eval $e:expr, $(eval $es:expr),+) => {{
        calc! { eval $e }
        calc! { $(eval $es),+ }
    }};
}

fn main() {
    say_h!();
    crt_fn!(foo);
    crt_expr!({ 12 });
    foo();
    compose!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    compose!(true; or false);
    println!("{}", min!(1u32));
    println!("{}", min!(1u32 + 2 , 2u32));
    println!("{}", min!(5u32, 2u32 * 3, 4u32));
    calc! {
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 3) + 1
    }
}
