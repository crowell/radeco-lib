extern crate radeco_lib;

use radeco_lib::utils::{Pipeline, Runner, Pipeout, Analysis};
use radeco_lib::frontend::r2::R2;

#[test]
fn test1() {
	let pipeline = vec![Pipeline::ReadFromR2, Pipeline::ParseEsil,
	                    Pipeline::CFG, Pipeline::SSA];
	
	let test_name = "test1".to_owned();
	let bin_name = Some("./ex-bins/simple2".to_owned());
	let addr = Some("sym.main".to_owned());
	let mut test = Runner::new(test_name, bin_name, addr, false, pipeline, None);
	test.run();
	test.dump();
}

#[test]
fn test_analysis1() {
	let esil = vec!["4,5,+".to_owned(), "6,*".to_owned(),
	                "100,>,zf,=".to_owned(),
					"5,rax,=".to_owned(),
					"6,rbx,=".to_owned(),
					"7,rbx,=".to_owned()
	               ];

	let test_name = "test2".to_owned();
	// Get a new r2 instance.
    let mut r2 = R2::new("./ex-bins/simple2");
    // Initialize with sane defaults.
    r2.init();
    let r = r2.get_reg_info().unwrap();

	let pipeline = vec![
		Pipeline::ParseEsil,
		Pipeline::CFG,
		Pipeline::SSA,
		Pipeline::Verify,
		Pipeline::AnalyzeSSA(Analysis::ConstProp),
		Pipeline::DCE,
		Pipeline::Verify
	];

	let mut test = Runner::new(test_name, None, None, true, pipeline, None);
	test.state.pipeout = Some(Pipeout::Esil(esil));
	test.state.reg_info = Some(r.clone());
	test.run();
	test.dump();
}

#[test]
fn test_analysis2() {
	let test_name = "test_analysis".to_string();
	let bin_name = Some("./ex-bins/constprop.o".to_string());
	let addr = Some("section..text".to_string());
	let pipeline = vec![
		Pipeline::ReadFromR2,
		Pipeline::ParseEsil,
		Pipeline::CFG,
		Pipeline::SSA,
		Pipeline::Verify
		//Pipeline::DCE,
		//Pipeline::AnalyzeSSA(Analysis::ConstProp),
		//Pipeline::DCE
	];
	let mut test = Runner::new(test_name, bin_name, addr, true, pipeline, None);
	test.run();
	test.dump();
}

//#[test]
//fn tachikoma() {
	//let test_name = "tachikoma".to_string();
	//let bin_name = Some("./ex-bins/tachikoma".to_string());
	//let addr = Some("fcn.0002b401".to_string());
	//let pipeline = vec![
		//Pipeline::ReadFromR2,
		//Pipeline::ParseEsil,
		//Pipeline::CFG,
		//Pipeline::SSA,
		//Pipeline::Verify
		////Pipeline::DCE,
		////Pipeline::Verify
		////Pipeline::AnalyzeSSA(Analysis::ConstProp),
		////Pipeline::DCE
	//];
	//let mut test = Runner::new(test_name, bin_name, addr, true, pipeline, None);
	//test.run();
	//test.dump();
//}
