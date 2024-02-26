# Setting up Mixpanel for Flutter

### Getting Started
In order  to use Mixpanel on your flutter project, it's pretty much straightforwrd. Install the current version of Mixpanel by adding it as a dependency in your **pubspec.yaml**

``` dart
dependencies:
  mixpanel_flutter: ^2.2.0
```

### Usage
To use Mixpanel, we are leveraging on GetX for state management. Our custom Sisitech Mixpanel library exposes a **MixPanelController** that takes a mixpanel Token and **MixpanelOptions.** The current options include : 

``` dart
  const MixpanelOptions({
    this.enableAnonymous = true,
    this.enabled = true,
    this.persistentAnonymous = true,
    this.disableInDebug = true,
  });
```

In your **main()** function, add mixpanel and pass the token as below. 

``` dart
void main() async {
  Get.put(
    MixPanelController(mixpanelToken: "your_mixpanel_token"),
  );
}
```

**NOTE: All tracking happens in the specific Sisitech library. In order to expand a feature you will need to do it in the library.**

### Under The Hood
The mixpanel library is found in [Sisitech's Flutter Utils](https://github.com/sisitech/flutter_utils). The library folder contains two files: **mixpanel.dart** and **mixpanel_controller.dart**. 

The **mixpanel** file contains an initMixpanel() function that sets up your product for initialization. 

``` dart
import 'package:mixpanel_flutter/mixpanel_flutter.dart';

Future<Mixpanel> initMixpanel(token, {trackAutomaticEvents=true}) async {
    // dprint('mixpanel init');
    var mixpanel = await Mixpanel.init(token, trackAutomaticEvents: trackAutomaticEvents);
    return mixpanel;
  }
```

The **mixpanel_controller** file contains different functions used to track Mixpanel events.

``` dart
import 'package:flutter_auth/flutter_auth_controller.dart';
import 'package:flutter_utils/flutter_utils.dart';
import 'package:flutter_utils/mixpanel/mixpanel.dart';
import 'package:get/get.dart';
import 'package:mixpanel_flutter/mixpanel_flutter.dart';

class MixpanelOptions {
  final bool enableAnonymous;
  final bool enabled;
  final bool persistentAnonymous;
  final bool disableInDebug;
  const MixpanelOptions({
    this.enableAnonymous = true,
    this.enabled = true,
    this.persistentAnonymous = true,
    this.disableInDebug = true,
  });
}

class MixPanelController extends GetxController {
  late String mixpanelToken;
  Mixpanel? _mixpanel;
  late MixpanelOptions options;
  AuthController authController = Get.find<AuthController>();

  MixPanelController(
      {required this.mixpanelToken, this.options = const MixpanelOptions()});

  @override
  void onInit() {
    super.onInit();
    initializeMixPanel(options);
  }

  get mixpanel {
    return _mixpanel;
  }

  saveAnonymousId() {}

  getSavedAnonymousId() {}

  getAnonymouseuser() {
    // Save a unique id to local storage and use it everytime
    // The key should be passed when initializing.
    if (options.enableAnonymous && options.persistentAnonymous) {
      // Save or get
    }
    return "Anonymous";
  }

  get isDisAbled {
    if (options.disableInDebug) {
      return false;
    }
    // Check if anonymous mode enalbed
    if (!authController.isAuthenticated$.value && !options.enableAnonymous) {
      return false;
    }
    return options.enabled;
  }

  getUser() {
    var anymousProfile = {"username": getAnonymouseuser()};
    Map<String, dynamic> profile;
    if (authController.isAuthenticated$.value) {
      profile = authController.profile.value ?? anymousProfile;
      dprint(
          "Mixpanel User ${authController.profile.value?["username"]} initialized.");
    } else {
      profile = anymousProfile;
    }
    return profile;
  }

  initializeMixPanel(MixpanelOptions options) async {
    if (isDisAbled) {
      dprint(
          "Mixpanel disabled,disableInDebug:${options.disableInDebug} enabled:${options.enabled}");
      return;
    }
    _mixpanel = await initMixpanel(mixpanelToken);
    var profile = getUser();
    _mixpanel?.identify(profile["username"]);
  }

  track(String eventName, {Map<String, dynamic>? properties}) {
    _mixpanel?.track(eventName, properties: properties);
  }

  timeEvent(String eventName) {
    _mixpanel?.timeEvent(eventName);
  }
}

```

Most importantly, **initializeMixPanel** and **track** functions are most resourcesful. You can add new functions in the controller i.e. **getAnonymouseuser()**