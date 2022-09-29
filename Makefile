default: debug
	
keys:
	if [[ ! -f "key.pem" ]]; then \
		openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem \
		-days 365 -sha256 -subj "/C=None/ST=None/L=None/O=None/OU=None/CN=localhost"; \
		openssl rsa -in key.pem -out key.pem; \
	fi

clean:
	rm -rf build

npm:
	npm i
	npm run build

dir:
	mkdir build

move_dbg: dir
	mv server_src/target/debug/server build/server

move_release: dir
	mv server_src/target/release/server build/server

dist: npm
	mv dist/ build/dist

copy_assets: dir
	cp -r assets/ build/assets
	cp index.html build/
	cp contributors.json build/

copy_keys: dir keys
	cp key.pem build/
	cp cert.pem build/

release: _release move_release


dev: _dev move_dbg


_release: clean dir dist copy_assets copy_keys
	cd server_src && \
	cargo build --release

_dev: clean dir dist copy_assets copy_keys
	cd server_src && \
	cargo build

debug: dev
