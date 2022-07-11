mod base;
use {
    base::adj::AdjacencyList,
    std::time::{Duration, SystemTime, SystemTimeError},
};

/** Basic benchmark func */
fn benchmark<T>(f: impl Fn() -> T) -> Result<Duration, SystemTimeError> {
    let a = SystemTime::now();
    f();
    let b = SystemTime::now();
    b.duration_since(a)
}

fn main() {
    let mut graph = AdjacencyList::<&str, u8>::default();

    //  Create 3 vertexes
    let u = graph.add_vertex("a").unwrap();
    let v = graph.add_vertex("b").unwrap();
    let w = graph.add_vertex("c").unwrap();

    //  Connect the 3 with different edge values forming a triangle-like structure
    let uv = graph.add_edge(u.borrow_mut(), v.borrow_mut(), 1).unwrap();
    let vw = graph.add_edge(v.borrow_mut(), w.borrow_mut(), 2).unwrap();
    let wu = graph.add_edge(w.borrow_mut(), u.borrow_mut(), 3).unwrap();

    println!(
        "\nu: {:?}\nv: {:?}\nw: {:?}\n\nuv: {:?}\nvw: {:?}\nwu: {:?}\n\n{:?}\n{:?}\n{:?}",
        u,
        v,
        w,
        uv,
        vw,
        wu,
        benchmark(|| graph.adjacent(u.borrow(), v.borrow()).unwrap()),
        benchmark(|| graph.adjacent(v.borrow(), w.borrow()).unwrap()),
        benchmark(|| graph.adjacent(w.borrow(), u.borrow()).unwrap())
    )
}
