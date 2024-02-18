#![forbid(unsafe_code)]

fn main() {
    let result = prs_1brc_cg::run();

    if let Err(e) = result {
        eprintln!("Error {e:?}");
    }
}
