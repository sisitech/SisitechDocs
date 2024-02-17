
# Setting Up Angular Libraries

### Generate a blank Angular workspace
```
ng new mylibspace --createApplication=false 

```

!!! note
    `--createApplication=false` does not create a default project


Switch to the newly created folder

``` bash
cd mylibspace
```

### Generate the library
```bash
ng generate library myform
```
This creates the library folder inside a projects folder


### Generate an example angular test app

```bash
ng generate application example
```
!!! note
        The name of the test application should always be `example` 

```json linenums="14" title="tsconfig.json"
 "paths": {
      "myform": [
        "dist/myform/myform",
        "dist/myform"
      ]
    }
```
Angular cli automatically adds the paths for the library build to be availbe in the workspace even if it's not in the `node_modules`. This eases development.


### Add build script

In the `package.json` add a script to build the library

```json title="package.json" linenums="11"
"scripts":{
    ...
    "myform:build":"ng build myform"
}
```

#### Build the library
```
npm run myform:build
```

### Testing the library in the example project

Import the required modules 

```ts linenums="1" hl_lines="6 16" title="projects/example/src/app/app.module.ts"

import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { MyformModule } from 'myform'

@NgModule({
  declarations: [
    AppComponent
  ],
  imports: [
    BrowserModule,
    AppRoutingModule,
    BrowserAnimationsModule,
    MyformModule
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule { }

```

Html

```html title="projects/example/src/app/app.component.html" linenums="1"
<div class="container">
  <h1>My Forms Example</h1>

  <app-myform [extraFields]="extra_fields" [formItems]="formItems" [url]="url"
    (onValidatedData)="onValidatedData($event)" [submitButtonText]="'Teacher'" [formGroupOrder]="formGroupOrder"
    [hideButtons]=false (onPostedData)="onPostedData($event)" [instance]="instance">
  </app-myform>

</div>
```

## Packaging the Library

### Add the packaging script
```json title="package.json" linenums="12"
"myform:pack": "cd dist/myform; npm pack"
```

### Package the library
```bash
npm run myform:pack
```
A `myform-0.0.1.tgz` is created inside the dist folder. This can be uploaded to any repository.

### Installing the packaged library

```bash
npm install myform-0.0.1.tgz
```

## Adding a scope to the name @myorg/library

Edit the myform lib `package.json` to reflect the same

```json title="projects/myform/package.json" linenums="1" hl_lines="2"
{
  "name": "@sisitech/myform",
  "version": "0.0.1",
  "peerDependencies": {
    "@angular/common": "^13.1.0",
    "@angular/core": "^13.1.0",
    "@angular/material": "13.1.0",
    "@angular/cdk": "13.1.0",
    "@angular/platform-browser": "13.1.0",
    "rxjs": "7.4.0"
  },
  "dependencies": {
    "tslib": "^2.3.0"
  }
}
```
Refer [here](- https://docs.npmjs.com/cli/v8/using-npm/scope) for more information on  npm scope naming.

Building and packaging the libary generates a `sisitech-myform-0.0.1.tgz` package, reflecting the added scope name.
```bash
npm run myform:build
npm run myform:pack

```
## Including Assets / Styles when Building the Library

Add `"assets": ["./styles/*.*","./assets/*.*"]` into the `ng-package.json` file of the library.

``` json title="ng-package.json" linenums="1" hl_lines="7 8 9 10"

{
  "$schema": "../../node_modules/ng-packagr/ng-package.schema.json",
  "dest": "../../dist/footer",
  "lib": {
    "entryFile": "src/public-api.ts"
  },
  "assets": [
    "./styles/*.*",
    "./assets/*.*"
  ]
}

```

## Using Assets, Javascript & styles Within Your Project

Add the code below to your `angular.json` file

``` json title="angular.json"
  "assets": [ 
    "projects/example/src/assets"
    ],
  "styles": [ 
    "projects/example/src/styles.scss"
    ],
  "scripts": [
    "projects/example/src/my-js-file.js"
    ]
```

## Resources

Example project with myform
[Github Repo](https://github.com/sisitech/angular-lib-demo)

