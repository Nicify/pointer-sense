VERSION ?= 0.0.7

update-version:
	sed -i '' -e "s/\"version\": \".*\"/\"version\": \"$(VERSION)\"/" src-tauri/tauri.conf.json
	sed -i '' -e "s/\"version\": \".*\"/\"version\": \"$(VERSION)\"/" package.json
