# Android và Google Play

Bản 0.3.0 dùng versionCode 3. APK debug dùng để cài thử, có application ID `dev.trong.gamehub.debug`. AAB release dùng `dev.trong.gamehub`, target SDK 36, min SDK 26. Chủ tài khoản cần xác nhận tên gói trước lần phát hành đầu tiên. Mỗi lần cập nhật phải tăng versionCode. [Workflow Web và Android](https://github.com/trongcheckmateking/game-hub-rust/actions/workflows/web-android.yml)

## Tự build

Cài Rust stable, JDK 17, Gradle 8.13, Android SDK Platform 36 và Build Tools 36.0.0. Chạy từ thư mục gốc.

```sh
rustup target add wasm32-unknown-unknown
cargo build --locked --release --target wasm32-unknown-unknown --bin game-hub
python3 scripts/package-web.py
gradle -p android :app:assembleDebug :app:bundleRelease
gradle -p android :app:connectedDebugAndroidTest
```

Lệnh test cần emulator hoặc điện thoại có WebGL và Android System WebView cập nhật. Luật chơi, menu, va chạm, âm thanh và tính điểm ở Rust. Android mở bundle nội bộ bằng WebViewAssetLoader. Bản này chứa WebAssembly và không đóng gói thư viện native Android .so. Nếu thêm SDK native, kiểm tra từng ABI và trang 16 KB. [Android Developers](https://developer.android.com/guide/practices/page-sizes)

## Kiểm soát kỹ thuật

- Android chặn network loads và cleartext, chỉ phục vụ danh sách tài nguyên đóng gói từ HTTPS origin nội bộ. Kiểm tra quyền trên manifest thực tế sau khi gộp dependency. App không cần Internet, camera, microphone, danh bạ hoặc vị trí.
- Tắt file/content access, quyền truy cập từ file URL, mixed content, cookie và nhiều cửa sổ. JavaScript phục vụ runtime WASM. Không có JavaScript interface vào native. WebView debugging tắt ở release.
- Tắt sao lưu hệ điều hành. Điểm và cài đặt lưu trên máy. Import kiểm tra kích cỡ, schema và giá trị trước khi thay dữ liệu. Chính sách riêng tư có trong app ngay từ màn hình đầu.
- CI tạo AAB chưa ký, APK debug dùng khóa thử của runner. Hai lần build CI có thể dùng khóa debug khác nhau. Gỡ bản debug cũ sẽ xóa dữ liệu của bản đó.

## Khi chủ tài khoản muốn gửi review

1. Xác nhận tên nhà phát triển, kênh liên hệ và nhóm tuổi thực tế. Đưa [trang chính sách](../web/privacy.html) lên một URL HTTPS công khai đọc được không cần đăng nhập. Đồng bộ tên và liên hệ giữa trang, app và Play Console. [Chuẩn bị duyệt](https://support.google.com/googleplay/android-developer/answer/9859455)
2. Dùng upload key của tài khoản để ký AAB. Nếu cập nhật app đã có, giữ đúng key và package ID. Ví dụ chạy `jarsigner -keystore /duong-dan/upload.jks app-release.aab ten-alias`, nhập mật khẩu ở máy cá nhân, rồi `jarsigner -verify -verbose -certs app-release.aab`. Dùng Play App Signing cho app mới. [Ký ứng dụng](https://developer.android.com/studio/publish/app-signing)
3. Tạo listing có ảnh chụp thật. Khai App access, Ads, Target audience, Content rating và Data safety theo bản gửi. Kiểm tra lại nếu thêm quảng cáo, thanh toán, telemetry hoặc SDK ngoài. [Chính sách dữ liệu](https://support.google.com/googleplay/android-developer/answer/10144311)
4. Đưa AAB đã ký lên internal/closed testing. Thử chế độ máy bay, chuyển ứng dụng, xoay màn hình, đa chạm, Bluetooth gamepad và lưu điểm sau cập nhật. Xem pre-launch report. Tài khoản cá nhân mới thuộc diện yêu cầu cần tối thiểu 12 người opt-in liên tục 14 ngày trước khi xin production access. [Yêu cầu thử nghiệm](https://support.google.com/googleplay/android-developer/answer/14151465)
5. Kiểm tra target API tại thời điểm nộp. Chính sách từ 31/08/2026 yêu cầu ứng dụng điện thoại mới target API 36 trở lên. Chủ tài khoản hoàn tất khai báo và gửi review. Google Play quyết định phê duyệt. [Yêu cầu API](https://support.google.com/googleplay/android-developer/answer/11926878)
