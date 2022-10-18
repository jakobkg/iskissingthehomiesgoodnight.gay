default: debug

clean:
	rm -rf build
	
keys:
	if [[ ! -f "key.pem" ]]; then \
		openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem \
		-days 365 -sha256 -subj "/C=NO/ST=None/L=None/O=None/OU=None/CN=localhost"; \
		openssl rsa -in key.pem -out key.pem; \
	fi

watch:
	@echo Watching /src for file changes
	@while sleep 1; do find ./src | entr -cdp make rebuild; done
	@echo Stopped watching

rebuild:
	@echo Change detected, rebuilding...
	@make cp-dist --no-print-directory

npm-fetch:
	@echo Fetching npm dependencies...
	@npm i -silent
	@echo Done!

npm-build:
	@echo Running npm build action...
	@npm run build
	@echo Done!

dir:
	@-mkdir -p build
	@-mkdir -p build/dist

move_dbg: dir
	@cp -u server_src/target/debug/server build/server

move_release: dir
	@cp -u server_src/target/release/server build/server

dist: dir npm-fetch npm-build
	@-cp -ru dist/* build/dist
	@-cp -ru src/* build/dist

cp-dist: npm-build
	@echo Copying built files
	@-cp -ru dist/* build/dist
	@-cp -ru src/* build/dist
	@echo Done :\)

copy_assets: dir
	@cp -ru assets/ build/assets
	@cp -u src/index.html build/
	@cp -u src/contributors.json build/

copy_keys: dir keys
	@cp -u key.pem build/
	@cp -u cert.pem build/

release: _release move_release


dev: _dev move_dbg


_release: clean dir dist copy_assets copy_keys
	cd server_src && \
	cargo build --release --features "production"

_dev: dir dist copy_assets
	cd server_src && \
	cargo build

debug: dev
