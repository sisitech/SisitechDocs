# Angular Inputs & Outputs

The ```@Input()``` and ```@Output()``` decorators are used in angular to enable sharing of data between a parent component and one or two child components.

```@Input()``` lets a parent component update data in the child component while ```@Output()``` lets the child send data to a parent component.

## Input

This decorator marks a class field as an input property and supplies configuration metadata and the Input property is bound to a DOM property in the template

### Sending data to a child component

When the ```@Input()``` decorator is used in a child component or directive this signifies that the property can receive its value from its parent component. i.e.
![flow of data](https://angular.io/generated/images/guide/inputs-outputs/input.svg)

### Configuring child component

To use ```@Input()``` you must configure the parent and child.
Using this decorater in the child you must first import ```Input``` and decorate it with the ```@Input()``` for example:

```code
import { Component, Input } from '@angular/core'; // First, import Input
export class ItemDetailComponent {
  @Input() item = ''; // decorate the property with @Input()
}
```

```@Input()``` properties can have any type, such as number, string, boolean, or object. The value for item comes from the parent component.
In our above example  ```@Input()``` decorates *Item* which is of type string
To the child component template add;

```code
<p>
  Today's item: {{item}}
</p>
```

### Configuring parent component

You should then bind the property in the parents component template. For eaxample using our child component;

- Use the child's selector, here `<app-item-detail>`, as a directive within the parent component template.
- Use property binding to bind the item property in the child to the currentItem property of the parent.

```code
<app-item-detail [item]="currentItem"></app-item-detail>
```

- In the parent component class, designate a value for currentItem:

```code
export class AppComponent {
  currentItem = 'Television';
}
```

With `@Input()`, Angular passes the value for currentItem to the child so that item renders as Television.

## Usage

You can supply an optional name to use in templates when the component is instantiated, that maps to the name of the bound property. By default, the original name of the bound property is used for input binding.

The following example creates a component with two input properties, one of which is given a special binding name.

```code
@Component({
  selector: 'bank-account',
  template: `
    Bank Name: {{bankName}}
    Account Id: {{id}}
  `
})
class BankAccount {
  // This property is bound using its original name.
  @Input() bankName: string;
  // this property value is bound to a different property name
  // when this component is instantiated in a template.
  @Input('account-id') id: string;

  // this property is not bound, and is not automatically updated by Angular
  normalizedBankName: string;
}

@Component({
  selector: 'app',
  template: `
    <bank-account bankName="RBC" account-id="4747"></bank-account>
  `
})
class App {}
```

## Output

Decorator that marks a class field as an output property and supplies configuration metadata. The DOM property bound to the output property is automatically updated during change detection.
It allows for data to flow from parent to child and can be represented as follows;

![Output](https://angular.io/generated/images/guide/inputs-outputs/output.svg)

`@Output()` marks a property in a child component as a doorway through which data can travel from the child to the parent.
The child component uses the `@Output()` property to raise an event to notify the parent of the change. To raise an event, an `@Output()` must have the type of `EventEmitter`, which is a class in @angular/core that you use to emit custom events.

To use `@Output()`, you must configure the parent and child.

### Configuring child component

1. Import Output and EventEmitter in the child component class:

```code
import { Output, EventEmitter } from '@angular/core';
```

2. In the component class, decorate a property with `@Output()`. The following example newItemEvent `@Output()` has a type of `EventEmitter`, which means it's an event.

```code
@Output() newItemEvent = new EventEmitter<string>();
```

For more information on EventEmitter, see the [EventEmitter API documentation](https://angular.io/api/core/EventEmitter).

3. Create an `addNewItem()` method in the same component class:

```code
export class ItemOutputComponent {

  @Output() newItemEvent = new EventEmitter<string>();

  addNewItem(value: string) {
    this.newItemEvent.emit(value);
  }
}
```

The `addNewItem()` function uses the `@Output()`, newItemEvent, to raise an event with the value the user types into the `<input>`.

4. Configure the child component template as follows;

```code
<label for="item-input">Add an item:</label>
<input type="text" id="item-input" #newItem>
<button type="button" (click)="addNewItem(newItem.value)">Add to parent's list</button>
```

### Configuring parent component

The `addItem()` method takes an argument in the form of a string and then adds that string to the items array.

```code
export class AppComponent {
  items = ['item1', 'item2', 'item3', 'item4'];

  addItem(newItem: string) {
    this.items.push(newItem);
  }
}
```

### Configuring the parent's template

In the parent's template, bind the parent's method to the child's event.

Put the child selector, here `<app-item-output>`, within the parent component's template.

```code
<app-item-output (newItemEvent)="addItem($event)"></app-item-output>
```

The event binding, `(newItemEvent)="addItem($event)"`, connects the event in the child, `newItemEvent`, to the method in the parent, `addItem()`.

The $event contains the data that the user types into the `<input>` in the child template UI.

To see the `@Output()` working, add the following to the parent's template:

```code
<ul>
  <li *ngFor="let item of items">{{item}}</li>
</ul>
```

The `*ngFor` iterates over the items in the items array. When you enter a value in the child's `<input>` and click the button, the child emits the event and the parent's `addItem()` method pushes the value to the items array and new item renders in the list.

### Usage

You can supply an optional name to use in templates when the component is instantiated, that maps to the name of the bound property. By default, the original name of the bound property is used for output binding.

## Using `@Input()` and `@Output()` together

Use `@Input()` and `@Output()` on the same child component as follows:

```code  
<app-input-output
  [item]="currentItem"
  (deleteRequest)="crossOffItem($event)">
</app-input-output>
```

The target, item, which is an `@Input()` property in the child component class, receives its value from the parent's property, `currentItem`. When you click delete, the child component raises an event, `deleteRequest`, which is the argument for the parent's `crossOffItem()` method.


## References

- [Sharing data between child and parent directives and components](https://angular.io/guide/inputs-outputs)
- [Input](https://angular.io/api/core/Input)
- [Output](https://angular.io/api/core/Output)
- [EventEmitter](https://angular.io/api/core/EventEmitter)
- [Angular @input, @output & EventEmitter](https://www.tektutorialshub.com/angular/angular-input-output-eventemitter/)