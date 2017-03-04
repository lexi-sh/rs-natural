extern crate natural;

use natural::tf_idf::TfIdf;

#[test]
fn test_basic_usage() {
	let mut tf_idf = TfIdf::new();

	tf_idf.add("this document is about rust.");
	tf_idf.add("this document is about erlang.");
	tf_idf.add("this document is about erlang and rust.");
	tf_idf.add("this document is about rust. it has rust examples");
	
	assert!(tf_idf.get("rust") > tf_idf.get("erlang"));
}