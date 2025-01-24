install:
	# 配置文件
	# curl https://raw.githubusercontent.com/LuVx21/coding-config/refs/heads/master/rust/config.toml > /usr/local/cargo/config.toml
	# 格式化工具
	# rustup component add rustfmt
	# rocksdb 使用
	# apt install -y llvm clang libclang-dev
	# 共享缓存
	# cargo install sccache
	# export RUSTC_WRAPPER=sccache

sqlx-prepare:
	# DATABASE_URL=mysql://root:xxx@luvx.rx:3306/boot
	cargo sqlx prepare --workspace -- --all-targets --all-features --tests
bench:
	cargo bench
test:
	cargo t
test-run: test
	# cargo r -p coding-usage
	cargo r --bin coding-usage
run-cmd:
	# cargo r -p cmd
	cargo r --bin cmd
	cargo r --bin cmd -- add "foo bar"
	cargo r --bin cmd -- done 1001
