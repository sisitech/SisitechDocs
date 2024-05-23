# Setting Up Custom Icons and Splash Screen

### App Icon
#### Step 1: Configuring an App Icon
In order to easily add an app icon to your application, we will be leveraging on the [Flutter Launcher Icons Library](https://pub.dev/packages/flutter_launcher_icons). In order tp add your flutter launcher icons, add its configuration to your **pubspec.yaml** file. There are a couple of device options in the official library documentation. Here's an example of setting up. 

``` dart
dev_dependencies:
  flutter_launcher_icons: "^0.13.1"

flutter_launcher_icons:
  android: "launcher_icon"
  ios: true
  image_path: "assets/icon/icon.png"
  min_sdk_android: 21 # android min sdk min:16, default 21
  web:
    generate: true
    image_path: "path/to/image.png"
    background_color: "#hexcode"
    theme_color: "#hexcode"
  windows:
    generate: true
    image_path: "path/to/image.png"
    icon_size: 48 # min:48, max:256, default: 48
  macos:
    generate: true
    image_path: "path/to/image.png"
```
**NOTE: I use icon dimension image of 512x512 px to generate the icon.**

#### Step 2: Run the package

After setting up the configuration, all that is left to do is run the package
åß
``` dart
flutter pub get
flutter pub run flutter_launcher_icons
```

**NOTE: In order to creative adaptive icons for android pa**

### Splash Screen
#### Step 1: Configuring a splash screen
We will be using the [Flutter Native Splash Library](https://pub.dev/packages/flutter_native_splash). First, add flutter_native_splash as a dependency in your pubspec.yaml file.

``` dart
dependencies:
  flutter_native_splash: ^2.3.13
```

#### Step 2: Setting up the splash screen
There are multiple splash screen options as outlined in the official documentation, however for basic usage. We require an image with size 1152x1152px and the icon to cover a circle diameter of 768px within the image. Here's an example implementation in the **pubspec.yaml**

``` dart
flutter_native_splash:

  color: "#42a5f5"
  #background_image: "assets/background.png"
  #image: assets/splash.png
  #branding: assets/dart.png
  #branding_mode: bottom
  #color_dark: "#042a49"
  #background_image_dark: "assets/dark-background.png"
  #image_dark: assets/splash-invert.png
  #branding_dark: assets/dart_dark.png

  android_12:
    #image: assets/android12splash.png
    #color: "#42a5f5"
    #icon_background_color: "#111111"
    #branding: assets/dart.png
    #image_dark: assets/android12splash-invert.png
    #color_dark: "#042a49"
    #icon_background_color_dark: "#eeeeee"
  #android: false
  #ios: false
  #web: false
  #color_android: "#42a5f5"
  #color_dark_android: "#042a49"
  #color_ios: "#42a5f5"
  #color_dark_ios: "#042a49"
  #color_web: "#42a5f5"
  #color_dark_web: "#042a49"
  #image_android: assets/splash-android.png
  #image_dark_android: assets/splash-invert-android.png
  #image_ios: assets/splash-ios.png
  #image_dark_ios: assets/splash-invert-ios.png
  #image_web: assets/splash-web.gif
  #image_dark_web: assets/splash-invert-web.gif
  #background_image_android: "assets/background-android.png"
  #background_image_dark_android: "assets/dark-background-android.png"
  #background_image_ios: "assets/background-ios.png"
  #background_image_dark_ios: "assets/dark-background-ios.png"
  #background_image_web: "assets/background-web.png"
  #background_image_dark_web: "assets/dark-background-web.png"
  #branding_android: assets/brand-android.png
  #branding_dark_android: assets/dart_dark-android.png
  #branding_ios: assets/brand-ios.png
  #branding_dark_ios: assets/dart_dark-ios.png
  #branding_web: assets/brand-web.gif
  #branding_dark_web: assets/dart_dark-web.gif

  #android_gravity: center
  #ios_content_mode: center
  #web_image_mode: center
  #android_screen_orientation: sensorLandscape

  #To show the notification bar, add the following code to your Flutter app:
  #WidgetsFlutterBinding.ensureInitialized();
  # SystemChrome.setEnabledSystemUIMode(SystemUiMode.manual, overlays: [SystemUiOverlay.bottom, SystemUiOverlay.top], );
  #fullscreen: true

  # If you have changed the name(s) of your info.plist file(s), you can specify the filename(s)
  # with the info_plist_files parameter.  Remove only the # characters in the three lines below,
  # do not remove any spaces:
  #info_plist_files:
  #  - 'ios/Runner/Info-Debug.plist'
  #  - 'ios/Runner/Info-Release.plist'
```
#### Step 3: Run the package

After setting up the configuration, all that is left to do is run the package

``` dart
flutter pub get
dart run flutter_native_splash:create
```