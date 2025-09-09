mod box_examples;
mod rc_examples;
mod refcell_examples;
mod memory_leak;

fn main() {
    box_examples::examples();
    rc_examples::examples();
    refcell_examples::examples();
    memory_leak::examples();
}
