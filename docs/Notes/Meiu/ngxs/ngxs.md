# QUICK GET STARTED WITH NGXS

## What is NGXS

`NGXS` is a state management pattern + library for Angular. It acts as a single source of truth for your application’s state, providing simple rules for predictable state mutations.

`NGXS` is modeled after the `CQRS` pattern popularly implemented in libraries like `Redux` and `NGRX` but reduces boilerplate by using modern TypeScript features such as classes and decorators.

## Adding to an Angular Project

### Installing Requirements

Core

```
npm install @ngxs/store --save
```

Debugging Plugin

```
@ngxs/logger-plugin @ngxs/devtools-plugin --dev
```

Storage Plugin

```
npm install @ngxs/storage-plugin --save
```

- [Chrome Redux Debugger](https://chrome.google.com/webstore/detail/redux-devtools/lmhkpmbekcpmknklioeibfkpmmfibljd)
- [Firefox Redux Debugger](https://addons.mozilla.org/en-US/firefox/addon/reduxdevtools/)

![Image](https://490253082-files.gitbook.io/~/files/v0/b/gitbook-x-prod.appspot.com/o/spaces%2F-L9CoGJCq3UCfKJ7RCUg-347405460%2Fuploads%2Fgit-blob-14fd00e64a2f1da8023ada2c9244a2d49a99818e%2Fdevtools.png?alt=media)

### Adding to an root app module

``` title="app.modules.ts" hl_lines="9" linenums="1"
import { NgxsModule } from '@ngxs/store';
import { NgxsReduxDevtoolsPluginModule } from '@ngxs/devtools-plugin';

@NgModule({
  imports: [  
   // Main Module
   NgxsModule.forRoot([]),
   // Plugins
   NgxsStoragePluginModule.forRoot()
   NgxsReduxDevtoolsPluginModule.forRoot()
   ]
})
export class AppModule {}
```

!!! note
    It is recommended to register the storage plugin before other plugins so initial state can be picked up by those plugins.
