# SisitechDocs
# Research Topics

**Table of Contents:**
[Form Builder](#form-builder)
[User Automation Testing](#user-automation-testing)
[Design Systems / Philosophies](#design-systems-/-philosophies)
[State Management](#state-management)
[Software Documentation](#software-documentation)
[ Angular – Component Library]( #angular-component-library)



#### Form Builder  :innocent:

- Structuring & Packaging for Libraries - Modularized?
- HTTP Integration structure / Auth interceptors (on Angular)
- Research whether getx has auth interceptors
#### User Automation Testing

- Appium (Research)
   >Appium Server  
   > Appium Client
   
- WD (Promise)  
- Web Driver.io  
- Oxygen HQ
#### Design Systems / Philosophies

- Research on latest Bootstrap Documentation  
- Ant Design / Material Design Philosophies
 
#### State Management
------
- Redux  
- Store  
- Ngrx (Angular)  
- Getx (Flutter)

**Getx sample Code**
```dart 
import 'package:flutter/material.dart';
import 'package:get/get_navigation/src/root/get_material_app.dart';
import 'package:shopping_app/screens/product_overview_screen.dart';


void main() {
  runApp(MyApp());
}

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return  GetMaterialApp(
        debugShowCheckedModeBanner: false,
        title: 'Flutter Demo',
        theme: ThemeData(
          primarySwatch: Colors.purple,
          accentColor: Colors.deepOrange,
          fontFamily: "Lato",
        ),
        home: ProductOverviewPage(),

    );
  }
}
```
####  Software Documentation

- Documentation as Code Principle
    > Why it was built the way we built it (inline comments)  
     >How to use the library (readme)
      >Markdown scripting (HTML language - research)  
    Commit procedure guidelines  
    Breaking changes leads to upgrade of major versions
	
#### Angular – Component Library
   > Angular Ivy  
  > Angular Content Projection  
  > Lifecycle Hooks  
> Internationalization - Translation
 
