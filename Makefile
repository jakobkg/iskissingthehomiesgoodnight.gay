default: debug
	

clean:
	rm -rf build

npm:
	npm i
	npm run build

dir:
	mkdir build

move_dbg:
	mv server_src/target/debug/server build/server

move_release:
	mv server_src/target/release/server build/server

dist: npm
	mv dist/ build/dist

copy_assets: dir
	cp -r assets/ build/assets
	cp index.html build/
	cp contributors.json build/

release: _release move_release


dev: _dev move_dbg


_release: clean dir dist copy_assets
	cd server_src && \
	cargo build --release

_dev: clean dir dist copy_assets
	cd server_src && \
	cargo build

debug: dev
