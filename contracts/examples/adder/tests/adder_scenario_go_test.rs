mod adder_mandos_constructed_raw_upgrade_test;

#[test]
fn adder_go() {
    multiversx_sc_scenario::run_go("scenarios/adder.scen.json");
}
