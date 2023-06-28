APP_NAME = UrbanAscent
RUST_CRATE_NAME = urban_ascent
BUILD_DIR = build
SOURCE_IMAGE = icon.png

.PHONY: all clean_mac gen_mac_icon

all: build_mac_app

build_mac_app: gen_mac_icon
	mkdir -p $(BUILD_DIR)/$(APP_NAME).app/Contents/MacOS
	mkdir -p $(BUILD_DIR)/$(APP_NAME).app/Contents/Resources
	cp Info.plist $(BUILD_DIR)/$(APP_NAME).app/Contents/Info.plist
	cp -a assets $(BUILD_DIR)/$(APP_NAME).app/Contents/MacOS/
	cargo build --release --target x86_64-apple-darwin
	cargo build --release --target aarch64-apple-darwin
	lipo target/x86_64-apple-darwin/release/$(RUST_CRATE_NAME) \
	     target/aarch64-apple-darwin/release/$(RUST_CRATE_NAME) \
	     -create -output $(BUILD_DIR)/$(APP_NAME).app/Contents/MacOS/$(APP_NAME)
	hdiutil create -fs HFS+ -volname $(APP_NAME) -srcfolder $(BUILD_DIR)/$(APP_NAME).app $(RUST_CRATE_NAME)_release_mac.dmg

gen_mac_icon:
	mkdir -p $(BUILD_DIR)/$(APP_NAME).app/Contents/Resources
	mkdir -p AppIcon.iconset
	sips -z 16 16     $(SOURCE_IMAGE) --out AppIcon.iconset/icon_16x16.png
	sips -z 32 32     $(SOURCE_IMAGE) --out AppIcon.iconset/icon_16x16@2x.png
	sips -z 32 32     $(SOURCE_IMAGE) --out AppIcon.iconset/icon_32x32.png
	sips -z 64 64     $(SOURCE_IMAGE) --out AppIcon.iconset/icon_32x32@2x.png
	sips -z 128 128   $(SOURCE_IMAGE) --out AppIcon.iconset/icon_128x128.png
	sips -z 256 256   $(SOURCE_IMAGE) --out AppIcon.iconset/icon_128x128@2x.png
	sips -z 256 256   $(SOURCE_IMAGE) --out AppIcon.iconset/icon_256x256.png
	sips -z 512 512   $(SOURCE_IMAGE) --out AppIcon.iconset/icon_256x256@2x.png
	sips -z 512 512   $(SOURCE_IMAGE) --out AppIcon.iconset/icon_512x512.png
	cp $(SOURCE_IMAGE) AppIcon.iconset/icon_512x512@2x.png
	iconutil -c icns AppIcon.iconset
	mv AppIcon.icns $(BUILD_DIR)/$(APP_NAME).app/Contents/Resources
	rm -R AppIcon.iconset

clean_mac:
	rm -rf $(BUILD_DIR)/$(APP_NAME).app
	cargo clean
