## Update all the Sisitech flutter libs packages to branch `update_flutter`

```yaml title="pubspec.yaml"
---

  flutter_utils:
    git:
      url: git@github.com:sisitech/flutter_utils.git
      ref: "b68a85f"

  flutter_auth:
    git:
      url: git@github.com:sisitech/flutter_auth.git
      ref: "67872f9"

  flutter_form:
    git:
      url: git@github.com:sisitech/flutter-forms.git
      ref: "68aa594"

  flutter_tables:
    git:
      url: git@github.com:sisitech/flutter-tables.git
      ref: "a24c8cb"

  flutter_login:
    git:
      url: git@github.com:sisitech/flutter_login.git
      ref: "2870a15"

  sistch_fl_onboarding:
    git:
      url: git@github.com:sisitech/flutter_onboarding.git
      ref: "adeb6b7"
---
```


##  Rename Android folder to android_old
```bash
mv android android_old
```
## Recreate Android Folder
In your root project run 
```bash
flutter create . 
```

!!! note
    If it's a `library` switch into the example folder before recreating.
    `cd example_folder`

## Copy the andorid `res` folder
```bash
rm -rf android/app/src/main/res
cp -r android/app/src/main/res android/app/src/main/
```
## Create a pro-guard rules file 
Create a `android/app/proguard-rules.pro` file
```txt title="android/app/proguard-rules.pro"
# Please add these rules to your existing keep rules in order to suppress warnings.
# This is generated automatically by the Android Gradle plugin.
-dontwarn com.google.errorprone.annotations.CanIgnoreReturnValue
-dontwarn com.google.errorprone.annotations.CheckReturnValue
-dontwarn com.google.errorprone.annotations.Immutable
-dontwarn com.google.errorprone.annotations.RestrictedApi
-dontwarn javax.annotation.Nullable
-dontwarn javax.annotation.concurrent.GuardedBy
```

## Update the main `android/build.gradle` file

Update the `allprojects` section to this

```title="android/build.gradle" hl_lines="8 9 10 11 12 13 14 15 16 17 18"
---

allprojects {
    repositories {
        google()
        mavenCentral()
    }
    subprojects {
        afterEvaluate { project ->
            if (project.hasProperty('android')) {
                project.android {
                    if (namespace == null) {
                        namespace project.group
                    }
                }
            }
        }
    }
}

---
```

## Move over any permission required to the new `AndroidManifest.xml` permissions and other settings

## Clean the project
```bash
flutter clean
```

## Update any packages with error

```bash title="terminal"
Execution failed for task ':workmanager:compileReleaseKotlin'.
> 'compileReleaseJavaWithJavac' task (current target is 1.8) and 'compileReleaseKotlin' task (current target is 17) jvm target compatibility should be set to the same Java version.
```

!!! note
    From the above error `workmanager` manager app seems to be the problem.
    Updating it to
    `workmanager: 0.5.2` fixes this issue

```yaml title="pubspec.yaml"

dependency_overrides:
  intl: ^0.19.0
  win32: ^5.5.4
  workmanager: 0.5.2
```

