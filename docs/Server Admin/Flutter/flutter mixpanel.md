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