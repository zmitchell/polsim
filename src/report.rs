use polarization::jones::{Beam, JonesVector};
use prettytable::{Table, Row, Cell};


pub fn basic_report(beam: Beam) {
    println!("intensity: {:.5e}", beam.intensity().unwrap());
    println!("x_mag: {:.5e}", beam.x().norm());
    println!("x_phase: {:.5e}", beam.x().arg());
    println!("y_mag: {:.5e}", beam.y().norm());
    println!("y_phase: {:.5e}", beam.y().arg());
}


pub fn table_report(mut beam: Beam) {
    beam.remove_common_phase_mut();
    let mut table = Table::new();
    table.add_row(row!["intensity", "x_mag", "x_phase", "y_mag", "y_phase"]);
    table.add_row(Row::new(vec![
        Cell::new(&format!("{:.5e}", beam.intensity().unwrap())),
        Cell::new(&format!("{:.5e}", beam.x().norm())),
        Cell::new(&format!("{:.5e}", beam.x().arg())),
        Cell::new(&format!("{:.5e}", beam.y().norm())),
        Cell::new(&format!("{:.5e}", beam.y().arg())),
    ]));
    table.printstd();
}