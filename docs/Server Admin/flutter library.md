# Setting Up Flutter Libraries

### Create a blank flutter package template
Run the below code. Replace the package name **(sistch_fl_onboarding)** with your custom library's package name.

```
flutter create --template=package sistch_fl_onboarding
cd sistch_fl_onboarding
flutter create onboarding_example
```

### In your example pubspec.yaml
Add the below code in your dependencies and dependency overrides.

```
dependencies:
  flutter:
    sdk: flutter

  get: ^4.6.1
  sistch_fl_onboarding:

dependency_overrides:
  sistch_fl_onboarding:
    path: ../
```