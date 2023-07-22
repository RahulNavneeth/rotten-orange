use std::{cmp, env, io::BufRead};

type Graph = Vec<Vec<usize>>;

fn is_valid(graph: &Graph, row: usize, col: usize) -> bool {
    let rows = graph.len();
    let cols = graph[0].len();
    rows >= 0 as usize && row < rows && col >= 0 as usize && col < cols
}

fn solve(graph: &mut Graph) -> isize {
    let mut queue: Vec<((usize, usize), usize)> = Vec::new();

    let mut o: usize = 0;
    let mut c: usize = 0;

    graph.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, &cell)| {
            if cell == 2 {
                queue.push(((i, j), 0));
            }
            if cell == 1 {
                o += 1;
            }
        })
    });

    let mut count = usize::MIN;
    while !queue.is_empty() {
        let ((i, j), value) = queue[0];
        queue.remove(0);
        let dir: Vec<(isize, isize)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

        dir.iter().for_each(|(x, y)| {
            let new_i = (i as isize + x) as usize;
            let new_j = (j as isize + y) as usize;
            if is_valid(graph, new_i, new_j) && graph[new_i][new_j] == 1 {
                c += 1;
                graph[new_i][new_j] = 2;
                let new_value = value + 1;
                count = cmp::max(count, new_value);
                queue.push(((new_i, new_j), new_value));
            }
        })
    }
    if c == o {
        count as isize
    } else {
        -1
    }
}

fn main() -> std::io::Result<()> {
    let mut graph: Graph = Vec::new();
    let args = env::args().collect::<Vec<String>>();
    let fs = std::fs::OpenOptions::new()
        .read(true)
        .open(args.get(1).map_or("./data.txt", |f| *&f))?;
    let f = std::io::BufReader::new(fs);
    f.lines().for_each(|i| {
        if let Ok(i) = i {
            let ch = i
                .split_whitespace()
                .filter_map(|f| match f.parse::<usize>() {
                    Ok(value) => Some(value),
                    Err(..) => None,
                })
                .collect::<Vec<usize>>();
            graph.push(ch);
        }
    });
    println!("{:?}", solve(&mut graph));
    Ok(())
}
