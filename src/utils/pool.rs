// CR alee: move to sdk?
// CR estokes: move to netidx, eliminate lazy_static from netidx, and
// #![forbid(unsafe_code)] test if the performance is as good as
// lazy_static (it should be since once_cell is based on lazy_static)
/// static pooled objects to avoid allocation
///
/// pool(vis, name, type, capacity, max_elt_size)
///
/// Create a static memory pool. Objects are taken from the pool or
/// allocated normally if it is empty, when they are dropped instead
/// of being deallocated they are cleared and inserted into the pool,
/// up to capacity elements no more than max_elt_size may be stored in
/// the pool.
#[macro_export]
macro_rules! pool {
    ($vis:vis, $name:ident, $ty:ty, $max_capacity:expr, $max_elt_size:expr) => {
        $vis fn $name() -> &'static netidx::pool::Pool<$ty> {
            static POOL: once_cell::race::OnceBox<netidx::pool::Pool<$ty>> = once_cell::race::OnceBox::new();
            POOL.get_or_init(|| Box::new(netidx::pool::Pool::new($max_capacity, $max_elt_size)))
        }
    };
    ($name:ident, $ty:ty, $max_capacity:expr, $max_elt_size:expr) => {
        fn $name() -> &'static netidx::pool::Pool<$ty> {
            static POOL: once_cell::race::OnceBox<netidx::pool::Pool<$ty>> = once_cell::race::OnceBox::new();
            POOL.get_or_init(|| Box::new(netidx::pool::Pool::new($max_capacity, $max_elt_size)))
        }
    }
}
