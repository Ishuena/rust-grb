extern crate gurobi;

fn main() {
  let env = gurobi::Env::new("sos.log").unwrap();
  let mut model = env.new_model("sos").unwrap();

  let x0 = model.add_var("x0", gurobi::Continuous(0.0, 1.0), -2.0).unwrap();
  let x1 = model.add_var("x1", gurobi::Continuous(0.0, 1.0), -1.0).unwrap();
  let x2 = model.add_var("x2", gurobi::Continuous(0.0, 2.0), -1.0).unwrap();
  model.update().unwrap();

  // [x0 = 0] or [x1 = 0]
  model.add_sos(&[x0, x1], &[1.0, 2.0], gurobi::SOSType1).unwrap();

  // [x0 = 0] or [x2 = 0]
  model.add_sos(&[x0, x2], &[1.0, 2.0], gurobi::SOSType1).unwrap();

  model.optimize().unwrap();

  model.write("sos.lp").unwrap();
  model.write("sos.sol").unwrap();

  let obj = model.get(gurobi::attr::ObjVal).unwrap();
  assert!((obj + 3.0).abs() < 1e-12);

  let x0 = model.get_element(gurobi::attr::X, x0).unwrap();
  let x1 = model.get_element(gurobi::attr::X, x1).unwrap();
  let x2 = model.get_element(gurobi::attr::X, x2).unwrap();

  assert!((x0 - 0.0).abs() < 1e-12);
  assert!((x1 - 1.0).abs() < 1e-12);
  assert!((x2 - 2.0).abs() < 1e-12);
}