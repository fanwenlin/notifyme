run:
	cargo run run -- ping -c 10 163.com


run-lark:
	cargo run run -c lark -- ping -c 2 163.com

list:
	cargo run list

create:
	cargo run create test-config-set

delete:
	cargo run delete test-config-set

test:
	cargo run test default

edit:
	cargo run edit test-config-set
