# Setting Up Custom Flutter Theming Library

### Step 1: Defining a Theme Extension
Define a theme extension for your application. This should include all the colors, and typography customizable parameters for your application. This is also the reference to know what style properties are customizable. 

#### Approach
We leverage on the [ThemeExtension ](https://api.flutter.dev/flutter/material/ThemeExtension-class.html) class, an interface tht defines custom additions to a ThemeData object

To use, subclass **ThemeExtension**, define a number of fields (e.g. ```Colors and TextStyle```), and implement the **copyWith** and **lerp** methods. The latter will ensure smooth transitions of properties when switching themes.

##### Color class - dart:ui library - Dart API
API docs for the Color class from the dart:ui library, for the Dart programming language.

##### copyWith method - ThemeExtension class - material library - Dart API
API docs for the copyWith method from the ThemeExtension class, for the Dart programming language.

##### lerp method - ThemeExtension class - material library - Dart API
API docs for the lerp method from the ThemeExtension class, for the Dart programming language.

#### Extended Class Implemntation Example (SisitechThemeExtension)

``` dart
class SisitechThemeExtension extends ThemeExtension<SisitechThemeExtension> {
  final Color? backgroundColor;
  final TextStyle? titleStyle;
  final TextStyle? subTitleStyle;
  final TextStyle? descriptionStyle;

  const SisitechThemeExtension(
      {this.backgroundColor,
      this.titleStyle,
      this.subTitleStyle,
      this.descriptionStyle});

  @override
  SisitechThemeExtension copyWith(
      {Color? backgroundColor,
      TextStyle? titleStyle,
      TextStyle? subTitleStyle,
      TextStyle? descriptionStyle}) {
    return SisitechThemeExtension(
      backgroundColor: backgroundColor ?? this.backgroundColor,
      titleStyle: titleStyle ?? this.titleStyle,
      subTitleStyle: subTitleStyle ?? this.subTitleStyle,
      descriptionStyle: descriptionStyle ?? this.descriptionStyle,
    );
  }

  @override
  SisitechThemeExtension lerp(
      ThemeExtension<SisitechThemeExtension>? other, double t) {
    if (other is! SisitechThemeExtension) {
      return this;
    }
    return SisitechThemeExtension(
      backgroundColor: Color.lerp(backgroundColor, other.backgroundColor, t),
      titleStyle: TextStyle.lerp(titleStyle, other.titleStyle, t),
      subTitleStyle: TextStyle.lerp(subTitleStyle, other.subTitleStyle, t),
      descriptionStyle:
          TextStyle.lerp(descriptionStyle, other.descriptionStyle, t),
    );
  }
}
```

### Step 2: Using the ThemeExtension in your Library Components
In order to get an instance of your customTheme anywhere in your code, use:

``` dart
var sisitechTheme = Theme.of(context).extension<SisitechThemeExtension>();
```

### Step 3: Setting default values for your customTheme
``` dart
class OnboardingPageWidget extends StatelessWidget {
  final String title;
  final String description;
  final String imagePath;
  final Widget? child;

  const OnboardingPageWidget({
    Key? key,
    required this.title,
    required this.description,
    required this.imagePath,
    this.child,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    if (child != null) {
      return child!;
    }
    var onBoardingTheme =
        Theme.of(context).extension<SisitechOnboardingTheme>();

    // Applying  Default Styles
    var titleStyle =
        onBoardingTheme?.titleStyle ?? Theme.of(context).textTheme.titleMedium;

    return Text(
              title,
              style: titleStyle,
    )
  }
}
```


### Step 4: Example Implementation in a custom widget of the library

``` dart
library sistch_fl_lib;

import 'package:flutter/material.dart';
import 'package:sistch_fl_lib/theme_extension.dart';

class SistchCustomWidget extends StatelessWidget {
  const SistchCustomWidget({super.key});

  @override
  Widget build(BuildContext context) {
    var sisitechTheme = Theme.of(context).extension<SisitechThemeExtension>();

    return Container(
      padding: const EdgeInsets.all(10),
      decoration: BoxDecoration(
          color: sisitechTheme?.backgroundColor,
          borderRadius: const BorderRadius.all(Radius.circular(10))),
      child: Column(
        children: [
          Text(
            "Title",
            style: sisitechTheme?.titleStyle,
          ),
          Text(
            "Sub Title With Description",
            style: sisitechTheme?.subTitleStyle,
          ),
          Text(
            "Footer",
            style: sisitechTheme?.descriptionStyle,
          ),
        ],
      ),
    );
  }
}
```

### Step 5: Usage in your project
There are two methods: 

1. Define the custom extension as part of your themeData in your main.dart

``` dart
  darkTheme: ThemeData(
        brightness: Brightness.dark,
        extensions: const <ThemeExtension<dynamic>>[
          SisitechThemeExtension(
            backgroundColor: Colors.black12,
            titleStyle: TextStyle(fontSize: 16, fontWeight: FontWeight.bold),
          ),
        ],
        /* dark theme settings */
      ),
      theme: ThemeData(
        extensions: const <ThemeExtension<dynamic>>[
          SisitechThemeExtension(
            backgroundColor: Colors.blue,
            titleStyle: TextStyle(fontSize: 16, fontWeight: FontWeight.bold),
            ),
        ],
      )
```

2. Extend a theme anywhere inside any component using **copyWith()** which can be used to override the default theme in yout main app. 

``` dart
Theme(
  data: Theme.of(context).copyWith(extensions: [
    SisitechThemeExtension(
      backgroundColor: Colors.brown,
      titleStyle: TextStyle(
          fontSize: 16,
          fontWeight: FontWeight.bold,
          color: Theme.of(context).primaryColor),
    )
  ]),
  child: SistchCustomWidget(),
)
```

### Step 6: Complete Example
``` dart
import 'package:flutter/material.dart';
import 'package:sistch_fl_lib/sistch_fl_lib.dart';
import 'package:sistch_fl_lib/theme_extension.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      darkTheme: ThemeData(
        brightness: Brightness.dark,
        extensions: const <ThemeExtension<dynamic>>[
          SisitechThemeExtension(
            backgroundColor: Colors.black12,
            titleStyle: TextStyle(fontSize: 16, fontWeight: FontWeight.bold),
          ),
        ],
        /* dark theme settings */
      ),
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.red),
        useMaterial3: true,
        extensions: const <ThemeExtension<dynamic>>[
          // SisitechThemeExtension(
          //   backgroundColor: Colors.blue,
          //   titleStyle: TextStyle(fontSize: 16, fontWeight: FontWeight.bold),
          // ),
        ],
      ),
      // themeMode: ThemeMode.dark,
      themeMode: ThemeMode.light,
      home: Scaffold(
        body: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            const SistchCustomWidget(),
            const SizedBox(
              height: 10,
            ),
            Theme(
                data: Theme.of(context).copyWith(extensions: [
                  SisitechThemeExtension(
                    backgroundColor: Colors.brown,
                    titleStyle: TextStyle(
                        fontSize: 16,
                        fontWeight: FontWeight.bold,
                        color: Theme.of(context).primaryColor),
                  )
                ]),
                child: SistchCustomWidget())
          ],
        ),
      ),
    );
  }
}
```
