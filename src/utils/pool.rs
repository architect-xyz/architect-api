use crossbeam::queue::ArrayQueue;
use fxhash::FxHashMap;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use schemars::{schema::InstanceType, JsonSchema};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    any::{Any, TypeId},
    borrow::Borrow,
    cell::RefCell,
    cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
    collections::{HashMap, HashSet, VecDeque},
    default::Default,
    fmt::Debug,
    hash::{BuildHasher, Hash, Hasher},
    ops::{Deref, DerefMut},
    sync::{Arc, Weak},
};
use triomphe::Arc as TArc;

/// pool(vis, name, type, capacity, max_elt_size)
///
/// Create a static memory pool. Objects are taken from the pool or
/// allocated normally if it is empty, when they are dropped instead
/// of being deallocated they are cleared and inserted into the pool,
/// up to capacity elements no more than max_elt_size may be stored in
/// the pool.
#[macro_export]
macro_rules! pool {
    ($vis:vis, $name:ident, $ty:ty, $max_capacity:expr_2021, $max_elt_size:expr_2021) => {
        $vis fn $name() -> &'static api::utils::pool::Pool<$ty> {
            static POOL: once_cell::race::OnceBox<api::utils::pool::Pool<$ty>> = once_cell::race::OnceBox::new();
            POOL.get_or_init(|| Box::new(api::utils::pool::Pool::new($max_capacity, $max_elt_size)))
        }
    };
    ($name:ident, $ty:ty, $max_capacity:expr_2021, $max_elt_size:expr_2021) => {
        fn $name() -> &'static $crate::utils::pool::Pool<$ty> {
            static POOL: once_cell::race::OnceBox<$crate::utils::pool::Pool<$ty>> = once_cell::race::OnceBox::new();
            POOL.get_or_init(|| Box::new($crate::utils::pool::Pool::new($max_capacity, $max_elt_size)))
        }
    }
}

pub trait Poolable {
    fn empty() -> Self;
    fn reset(&mut self);
    fn capacity(&self) -> usize;
    /// in case you are pooling something ref counted e.g. arc
    fn really_dropped(&self) -> bool {
        true
    }
}

macro_rules! impl_hashmap {
    ($ty:ident) => {
        impl<K, V, R> Poolable for $ty<K, V, R>
        where
            K: Hash + Eq,
            R: Default + BuildHasher,
        {
            fn empty() -> Self {
                $ty::default()
            }

            fn reset(&mut self) {
                self.clear()
            }

            fn capacity(&self) -> usize {
                $ty::capacity(self)
            }
        }
    };
}

impl_hashmap!(HashMap);

macro_rules! impl_hashset {
    ($ty:ident) => {
        impl<K, R> Poolable for $ty<K, R>
        where
            K: Hash + Eq,
            R: Default + BuildHasher,
        {
            fn empty() -> Self {
                $ty::default()
            }

            fn reset(&mut self) {
                self.clear()
            }

            fn capacity(&self) -> usize {
                $ty::capacity(self)
            }
        }
    };
}

impl_hashset!(HashSet);

impl<T> Poolable for Vec<T> {
    fn empty() -> Self {
        Vec::new()
    }

    fn reset(&mut self) {
        self.clear()
    }

    fn capacity(&self) -> usize {
        Vec::capacity(self)
    }
}

impl<T> Poolable for VecDeque<T> {
    fn empty() -> Self {
        VecDeque::new()
    }

    fn reset(&mut self) {
        self.clear()
    }

    fn capacity(&self) -> usize {
        VecDeque::capacity(self)
    }
}

impl Poolable for String {
    fn empty() -> Self {
        String::new()
    }

    fn reset(&mut self) {
        self.clear()
    }

    fn capacity(&self) -> usize {
        self.capacity()
    }
}

impl<T: Poolable> Poolable for TArc<T> {
    fn empty() -> Self {
        TArc::new(T::empty())
    }

    fn reset(&mut self) {
        if let Some(inner) = TArc::get_mut(self) {
            inner.reset()
        }
    }

    fn capacity(&self) -> usize {
        1
    }

    fn really_dropped(&self) -> bool {
        TArc::is_unique(self)
    }
}

