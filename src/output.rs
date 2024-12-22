

#[macro_export]
macro_rules! printip {
    ($name:expr, $ttl:expr, $class:expr, $tpe:expr, $ip:expr) => {
        println!("{0: <10} | {1} | {2: <10} | {3: <10} | {4: <10}", $name, $ttl, $class, $tpe, $ip);
    };
}