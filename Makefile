default: debug
	
keys:
	if [[ ! -f "key.pem" ]]; then \
		openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem \
		-days 365 -sha256 -subj "/C=NO/ST=None/L=None/O=None/OU=None/CN=localhost"; \
		openssl rsa -in key.pem -out key.pem; \
	fi

clean:
	rm -rf build

npm:
	npm i
	npm run build

dir:
	mkdir -p build

move_dbg: dir
	cp -u server_src/target/debug/server build/server

move_release: dir
	cp -u server_src/target/release/server build/server

dist: npm
	cp -ru dist/ build/dist

copy_assets: dir
	cp -ru assets/ build/assets
	cp -u index.html build/
	cp -u contributors.json build/

copy_keys: dir keys
	cp -u key.pem build/
	cp -u cert.pem build/

release: _release move_release


dev: _dev move_dbg


_release: clean dir dist copy_assets copy_keys
	cd server_src && \
	cargo build --release --features "production"

_dev: dir dist copy_assets
	cd server_src && \
	cargo build

debug: dev
