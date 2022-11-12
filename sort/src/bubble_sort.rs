pub fn sort(x: &mut [u64]) {
    let length = x.len();
    for i in 0..length {
        for j in (i + 1..length).rev() {
            if x[j] < x[j - 1] {
                let tmp = x[j];
                x[j] = x[j - 1];
                x[j - 1] = tmp;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::function_name::named;
    use rand::Rng;
    use std::fs::File;
    use std::io::Write;

    #[test]
    #[named]
    fn test_bubble_sort_simple() {
        let mut x_array: [u64; 6] = [8, 1, 2, 5, 0, 5];
        sort(&mut x_array);
        if cfg!(feature = "graph") {
            let graph_dir = "/tmp/graph";
            let _ = std::fs::create_dir(graph_dir);

            let dot_file = format!("{}/{}.dot", graph_dir, function_name!());
            let mut dot_file = File::create(dot_file).unwrap();
            write!(dot_file, "digraph Graph {{\n").unwrap();
            for i in 0..x_array.len() {
                write!(dot_file, "{}", x_array[i]).unwrap();
                if i < x_array.len() - 1 {
                    write!(dot_file, " -> ").unwrap();
                } else {
                    write!(dot_file, ";").unwrap();
                }
            }
            write!(dot_file, "\n}}\n").unwrap();
        }
    }

    #[test]
    fn test_bubble_sort_random() {
        let mut x_array: [u64; 12] = [0; 12];
        for i in 0..x_array.len() {
            x_array[i] = rand::thread_rng().gen_range(0..20);
        }
        sort(&mut x_array);
    }
}