trait Prune {
    fn prune(&self);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pid(u32);

impl Pid {
    fn new() -> Self {
        use std::sync::atomic::{AtomicU32, Ordering};
        static PID: AtomicU32 = AtomicU32::new(0);
        Self(PID.fetch_add(1, Ordering::Relaxed))
    }
}

struct GlobalState {
    pools: FxHashMap<Pid, Box<dyn Prune + Send + 'static>>,
    pool_shark_running: bool,
}

static POOLS: Lazy<Mutex<GlobalState>> = Lazy::new(|| {
    Mutex::new(GlobalState { pools: HashMap::default(), pool_shark_running: false })
});

#[derive(Debug)]
struct PoolInner<T: Poolable + Send + 'static> {
    pool: ArrayQueue<T>,
    max_elt_capacity: usize,
    id: Pid,
}

fn pool_shark() {
    use std::{
        thread::{sleep, spawn},
        time::Duration,
    };
    spawn(|| loop {
        sleep(Duration::from_secs(300));
        {
            for p in POOLS.lock().pools.values() {
                p.prune()
            }
        }
    });
}

/// a lock-free, thread-safe, dynamically-sized object pool.
///
/// this pool begins with an initial capacity and will continue
/// creating new objects on request when none are available. Pooled
/// objects are returned to the pool on destruction.
///
/// if, during an attempted return, a pool already has
/// `maximum_capacity` objects in the pool, the pool will throw away
/// that object.
#[derive(Clone, Debug)]
pub struct Pool<T: Poolable + Send + 'static>(Arc<PoolInner<T>>);

impl<T: Poolable + Send + 'static> Drop for Pool<T> {
    fn drop(&mut self) {
        // one held by us, and one held by the pool shark
        if Arc::strong_count(&self.0) <= 2 {
            let res = POOLS.lock().pools.remove(&self.0.id);
            drop(res)
        }
    }
}

impl<T: Poolable + Send + 'static> Prune for Pool<T> {
    fn prune(&self) {
        let len = self.0.pool.len();
        let ten_percent = std::cmp::max(1, self.0.pool.capacity() / 10);
        let one_percent = std::cmp::max(1, ten_percent / 10);
        if len > ten_percent {
            for _ in 0..ten_percent {
                self.0.pool.pop();
            }
        } else if len > one_percent {
            for _ in 0..one_percent {
                self.0.pool.pop();
            }
        } else if len > 0 {
            self.0.pool.pop();
        }
    }
}

impl<T: Poolable + Send + 'static> Pool<T> {
    /// creates a new `Pool<T>`. this pool will retain up to
    /// `max_capacity` objects of size less than or equal to
    /// max_elt_capacity. Objects larger than max_elt_capacity will be
    /// deallocated immediatly.
    pub fn new(max_capacity: usize, max_elt_capacity: usize) -> Pool<T> {
        let id = Pid::new();
        let t = Pool(Arc::new(PoolInner {
            pool: ArrayQueue::new(max_capacity),
            max_elt_capacity,
            id,
        }));
        let mut gs = POOLS.lock();
        gs.pools.insert(id, Box::new(Pool(Arc::clone(&t.0))));
        if !gs.pool_shark_running {
            gs.pool_shark_running = true;
            pool_shark()
        }
        t
    }

    /// takes an item from the pool, creating one if none are available.
    pub fn take(&self) -> Pooled<T> {
        let object = self.0.pool.pop().unwrap_or_else(Poolable::empty);
        Pooled { pool: Arc::downgrade(&self.0), object: Some(object) }
    }
}

/// an object, checked out from a pool.
#[derive(Debug, Clone)]
pub struct Pooled<T: Poolable + Send + 'static> {
    pool: Weak<PoolInner<T>>,
    // Safety invariant. This will always be Some unless the
    // pooled has been dropped
    object: Option<T>,
}

impl<T: Poolable + Send + 'static> Pooled<T> {
    #[inline(always)]
    fn get(&self) -> &T {
        match &self.object {
            Some(t) => t,
            None => unreachable!(),
        }
    }

    #[inline(always)]
    fn get_mut(&mut self) -> &mut T {
        match &mut self.object {
            Some(t) => t,
            None => unreachable!(),
        }
    }
}

