use criterion::{criterion_group, criterion_main, Criterion};
use fxhash::FxHashMap;
use rand::Rng;
use std::hint::black_box;
use uuid::Uuid;

fn build_simple_oid_table(n: usize) -> FxHashMap<u64, u64> {
    // create a hashmap of size n filled with random data
    let mut table = FxHashMap::default();
    let mut rng = rand::thread_rng();
    for i in 0..n {
        // order ids in order, which is reasonable-ish workload
        // generate random value
        table.insert(i as u64, rng.gen());
    }
    table
}

fn simple_oid_lookup(table: &FxHashMap<u64, u64>, k: &u64) -> Option<u64> {
    table.get(k).copied()
}

// fn build_uuid_oid_table(n: usize) -> (FxHashMap<Uuid, u64>, Uuid) {
//     let mut table = FxHashMap::default();
//     let mut rng = rand::thread_rng();
//     let guaranteed_hit = Uuid::new_v4();
//     for _i in 0..n - 1 {
//         table.insert(Uuid::new_v4(), rng.gen());
//     }
//     table.insert(guaranteed_hit, rng.gen());
//     (table, guaranteed_hit)
// }
//
// fn uuid_oid_lookup(table: &FxHashMap<Uuid, u64>, k: &Uuid) -> Option<u64> {
//     table.get(k).copied()
// }

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
struct CompoundOrderId {
    session_id: Uuid,
    order_id: u64,
}

struct CompoundOrderIdMap {
    slow_map: FxHashMap<CompoundOrderId, u64>,
    fast_map: FxHashMap<u64, u64>,
}

fn build_compound_oid_table(n: usize, session_id: Uuid) -> CompoundOrderIdMap {
    let mut slow_map = FxHashMap::default();
    let mut fast_map = FxHashMap::default();
    let mut rng = rand::thread_rng();
    for i in 0..n {
        let order_id = i as u64;
        let compound_order_id = CompoundOrderId { session_id, order_id };
        slow_map.insert(compound_order_id, rng.gen());
        fast_map.insert(order_id, rng.gen());
    }
    CompoundOrderIdMap { slow_map, fast_map }
}

fn compound_oid_lookup(table: &CompoundOrderIdMap, k: &CompoundOrderId) -> Option<u64> {
    if k.session_id.is_nil() {
        table.fast_map.get(&k.order_id).copied()
    } else {
        table.slow_map.get(k).copied()
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let table = build_simple_oid_table(100000);
    c.bench_function("simple_oid_lookup hit", |b| {
        b.iter(|| simple_oid_lookup(&table, black_box(&200)))
    });
    c.bench_function("simple_oid_lookup miss", |b| {
        b.iter(|| simple_oid_lookup(&table, black_box(&999999)))
    });
    // let (table, hit) = build_uuid_oid_table(100000);
    // c.bench_function("uuid_oid_lookup hit", |b| {
    //     b.iter(|| uuid_oid_lookup(&table, black_box(&hit)))
    // });
    // let miss = Uuid::new_v4();
    // c.bench_function("uuid_oid_lookup miss", |b| {
    //     b.iter(|| uuid_oid_lookup(&table, black_box(&miss)))
    // });
    let session_id = Uuid::new_v4();
    let table = build_compound_oid_table(100000, session_id);
    c.bench_function("compound_oid_lookup fast hit", |b| {
        b.iter(|| {
            compound_oid_lookup(
                &table,
                &CompoundOrderId { session_id: Uuid::nil(), order_id: 200 },
            )
        })
    });
    c.bench_function("compound_oid_lookup fast miss", |b| {
        b.iter(|| {
            compound_oid_lookup(
                &table,
                &CompoundOrderId { session_id: Uuid::nil(), order_id: 999999 },
            )
        })
    });
    c.bench_function("compound_oid_lookup slow hit", |b| {
        b.iter(|| {
            compound_oid_lookup(&table, &CompoundOrderId { session_id, order_id: 200 })
        })
    });
    c.bench_function("compound_oid_lookup slow miss", |b| {
        b.iter(|| {
            compound_oid_lookup(&table, &CompoundOrderId { session_id, order_id: 999999 })
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
