pub mod flamegraph;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn small_svg_build() {
        let reader = std::io::BufReader::new(std::fs::File::open("proof-data.txt").unwrap());
        let writer = std::io::BufWriter::new(
            std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .open("out.svg")
                .unwrap(),
        );
        flamegraph::handle_file(reader, writer).unwrap();
    }
}
