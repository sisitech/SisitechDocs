# SisitechDocs
# Research Topics

**Table of Contents:**

1. [Form Builder](#form-builder)

1. [User Automation Testing](#user-automation-testing)

1. [Design Systems and Philosophies](#design-systems-philosophies)

1. [State Management](#state-management)

1. [Software Documentation](#software-documentation)

1. [ Angular Component Library](#angular-component-library)



### Form Builder

To access the Sisitech Forms, click [@sisitech/forms](./forms.md)

- Structuring & Packaging for Libraries - Modularized?
- HTTP Integration structure / Auth interceptors (on Angular)
- Research whether getx has auth interceptors
### User Automation Testing

- Appium (Research)
   >Appium Server  
   > Appium Client
   
- WD (Promise)  
- Web Driver.io  
- Oxygen HQ
### Design Systems and Philosophies

- Research on latest Bootstrap Documentation  
- Ant Design / Material Design Philosophies
 
### State Management
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
### Software Documentation

- Learn more about markdown scripting [here](./markdown.md)
 <br>
 
- Documentation as Code Principle
  > Why it was built the way we built it (inline comments)  
  > How to use the library (readme)
  > Markdown scripting (HTML language - research)  
  > Commit procedure guidelines  
  > Breaking changes leads to upgrade of major versions
	
#### Angular – Component Library
  > Angular Ivy  
  `npm i -g @angular/cli@latestng update`

  > Angular Content Projection  
  > Lifecycle Hooks  
  > Internationalization - Translation
 
#### Additional Information

To run the markdown script to a static website, use [MkDocs](https://www.mkdocs.org/) by running the code 

```md

mkdocs serve

```



Getting started with Material for MkDocs [here](https://squidfunk.github.io/mkdocs-material/getting-started/) 
