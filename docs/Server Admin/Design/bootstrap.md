# Angular Bootstrap

### Install bootsrap & popperjs

Refer to [bootstrap-npm](https://www.npmjs.com/package/bootstrap) for latest bootsrap version.

```
npm i bootstrap@5.1.3 @popperjs/core --save
```

### Install typescript definitions for bootstrap

```
npm i @types/bootstrap --dev
```

### Adding the file paths to the styles and scripts array in file **.angular-cli.json**

```json
 "styles": [
        "src/styles.scss"
 ],
 "scripts": [
   "node_modules/@popperjs/core/dist/umd/popper.min.js",
   "node_modules/bootstrap/dist/js/bootstrap.bundle.min.js"
 ]
```

### In style.css overide the desired variables before importing bootstrap

```scss
$primary:#017b94;
$secondary:#D4AB13;
$modal-fade-transform: scale(.8);

@import "~bootstrap/scss/bootstrap";
```