impl<T: Poolable + Sync + Send + 'static> Borrow<T> for Pooled<T> {
    fn borrow(&self) -> &T {
        self.get()
    }
}

impl Borrow<str> for Pooled<String> {
    fn borrow(&self) -> &str {
        self.get().borrow()
    }
}

impl<T: Poolable + Send + 'static + PartialEq> PartialEq for Pooled<T> {
    fn eq(&self, other: &Pooled<T>) -> bool {
        self.get().eq(other.get())
    }
}

impl<T: Poolable + Send + 'static + Eq> Eq for Pooled<T> {}

impl<T: Poolable + Send + 'static + PartialOrd> PartialOrd for Pooled<T> {
    fn partial_cmp(&self, other: &Pooled<T>) -> Option<Ordering> {
        self.get().partial_cmp(other.get())
    }
}

impl<T: Poolable + Send + 'static + Ord> Ord for Pooled<T> {
    fn cmp(&self, other: &Pooled<T>) -> Ordering {
        self.get().cmp(other.get())
    }
}

impl<T: Poolable + Send + 'static + Hash> Hash for Pooled<T> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        Hash::hash(self.get(), state)
    }
}

impl<T: Poolable + Send + 'static> Pooled<T> {
    /// Creates a `Pooled` that isn't connected to any pool. E.G. for
    /// branches where you know a given `Pooled` will always be empty.
    pub fn orphan(t: T) -> Self {
        Pooled { pool: Weak::new(), object: Some(t) }
    }

    pub fn detach(mut self) -> T {
        self.object.take().unwrap()
    }
}

impl<T: Poolable + Send + 'static> AsRef<T> for Pooled<T> {
    fn as_ref(&self) -> &T {
        self.get()
    }
}

impl<T: Poolable + Send + 'static> Deref for Pooled<T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.get()
    }
}

impl<T: Poolable + Send + 'static> DerefMut for Pooled<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}

impl<T: Poolable + Send + 'static> Drop for Pooled<T> {
    fn drop(&mut self) {
        if self.get().really_dropped() {
            if let Some(inner) = self.pool.upgrade() {
                let cap = self.get().capacity();
                if cap > 0 && cap <= inner.max_elt_capacity {
                    let mut object = self.object.take().unwrap();
                    object.reset();
                    inner.pool.push(object).ok();
                }
            }
        }
    }
}

impl<T: Poolable + Send + 'static + Serialize> Serialize for Pooled<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.get().serialize(serializer)
    }
}

impl<'de, T: Poolable + Send + 'static + DeserializeOwned> Deserialize<'de>
    for Pooled<T>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut t = take_t::<T>(1000, 1000);
        Self::deserialize_in_place(deserializer, &mut t)?;
        Ok(t)
    }

    fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        <T as Deserialize>::deserialize_in_place(deserializer, place.get_mut())
    }
}
thread_local! {
    static POOLS_S: RefCell<FxHashMap<TypeId, Box<dyn Any>>> =
        RefCell::new(HashMap::default());
}

/// Take a poolable type T from the generic thread local pool set.
/// Note it is much more efficient to construct your own pools.
/// size and max are the pool parameters used if the pool doesn't
/// already exist.
pub fn take_t<T: Any + Poolable + Send + 'static>(size: usize, max: usize) -> Pooled<T> {
    POOLS_S.with(|pools| {
        let mut pools = pools.borrow_mut();
        let pool: &mut Pool<T> = pools
            .entry(TypeId::of::<T>())
            .or_insert_with(|| Box::new(Pool::<T>::new(size, max)))
            .downcast_mut()
            .unwrap();
        pool.take()
    })
}

impl<T: Poolable + Send + 'static + JsonSchema> JsonSchema for Pooled<T> {
    fn schema_name() -> String {
        // Exclude the module path to make the name in generated schemas clearer.
        "Pooled".to_owned()
    }

    fn json_schema(
        _gen: &mut schemars::r#gen::SchemaGenerator,
    ) -> schemars::schema::Schema {
        // FIXME probably
        schemars::schema::SchemaObject {
            instance_type: Some(InstanceType::Array.into()),
            ..Default::default()
        }
        .into()
    }

    fn is_referenceable() -> bool {
        true
    }
}
